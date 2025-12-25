use axum::{Extension, Router, routing::get};
use sea_orm::Database;

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db = Database::connect(std::env::var("DATABASE_URL")?).await?;

    let app = Router::new()
        .nest("/api", api::app())
        .route("/", get(hello))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::debug!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}
