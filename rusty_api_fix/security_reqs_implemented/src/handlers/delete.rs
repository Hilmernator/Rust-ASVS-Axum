use axum::{
    body::Body, extract::Path, http::{header::{self}, StatusCode}, response::{IntoResponse, Response}
};

use crate::utils::file::delete_directory;

pub async fn delete_handler(Path((alias, path)): Path<(String, String)>) -> Response {
    match delete_directory(&alias, &path) {
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