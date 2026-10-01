mod adapters;
mod app;
mod application;
mod bootstrap;
mod domain;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = bootstrap::build().await?;
    app.run().await
}
