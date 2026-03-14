use std::{net::SocketAddr, time::Duration};

use hickory_proto::{
    op::{Message, MessageType, ResponseCode},
    rr::RecordType,
    serialize::binary::{BinEncodable, BinEncoder},
};
use miette::{IntoDiagnostic, Result};
use tokio::{net::UdpSocket, time::timeout};

use crate::{
    core::rules::{built_in_rules, should_block},
    storage::config::AppConfig,
};

pub async fn handle_query(
    data: &[u8],
    upstream: SocketAddr,
    config: &AppConfig,
) -> Result<Vec<u8>> {
    let message = Message::from_vec(data).into_diagnostic()?;
    let query = if let Some(query) = message.queries().first() {
        query
    } else {
        return Ok(data.to_vec());
    };

    let name = query.name().to_utf8().trim_end_matches('.').to_lowercase();
    let allow_rules = config.user_rules();
    if query.query_type() == RecordType::A
        && should_block(&name, &built_in_rules(), &allow_rules)
    {
        return blocked_response(&message);
    }

    forward_query(data, upstream).await
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

async fn forward_query(data: &[u8], upstream: SocketAddr) -> Result<Vec<u8>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.into_diagnostic()?;
    socket.send_to(data, upstream).await.into_diagnostic()?;
    let mut buffer = [0u8; 4096];
    let (size, _) = timeout(Duration::from_secs(3), socket.recv_from(&mut buffer))
        .await
        .into_diagnostic()?
        .into_diagnostic()?;
    Ok(buffer[..size].to_vec())
}

pub fn default_upstream() -> SocketAddr {
    "1.1.1.1:53".parse().expect("static upstream address")
}
