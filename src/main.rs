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

    server.get(String::from("/user"), handler);
    server.listen(Some(7878));
}
