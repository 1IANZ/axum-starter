mod config;
mod core;
mod entity;
mod infra;
mod modules;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::run(modules::create_router()).await
}
