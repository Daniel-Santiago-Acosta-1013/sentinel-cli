mod app;
mod cli;
mod control;
mod core;
mod engine;
mod platform;
mod storage;

#[tokio::main]
async fn main() -> miette::Result<()> {
    app::run().await
}
