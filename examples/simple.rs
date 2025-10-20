use flashapi::{HttpServer, HttpStatus, Request, Response};
use serde::Serialize;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut server = HttpServer::new().with_state(());

    server.get(String::from("/user"), get_handler);
    server.get(String::from("/user/:id"), get_single_user_handler);

    let _ = server.listen(8000).await;
}

#[derive(Serialize)]
struct User {
    id: u16,
    name: String,
}

async fn get_handler(request: Request, mut response: Response, _state: Arc<()>) {
    let user = User {
        id: 1,
        name: String::from("Rust rusty"),
    };

    let user_2 = User {
        id: 2,
        name: String::from("Happy rusty"),
    };

    let search_query = match &request.query {
        Some(q) => q.get_one("search"),
        None => None,
    };

    let mut users = Vec::new();
    users.push(user);
    users.push(user_2);

    let filtered = match search_query {
        Some(query) => users
            .into_iter()
            .filter(|user| user.name.to_lowercase().contains(query))
            .collect(),
        None => users,
    };

    response.send_json(HttpStatus::Ok, &filtered).await;
}

async fn get_single_user_handler(request: Request, mut response: Response, _state: Arc<()>) {
    let user = User {
        id: 1,
        name: String::from("Rust rusty"),
    };

    let user_2 = User {
        id: 2,
        name: String::from("Happy rusty"),
    };

    let mut users = Vec::new();
    users.push(user);
    users.push(user_2);

    let mut user: Option<&User> = None;
    if let Some(params) = &request.params {
        if let Some(id) = params.get("id") {
            user = users.iter().find(|u| u.id.to_string() == id.to_string());
        }
    }

    response.send_json(HttpStatus::Ok, &user).await;
}
