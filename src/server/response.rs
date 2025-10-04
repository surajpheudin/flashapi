use crate::server::http::HttpStatus;
use serde::Serialize;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn send(&mut self, status: HttpStatus, body: &str, content_type: &str) {
        let length = body.len();
        let status_code = status.code();
        let response = format!(
            "HTTP/1.1 {status_code}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\nConnection: close\r\n\r\n{body}"
        );

        let res = self.stream.write_all(response.as_bytes()).await;

        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {e}");

                self.stream.write_all(response.as_bytes()).await.unwrap();
                return ();
            }
        }
    }

    pub async fn send_json<T: Serialize>(&mut self, status: HttpStatus, data: &T) {
        if let Ok(body) = serde_json::to_string(data) {
            self.send(status, &body, "application/json").await;
        } else {
            self.send(
                HttpStatus::InternalServerError,
                "Internal server error",
                "text/plain",
            )
            .await;
        }
    }
}
