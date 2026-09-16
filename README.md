# 🦀 rustkit-api

A fast and lightweight JSON utility API built with Rust + Axum.

## Endpoints

Base URL: `http://localhost:3000`

### POST `/format`

Pretty print JSON.

```json
{ "json": "{\"name\":\"john\"}" }
```

### POST `/minify`

Minify JSON.

```json
{ "json": "{  \"name\":  \"john\"  }" }
```

### POST `/validate`

Validate JSON.

```json
{ "json": "{\"name\":\"john\"}" }
```

### POST `/escape`

Escape JSON string.

```json
{ "json": "{\"name\":\"john\"}" }
```

### POST `/unescape`

Unescape JSON string.

```json
{ "json": "{\\\"name\\\":\\\"john\\\"}" }
```

### POST `/flatten`

Flatten nested JSON.

```json
{ "json": "{\"user\":{\"name\":\"john\",\"age\":20}}" }
```

### POST `/diff`

Diff two JSON objects.

```json
{
  "left": "{\"name\":\"john\",\"age\":20}",
  "right": "{\"name\":\"john\",\"age\":21}"
}
```

## Running Locally

```bash
git clone https://github.com/Khansa01/rustkit-api
cd rustkit-api
cargo run
```

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Serde](https://serde.rs/)
- [Tokio](https://tokio.rs/)
