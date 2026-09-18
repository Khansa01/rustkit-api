mod handlers;
mod models;

use axum::{
    Router,
    routing::{post, get},
};

use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/format",               post(handlers::format_json))
        .route("/minify",               post(handlers::minify_json))
        .route("/validate",             post(handlers::validate_json))
        .route("/escape",               post(handlers::escape_json))
        .route("/unescape",             post(handlers::unescape_json))
        .route("/flatten",              post(handlers::flatten_json))
        .route("/diff",                 post(handlers::diff_json))
        .route("/text/wordcount",       post(handlers::wordcount))
        .route("/text/slugify",         post(handlers::slugify))
        .route("/text/uppercase",       post(handlers::to_uppercase))
        .route("/text/lowercase",       post(handlers::to_lowercase))
        .route("/text/reverse",         post(handlers::reverse))
        .route("/qr/generate",          post(handlers::generate_qr))
        .route("/utils/base64/encode",  post(handlers::base64_encode))
        .route("/utils/base64/decode",  post(handlers::base64_decode))
        .route("/utils/url/encode",     post(handlers::url_encode))
        .route("/utils/url/decode",     post(handlers::url_decode))
        .route("/utils/hash/md5",       post(handlers::hash_md5))
        .route("/utils/hash/sha256",    post(handlers::hash_sha256))
        .route("/utils/jwt/decode",     post(handlers::jwt_decode))
        .route("/utils/ip",             post(handlers::ip_info))
        .route("/utils/jwt/encode",     post(handlers::jwt_encode))
        .route("/utils/myip",           get(handlers::my_ip))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 rustkit-api running on http://localhost:3000");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}