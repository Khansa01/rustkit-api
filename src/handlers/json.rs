use axum::Json;
use serde_json::Value;

use crate::models::{ApiResponse, DiffInput, JsonInput};

fn extract_str(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

pub async fn format_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let json_str = extract_str(&body.json);
    match serde_json::from_str::<Value>(&json_str) {
        Ok(v) => Json(ApiResponse {
            ok: true,
            result: Some(v),
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
    let json_str = extract_str(&body.json);
    match serde_json::from_str::<Value>(&json_str) {
        Ok(v) => Json(ApiResponse {
            ok: true,
            result: Some(v),
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
    let json_str = extract_str(&body.json);
    match serde_json::from_str::<Value>(&json_str) {
        Ok(_) => Json(ApiResponse {
            ok: true,
            result: Some("Valid JSON".into()),
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
    let json_str = extract_str(&body.json);
    let escaped = json_str
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    Json(ApiResponse {
        ok: true,
        result: Some(escaped.into()),
        error: None,
    })
}

pub async fn unescape_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let json_str = extract_str(&body.json);
    let unescaped = json_str
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\\", "\\");

    match serde_json::from_str::<Value>(&unescaped) {
        Ok(v) => Json(ApiResponse {
            ok: true,
            result: Some(v),  
            error: None,
        }),
        Err(_) => Json(ApiResponse {
            ok: true,
            result: Some(unescaped.into()),  
            error: None,
        }),
    }
}

pub async fn flatten_json(Json(body): Json<JsonInput>) -> Json<ApiResponse> {
    let json_str = extract_str(&body.json);
    match serde_json::from_str::<Value>(&json_str) {
        Ok(v) => {
            let mut flat = serde_json::Map::new();
            flatten_value("", &v, &mut flat);
            Json(ApiResponse {
                ok: true,
                result: Some(Value::Object(flat)),
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
    let left_str = extract_str(&body.left);
    let right_str = extract_str(&body.right);

    let left: Value = match serde_json::from_str(&left_str) {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse {
            ok: false,
            result: None,
            error: Some(format!("left: {}", e)),
        }),
    };

    let right: Value = match serde_json::from_str(&right_str) {
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
        result: Some(diff.join("\n").into()),
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