use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use crate::server::{
    http::{HttpMethod, HttpStatus},
    request::{Request, get_request_info},
    response::Response,
};

/// # Examples
/// ```ignore
/// use serde::Serialize;
/// use flashapi::{HttpServer, HttpStatus, Request, Response};
///
/// #[derive(Serialize)]
/// struct User {
///     id: u16,
///     name: String,
/// }
///
/// let mut server = HttpServer::new();
///
/// fn handler(_: Request, response: &mut Response) {
///     let user: User = User {
///         id: 1,
///         name: String::from("John Doe"),
///     };
///     response.send_json(HttpStatus::Ok, &user);
/// }
///
/// server.get(String::from("/user"), handler);
/// server.listen(Some(7878));
/// ```
pub type HandlerFn = fn(Request, &mut Response);

/// # Exaamples
/// ```ignore
/// use serde::Serialize;
/// use flashapi::{HttpServer, HttpStatus, Request, Response};
///
/// #[derive(Serialize)]
/// struct User {
///     id: u16,
///     name: String,
/// }
///
/// fn main() {
///     let mut server = HttpServer::new();
///
///     fn handler(_: Request, response: &mut Response) {
///         let user: User = User {
///             id: 1,
///             name: String::from("John Doe"),
///         };
///         response.send_json(HttpStatus::Ok, &user);
///     }
///
///     server.get(String::from("/user"), handler);
///     server.listen(Some(7878));
/// }
/// ```
pub struct HttpServer {
    routes: HashMap<(HttpMethod, String), HandlerFn>,
}

impl HttpServer {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn delete(&mut self, path: String, handler: HandlerFn) {
        self.routes.insert((HttpMethod::Delete, path), handler);
    }

    pub fn get(&mut self, path: String, handler: HandlerFn) {
        self.routes.insert((HttpMethod::Get, path), handler);
    }

    pub fn patch(&mut self, path: String, handler: HandlerFn) {
        self.routes.insert((HttpMethod::Patch, path), handler);
    }

    pub fn post(&mut self, path: String, handler: HandlerFn) {
        self.routes.insert((HttpMethod::Post, path), handler);
    }

    pub fn listen(&mut self, port: Option<u16>) {
        let open_port = match port {
            Some(p) => p,
            None => 7878,
        };

        let res = TcpListener::bind("127.0.0.1:".to_string() + &open_port.to_string());

        if let Ok(listener) = res {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => Self::handle_connection(self, stream),
                    Err(error) => println!("Connection failed: {error}"),
                }
            }
        } else if let Err(error) = res {
            eprintln!("error: {error}");
        }
    }

    fn handle_connection(&mut self, stream: TcpStream) {
        let request_info = get_request_info(&stream);

        let route = self.routes.get(&(
            request_info.method.clone(),
            String::from(&request_info.path),
        ));

        let mut response: Response = Response::new(stream);
        if let Some(handler) = route {
            handler(request_info, &mut response);
            return;
        }

        response.send(HttpStatus::NotFound, "Route not found.", "text/plain");
    }
}
