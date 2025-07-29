use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body,
};


use crate::utils::file::root_list_drop;

pub async fn root_list_handler(Path(alias): Path<String> ) -> Response {
    match root_list_drop(&alias) {
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

