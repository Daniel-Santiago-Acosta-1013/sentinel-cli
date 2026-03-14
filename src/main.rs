mod app;
mod blocking;
mod control;
mod install;
mod platform;
mod storage;
mod tui;

#[tokio::main]
async fn main() -> miette::Result<()> {
    app::run().await
}
