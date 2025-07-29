use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body,
};

use crate::utils::file::read_drop;

pub async fn preview_handler(Path((alias, filename)): Path<(String, String)>) -> Response {
    match read_drop(&alias, &filename) {
        Ok(contents) => {
            Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from(contents))
            .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "No file found").into_response(),
    }
}