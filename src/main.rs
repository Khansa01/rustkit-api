mod handlers;
mod models;

use axum::{
    Router,
    routing::post,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/format",           post(handlers::format_json))
        .route("/minify",           post(handlers::minify_json))
        .route("/validate",         post(handlers::validate_json))
        .route("/escape",           post(handlers::escape_json))
        .route("/unescape",         post(handlers::unescape_json))
        .route("/flatten",          post(handlers::flatten_json))
        .route("/diff",             post(handlers::diff_json))
        .route("/text/wordcount",   post(handlers::wordcount))
        .route("/text/slugify",     post(handlers::slugify))
        .route("/text/uppercase",   post(handlers::to_uppercase))
        .route("/text/lowercase",   post(handlers::to_lowercase))
        .route("/text/reverse",     post(handlers::reverse))
        .route("/qr/generate",      post(handlers::generate_qr))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 rustkit-api running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}