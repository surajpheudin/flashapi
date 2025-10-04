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
