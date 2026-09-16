use axum::Json;

use crate::models::{ApiResponse, JsonInput};

pub async fn wordcount(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = &body.json;
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let lines = text.lines().count();

    Json(ApiResponse {
        ok: true,
        result: Some(format!(
            "words: {}, characters: {}, lines: {}",
            words, chars, lines
        )),
        error: None,
    })
}

pub async fn slugify(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let slug = body.json
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    Json(ApiResponse {
        ok: true,
        result: Some(slug),
        error: None,
    })
}

pub async fn to_uppercase(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    Json(ApiResponse {
        ok: true,
        result: Some(body.json.to_uppercase()),
        error: None,
    })
}

pub async fn to_lowercase(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    Json(ApiResponse {
        ok: true,
        result: Some(body.json.to_lowercase()),
        error: None,
    })
}

pub async fn reverse(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    Json(ApiResponse {
        ok: true,
        result: Some(body.json.chars().rev().collect::<String>()),
        error: None,
    })
}