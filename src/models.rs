use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonInput {
    pub json: String,
}

#[derive(Deserialize)]
pub struct DiffInput {
    pub left: String,
    pub right: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub ok: bool,
    pub result: Option<String>,
    pub error: Option<String>,
}