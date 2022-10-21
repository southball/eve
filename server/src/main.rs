mod api;
mod auth;
mod session;

use axum::{extract::Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use api::get_api_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let app = Router::new()
        .nest("/api", get_api_router())
        .layer(Extension(pg_pool))
        .layer(CookieManagerLayer::new());

    let port = std::env::var("EVE_SERVER_PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()?;

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
