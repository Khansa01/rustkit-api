# 🦀 rustkit-api

A fast and lightweight utility API built with Rust + Axum.

## Endpoints

Base URL: `http://localhost:3000`

---

### 📦 JSON

#### POST `/format`

Pretty print JSON.

```json
{ "json": { "name": "john doe" } }
```

#### POST `/minify`

Minify JSON.

```json
{ "json": { "name": "john doe" } }
```

#### POST `/validate`

Validate JSON.

```json
{ "json": { "name": "john doe" } }
```

#### POST `/escape`

Escape JSON string.

```json
{ "json": { "name": "john doe" } }
```

#### POST `/unescape`

Unescape JSON string.

```json
{ "json": "{\"name\":\"john doe\"}" }
```

#### POST `/flatten`

Flatten nested JSON.

```json
{ "json": { "user": { "name": "john", "age": 20 } } }
```

#### POST `/diff`

Diff two JSON objects.

```json
{ "left": { "age": 20 }, "right": { "age": 21 } }
```

---

### 📝 Text

#### POST `/text/wordcount`

Count words, characters, and lines.

```json
{ "json": "hello world" }
```

#### POST `/text/slugify`

Convert text to slug.

```json
{ "json": "Hello World" }
```

#### POST `/text/uppercase`

Convert to uppercase.

```json
{ "json": "hello world" }
```

#### POST `/text/lowercase`

Convert to lowercase.

```json
{ "json": "HELLO WORLD" }
```

#### POST `/text/reverse`

Reverse text.

```json
{ "json": "hello world" }
```

---

### 📷 QR Code

#### POST `/qr/generate`

Generate QR code as base64 image.

```json
{ "json": "https://github.com/Khansa01/rustkit-api" }
```

---

### 🔧 Utils

#### POST `/utils/base64/encode`

Encode to base64.

```json
{ "json": "hello world" }
```

#### POST `/utils/base64/decode`

Decode from base64.

```json
{ "json": "aGVsbG8gd29ybGQ=" }
```

#### POST `/utils/url/encode`

URL encode.

```json
{ "json": "hello world & more" }
```

#### POST `/utils/url/decode`

URL decode.

```json
{ "json": "hello%20world%20%26%20more" }
```

#### POST `/utils/hash/md5`

Generate MD5 hash.

```json
{ "json": "hello world" }
```

#### POST `/utils/hash/sha256`

Generate SHA256 hash.

```json
{ "json": "hello world" }
```

#### POST `/utils/jwt/encode`

Encode JWT token.

```json
{
  "json": "{\"payload\":{\"name\":\"john doe\",\"id\":123},\"secret\":\"mysecret\"}"
}
```

#### POST `/utils/jwt/decode`

Decode JWT token.

```json
{ "json": "eyJhbGciOiJIUzI1NiJ9..." }
```

#### POST `/utils/ip`

Get info for a specific IP.

```json
{ "json": "8.8.8.8" }
```

#### GET `/utils/myip`

Get info for your current IP. No body needed.

---

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
- [Reqwest](https://docs.rs/reqwest)
