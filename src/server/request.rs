use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use serde_json::Value;

use crate::server::http::HttpMethod;

pub struct Request {
    pub body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub path: String,
}

pub(crate) fn get_request_info(stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);
    let mut content_length = 0;

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut request_line = String::new();

    // First line (e.g. GET /path HTTP/1.1)
    buf_reader.read_line(&mut request_line).unwrap();

    // Calculate headers by reading until an empty line is found
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();

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
        _ => HttpMethod::Get,
    };

    let path = String::from(second_word);

    // Calculate request body in JSON format
    let mut http_request_body = vec![0; content_length];
    let mut body: Option<Value> = None;

    if content_length > 0 {
        let result = buf_reader.read_exact(&mut http_request_body);
        if let Ok(_) = result {
            let body_str = String::from_utf8_lossy(&http_request_body);
            body = serde_json::from_str(&body_str).unwrap();
        }
    }

    Request {
        body,
        headers,
        method,
        path,
    }
}
