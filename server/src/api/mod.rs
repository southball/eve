mod files;
mod todo_files;
mod todos;
mod users;

use axum::{
    routing::{get, post},
    Router,
};
use serde::Serialize;

use files::*;
use todo_files::*;
use todos::*;
use users::*;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/user", get(get_user))
        .route("/users", post(post_users))
        .route("/login", post(post_login))
        .route("/todos", get(get_todos).post(post_todos))
        .route("/todo/:todo_id", post(post_todo))
        .route("/todo/:todo_id/files", post(post_todo_files))
        .route("/files", get(get_files).post(post_files))
        .route("/file/:file_id", get(get_file).post(post_file))
}

#[derive(Serialize)]
struct ErrorResponse {
    error_message: String,
}

impl From<&str> for ErrorResponse {
    fn from(error_message: &str) -> Self {
        ErrorResponse {
            error_message: error_message.to_owned(),
        }
    }
}
