use axum::{
    routing::{get, post},
    Router,
};

use tower_http::cors::{CorsLayer, Any};

mod handlers;
mod utils;

use handlers::{
    upload::upload_handler,
    read::preview_handler,
    export::export_handler,
    list::list_handler,
    delete::delete_handler,
    create::create_handler,
    list_root::root_list_handler
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // CORS Layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Du kan byta till .allow_origin("http://localhost:3000".parse().unwrap()) för mer kontroll
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/upload/:alias/*path", post(upload_handler))
        .route("/get/:alias/*path", get(preview_handler))
        .route("/export/:alias/*path", get(export_handler))
        .route("/list/:alias", get(root_list_handler))
        .route("/list/:alias/*path", get(list_handler))
        .route("/delete/:alias/*path", post(delete_handler))
        .route("/create/:alias/*path", post(create_handler))
        .layer(cors); // <-- applicera CORS-lagret här

    let listener = tokio::net::TcpListener::bind("192.168.1.129:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
