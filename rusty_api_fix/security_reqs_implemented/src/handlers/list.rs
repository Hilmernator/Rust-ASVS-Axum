use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body,
};

use crate::utils::file::list_drop;

pub async fn list_handler(Path((alias, path)): Path<(String, String)> ) -> Response {
    let effective_path = if path.trim().is_empty() {"."} else {&path};
    match list_drop(&alias, &effective_path) {
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