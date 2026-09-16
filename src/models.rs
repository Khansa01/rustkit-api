use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct JsonInput {
    pub json: Value,
}

#[derive(Deserialize)]
pub struct DiffInput {
    pub left: Value,
    pub right: Value,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub ok: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}