mod api;
mod app;
mod entity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
