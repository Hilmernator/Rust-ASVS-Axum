    use axum::{extract::{Path, Multipart}, response::{IntoResponse, Response}};
    use crate::utils::file::upload_drop;

    pub async fn upload_handler(Path((alias, path)): Path<(String,String)>, mut multipart: Multipart) -> Response {
        while let Some(field) = multipart.next_field().await.unwrap() {
            let data = match field.bytes().await {
                Ok(bytes) => bytes,
                Err(_bytes) => return "failed to read file".into_response(),
            };
            match upload_drop(&alias, &path, &data) {
                Ok(msg) => return msg.into_response(),
                Err(_msg) => return "failed to save file".into_response(),
            };
        }
        "No file found in upload".into_response()
    }