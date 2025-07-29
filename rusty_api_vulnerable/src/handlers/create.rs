use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body,
};

use crate::utils::file::create_directory;

pub async fn create_handler(Path((alias, path)): Path<(String, String)>) -> Response {
    match create_directory(&alias, &path) {
        Ok(content) => {
            Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from(content))
            .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "No files found").into_response(),
    }
}