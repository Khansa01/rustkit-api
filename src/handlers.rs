use axum::Json;
use serde_json::Value;

use crate::models::{ApiResponse, DiffInput, JsonInput};

pub async fn format_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    match serde_json::from_str::<Value>(&body.json) {
        Ok(v) => Json(ApiResponse {
            ok: true,
            result: Some(serde_json::to_string_pretty(&v).unwrap()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn minify_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    match serde_json::from_str::<Value>(&body.json) {
        Ok(v) => Json(ApiResponse {
            ok: true,
            result: Some(v.to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn validate_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    match serde_json::from_str::<Value>(&body.json) {
        Ok(_) => Json(ApiResponse {
            ok: true,
            result: Some("Valid JSON".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn escape_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let escaped = body.json
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    Json(ApiResponse {
        ok: true,
        result: Some(escaped),
        error: None,
    })
}

pub async fn unescape_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let unescaped = body.json
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\\", "\\");

    Json(ApiResponse {
        ok: true,
        result: Some(unescaped),
        error: None,
    })
}

pub async fn flatten_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    match serde_json::from_str::<Value>(&body.json) {
        Ok(v) => {
            let mut flat = serde_json::Map::new();
            flatten_value("", &v, &mut flat);
            Json(ApiResponse {
                ok: true,
                result: Some(serde_json::to_string_pretty(&flat).unwrap()),
                error: None,
            })
        }
        Err(e) => Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

fn flatten_value(prefix: &str, value: &Value, flat: &mut serde_json::Map<String, Value>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_value(&key, v, flat);
            }
        }
        _ => {
            flat.insert(prefix.to_string(), value.clone());
        }
    }
}

pub async fn diff_json(Json(body): Json<DiffInput>) -> Json<ApiResponse> {
    let left: Value = match serde_json::from_str(&body.left) {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(format!("left: {}", e)),
        }),
    };

    let right: Value = match serde_json::from_str(&body.right) {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(format!("right: {}", e)),
        }),
    };

    let diff = compare_values("root", &left, &right);

    Json(ApiResponse {
        ok: true,
        result: Some(diff.join("\n")),
        error: None,
    })
}

fn compare_values(path: &str, left: &Value, right: &Value) -> Vec<String> {
    let mut diffs = vec![];

    match (left, right) {
        (Value::Object(l), Value::Object(r)) => {
            for (k, lv) in l {
                let p = format!("{}.{}", path, k);
                match r.get(k) {
                    Some(rv) => diffs.extend(compare_values(&p, lv, rv)),
                    None => diffs.push(format!("- {}: {}", p, lv)),
                }
            }
            for (k, rv) in r {
                if !l.contains_key(k) {
                    diffs.push(format!("+ {}.{}: {}", path, k, rv));
                }
            }
        }
        _ => {
            if left != right {
                diffs.push(format!("~ {}: {} -> {}", path, left, right));
            }
        }
    }

    diffs
}