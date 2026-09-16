use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use image::{GrayImage, Luma};
use qrcode::QrCode;
use serde_json::Value;

use crate::models::{ApiResponse, JsonInput};

pub async fn generate_qr(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let text = match &body.json {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    match QrCode::new(text.as_bytes()) {
        Ok(code) => {
            let image = code.render::<Luma<u8>>().build();
            let mut buf = Vec::new();
            let gray = GrayImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap();
            gray.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();
            let base64 = general_purpose::STANDARD.encode(&buf);
            Json(ApiResponse { ok: true, result: Some(format!("data:image/png;base64,{}", base64).into()), error: None })
        }
        Err(e) => Json(ApiResponse { ok: false, result: None, error: Some(e.to_string()) }),
    }
}