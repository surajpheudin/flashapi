use serde::Serialize;

use crate::server::{request::Request, response::Response, server::HttpServer};

mod server;

#[derive(Serialize)]
struct User {
    id: u16,
    name: String,
}

fn main() {
    let mut server = HttpServer::new();

    fn handler(_: Request, response: &mut Response) {
        let user: User = User {
            id: 1,
            name: String::from("John Doe"),
        };
        response.send_json(server::http::HttpStatus::Ok, &user);
    }

    fn post_handler(request: Request, response: &mut Response) {
        response.send_json(server::http::HttpStatus::Ok, &request);
    }

    server.get(String::from("/user"), handler);
    server.post(String::from("/user"), post_handler);
    server.listen(Some(8000));
}
