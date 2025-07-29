use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body,
};
use std::path::Path as FilePath;

use crate::utils::file::export_drop;


pub async fn export_handler(Path((alias, path)): Path<(String, String)>) -> Response {
    match export_drop(&alias, &path) {
        Ok(file_bytes) => {
            let filename = FilePath::new(&path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
                .body(Body::from(file_bytes))
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
} 