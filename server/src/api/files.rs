use axum::http::StatusCode;

use super::{APIResponse, ErrorResponse};

pub async fn get_todo_files() -> APIResponse<()> {
    Err(ErrorResponse::from(
        StatusCode::NOT_FOUND,
        "Not yet implemented.",
    ))
}

pub async fn post_todo_files() -> APIResponse<()> {
    Err(ErrorResponse::from(
        StatusCode::NOT_FOUND,
        "Not yet implemented.",
    ))
}

pub async fn get_todo_file() -> APIResponse<()> {
    Err(ErrorResponse::from(
        StatusCode::NOT_FOUND,
        "Not yet implemented.",
    ))
}

pub async fn post_todo_file() -> APIResponse<()> {
    Err(ErrorResponse::from(
        StatusCode::NOT_FOUND,
        "Not yet implemented.",
    ))
}

pub async fn get_files() -> APIResponse<()> {
    Err(ErrorResponse::from(
        StatusCode::NOT_FOUND,
        "Not yet implemented.",
    ))
}
