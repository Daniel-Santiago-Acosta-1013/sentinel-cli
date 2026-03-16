mod app;
mod blocking;
mod cli;
mod control;
mod install;
mod platform;
mod storage;

#[tokio::main]
async fn main() -> miette::Result<()> {
    app::run().await
}
