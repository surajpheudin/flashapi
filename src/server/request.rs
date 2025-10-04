use crate::server::http::HttpMethod;
use serde_json::Value;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub path: String,
    pub raw_body: Vec<u8>,
}

pub(crate) async fn get_request_info(stream: &mut TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);
    let mut content_length = 0;

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut request_line = String::new();

    // First line (e.g. GET /path HTTP/1.1)
    buf_reader.read_line(&mut request_line).await.unwrap();

    // Calculate headers by reading until an empty line is found
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).await.unwrap();

        if line == "\r\n" || line == "\n" {
            break;
        }

        if let Some((key, value)) = line.split_once(":") {
            headers.insert(key.trim().to_string(), value.trim().to_string());

            if key.to_lowercase() == "content-length" {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
    }

    // Calculate request method and path
    let mut splits = request_line.split(" ");
    let first_word = splits.next().unwrap_or("");
    let second_word = splits.next().unwrap_or("");

    let method = match first_word {
        "DELETE" => HttpMethod::Delete,
        "GET" => HttpMethod::Get,
        "PATCH" => HttpMethod::Patch,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "HEAD" => HttpMethod::Head,
        "OPTIONS" => HttpMethod::Options,
        _ => HttpMethod::Get,
    };

    let path = String::from(second_word);

    // Calculate request body in JSON format
    let mut raw_body = vec![0; content_length];
    let mut body: Option<Value> = None;

    if content_length > 0 {
        let result = buf_reader.read_exact(&mut raw_body).await;
        if let Ok(_) = result {
            let body_str = String::from_utf8_lossy(&raw_body);
            body = serde_json::from_str(&body_str).unwrap();
        }
    }

    Request {
        body,
        headers,
        method,
        path,
        raw_body,
    }
}
