mod auth;

use auth::AccountId;
use axum::{extract::Extension, response::IntoResponse, routing::get, Json, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let api_router = Router::new()
        .route("/todos", get(get_todos))
        .layer(Extension(pg_pool));

    let app = Router::new().nest("/api", api_router);

    let port = std::env::var("EVE_SERVER_PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()?;

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_todos(
    AccountId(_account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let todos = sqlx::query!(
        "
        SELECT * FROM todo
        "
    )
    .fetch_all(&pg_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.title)
    .collect::<Vec<_>>();

    Json(todos)
}
