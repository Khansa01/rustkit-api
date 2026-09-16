use axum::Json;
use serde_json::Value;

use crate::models::{ApiResponse, JsonInput};

fn extract_str(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

pub async fn wordcount(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let lines = text.lines().count();
    Json(ApiResponse {
        ok: true,
        result: Some(format!("words: {}, characters: {}, lines: {}", words, chars, lines).into()),
        error: None,
    })
}

pub async fn slugify(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let slug = text.to_lowercase().split_whitespace().collect::<Vec<&str>>().join("-").chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>();
    Json(ApiResponse { ok: true, result: Some(slug.into()), error: None })
}

pub async fn to_uppercase(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    Json(ApiResponse { ok: true, result: Some(text.to_uppercase().into()), error: None })
}

pub async fn to_lowercase(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    Json(ApiResponse { ok: true, result: Some(text.to_lowercase().into()), error: None })
}

pub async fn reverse(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    Json(ApiResponse { ok: true, result: Some(text.chars().rev().collect::<String>().into()), error: None })
}