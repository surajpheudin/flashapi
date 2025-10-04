# FlashAPI

[![Crates.io](https://img.shields.io/crates/v/flashapi.svg)](https://crates.io/crates/flashapi)
[![Docs.rs](https://docs.rs/flashapi/badge.svg)](https://docs.rs/flashapi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight webserver framework for Rust, designed to be simple and minimal.

---

## Features

- Lightweight and dependency-minimal
- Easy routing with handler functions
- JSON response support (via `serde` / `serde_json`)
- Simple API for quick prototyping

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
flashapi = "0.2.0"
```

## Quickstart

### Start server and register routes

```rust
#[tokio::main]
async fn main() {
    let mut server = HttpServer::new().with_state(());

    server.get(String::from("/user"), get_handler);
    server.post(String::from("/user"), post_handler);

    let _ = server.listen(Some(8000)).await;

}
```

### Simple request

```rust
async fn get_handler(_: Request, mut response: Response, _state: Arc<()>) {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    response.send(HttpStatus::Ok, "Data fetched successfully", "text/plain").await;

}
```

### Return JSON

```rust
#[derive(serde::Serialize)]
struct User {
    name: String,
}

async fn get_handler(_: Request, mut response: Response, _state: Arc<()>) {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let user = User {
        name: String::from("Flash API"),
    };

    response.send_json(HttpStatus::Ok, &user).await;
}
```

### Basic validations

```rust
async fn post_handler(request: Request, mut response: Response, _state: Arc<()>) {

    if let None = request.headers.get("Authorization") {
        response.send(HttpStatus::UNAUTHORIZED, "Unauthorized", "text/plain").await;
    }

    if let None = request.body {
        response.send(HttpStatus::BadRequest, "No body found", "text/plain").await;
    }

    response.send_json(HttpStatus::Ok, &request.body).await;
}
```

### Using with database

```rust
use flashapi::{HttpServer, HttpStatus, Request, Response};
use serde::Serialize;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, prelude::FromRow};
use std::sync::Arc;

struct AppState {
    db: Arc<Pool<Postgres>>,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await;

    if let Ok(pool) = pool {
        let app_state = AppState { db: Arc::new(pool) };

        let mut server = HttpServer::new().with_state(app_state);

        server.get(String::from("/user"), get_handler);

        let _ = server.listen(8000).await;
    } else {
        println!("Failed to connect to the database");
    }
}

#[derive(FromRow, Serialize)]
struct Deployment {
    name: String,
}

async fn get_handler(_: Request, mut response: Response, state: Arc<AppState>) {
    let rows: Vec<Deployment> = sqlx::query_as("SELECT * FROM deployment")
        .fetch_all(&*state.db)
        .await
        .unwrap();

    response.send_json(HttpStatus::Ok, &rows).await;
}
```
