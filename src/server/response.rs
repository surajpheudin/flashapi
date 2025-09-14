use serde::Serialize;
use std::{io::Write, net::TcpStream};

use crate::server::http::HttpStatus;

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Send a response with the given status, body, and content type.
    ///
    /// # Examples
    /// ```
    /// use flashapi::{HttpStatus, Request, Response};
    ///
    /// fn handler(_: Request, response: &mut Response){
    ///     response.send(HttpStatus::Ok, "Hello world!", "text/plain");
    /// }
    /// ```
    pub fn send(&mut self, status: HttpStatus, body: &str, content_type: &str) {
        let length = body.len();
        let status_code = status.code();
        let response = format!(
            "HTTP/1.1 {status_code}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\nConnection: close\r\n\r\n{body}"
        );

        let res = self.stream.write_all(response.as_bytes());

        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {e}");

                self.stream.write_all(response.as_bytes());
                return ();
            }
        }
    }

    /// Send a response with the given status and body.
    ///
    /// # Examples
    /// ```
    /// use serde::Serialize;
    /// use flashapi::{HttpStatus, Request, Response};
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     id: u16,
    ///     name: String,
    /// }
    ///
    /// fn handler(_: Request, response: &mut Response){
    ///     let user_1: User = User {
    ///         id: 1,
    ///         name: String::from("John Doe"),
    ///     };
    ///     response.send_json(HttpStatus::Ok, &user_1);
    /// }
    /// ```
    pub fn send_json<T: Serialize>(&mut self, status: HttpStatus, data: &T) {
        if let Ok(body) = serde_json::to_string(data) {
            self.send(status, &body, "application/json");
        } else {
            self.send(
                HttpStatus::InternalServerError,
                "Internal server error",
                "text/plain",
            )
        }
    }
}
