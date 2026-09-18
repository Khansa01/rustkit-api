use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use hex;
use jsonwebtoken::{encode, EncodingKey, Header};
use md5::Md5;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use axum::extract::ConnectInfo;
use std::net::SocketAddr;

use crate::models::{ApiResponse, JsonInput};

fn extract_str(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[derive(Serialize, Deserialize)]
struct JwtInput {
    payload: Value,
    secret: String,
}

pub async fn base64_encode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let encoded = general_purpose::STANDARD.encode(text.as_bytes());
    Json(ApiResponse { ok: true, result: Some(encoded.into()), error: None })
}

pub async fn base64_decode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    match general_purpose::STANDARD.decode(text.as_bytes()) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(decoded) => Json(ApiResponse { ok: true, result: Some(decoded.into()), error: None }),
            Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
        },
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}

pub async fn url_encode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let encoded = urlencoding::encode(&text).to_string();
    Json(ApiResponse { ok: true, result: Some(encoded.into()), error: None })
}

pub async fn url_decode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    match urlencoding::decode(&text) {
        Ok(decoded) => Json(ApiResponse { ok: true, result: Some(decoded.to_owned().into()), error: None }),
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}

pub async fn hash_md5(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let mut hasher = Md5::new();
    hasher.update(text.as_bytes());
    let result = hex::encode(hasher.finalize());
    Json(ApiResponse { ok: true, result: Some(result.into()), error: None })
}

pub async fn hash_sha256(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hex::encode(hasher.finalize());
    Json(ApiResponse { ok: true, result: Some(result.into()), error: None })
}

pub async fn jwt_decode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let token = extract_str(&body.json);
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        return Json(ApiResponse {
            ok: false,
            result: None,
            error: Some("Invalid JWT format".to_string()),
        });
    }

    let decode_part = |part: &str| -> Result<Value, String> {
        let bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(part)
            .map_err(|e| e.to_string())?;
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())
    };

    match (decode_part(parts[0]), decode_part(parts[1])) {
        (Ok(header), Ok(payload)) => {
            let result = serde_json::json!({ "header": header, "payload": payload });
            Json(ApiResponse { ok: true, result: Some(result), error: None })
        }
        (Err(e), _) | (_, Err(e)) => Json(ApiResponse { ok: false, result: None, error: Some(e) }),
    }
}

pub async fn ip_info(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let ip = extract_str(&body.json);
    let url = format!("http://ip-api.com/json/{}", ip);
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(data) => Json(ApiResponse { ok: true, result: Some(data), error: None }),
            Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
        },
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}

pub async fn jwt_encode(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = extract_str(&body.json);
    let input: JwtInput = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    };

    match encode(
        &Header::default(),
        &input.payload,
        &EncodingKey::from_secret(input.secret.as_bytes()),
    ) {
        Ok(token) => Json(ApiResponse { ok: true, result: Some(token.into()), error: None }),
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}

pub async fn my_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<ApiResponse> {
    let ip = addr.ip().to_string();
    let url = format!("http://ip-api.com/json/{}", ip);
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(data) => Json(ApiResponse { ok: true, result: Some(data), error: None }),
            Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
        },
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}