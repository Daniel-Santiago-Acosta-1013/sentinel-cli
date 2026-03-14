use std::{net::SocketAddr, time::Duration};

use hickory_proto::{
    op::{Message, MessageType, ResponseCode},
    serialize::binary::{BinEncodable, BinEncoder},
};
use miette::{IntoDiagnostic, Result};
use tokio::{net::UdpSocket, time::timeout};

use crate::blocking::blocklist::BlocklistBundle;

pub fn should_block(domain: &str, blocklist: &BlocklistBundle) -> bool {
    blocklist.matches(domain)
}

pub async fn handle_query(
    payload: &[u8],
    upstream: SocketAddr,
    blocklist: &BlocklistBundle,
) -> Result<Vec<u8>> {
    let message = Message::from_vec(payload).into_diagnostic()?;
    let query = if let Some(query) = message.queries().first() {
        query
    } else {
        return Ok(payload.to_vec());
    };

    let domain = query.name().to_utf8().trim_end_matches('.').to_lowercase();
    if should_block(&domain, blocklist) {
        return blocked_response(&message);
    }

    forward_query(payload, upstream).await
}

fn blocked_response(message: &Message) -> Result<Vec<u8>> {
    let mut response = Message::new();
    response.set_id(message.id());
    response.set_message_type(MessageType::Response);
    response.set_op_code(message.op_code());
    response.set_response_code(ResponseCode::NXDomain);
    response.set_recursion_desired(message.recursion_desired());
    response.set_recursion_available(true);
    for query in message.queries() {
        response.add_query(query.clone());
    }

    let mut bytes = Vec::new();
    let mut encoder = BinEncoder::new(&mut bytes);
    response.emit(&mut encoder).into_diagnostic()?;
    Ok(bytes)
}

async fn forward_query(payload: &[u8], upstream: SocketAddr) -> Result<Vec<u8>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.into_diagnostic()?;
    socket.send_to(payload, upstream).await.into_diagnostic()?;
    let mut buffer = [0u8; 4096];
    let (size, _) = timeout(Duration::from_secs(3), socket.recv_from(&mut buffer))
        .await
        .into_diagnostic()?
        .into_diagnostic()?;
    Ok(buffer[..size].to_vec())
}
