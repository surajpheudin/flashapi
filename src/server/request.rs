use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use serde_json::Value;

use crate::server::http::HttpMethod;

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub request_body: Option<Value>,
}

pub(crate) fn get_request_info(stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);
    let mut content_length = 0;

    // Get headers before request body
    let mut headers_before_request_body = Vec::new();
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();

        if line == "\r\n" || line == "\n" {
            break;
        }

        if line.to_lowercase().starts_with("content-length:") {
            content_length = line
                .split(":")
                .nth(1)
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);
        }

        headers_before_request_body.push(line);
    }

    // Calculate request method and path
    let mut splits = headers_before_request_body[0].split(" ");
    let first_word = splits.next().unwrap();
    let second_word = splits.next().unwrap();

    let method = match first_word {
        "DELETE" => HttpMethod::Delete,
        "GET" => HttpMethod::Get,
        "PATCH" => HttpMethod::Patch,
        "POST" => HttpMethod::Post,
        _ => HttpMethod::Get,
    };

    let path = String::from(second_word);

    // Calculate request body in JSON format
    let mut body = vec![0; content_length];
    let mut request_body: Option<Value> = None;

    if content_length > 0 {
        let result = buf_reader.read_exact(&mut body);
        if let Ok(_) = result {
            let body_str = String::from_utf8_lossy(&body);
            request_body = serde_json::from_str(&body_str).unwrap();
        }
    }

    Request {
        method,
        path,
        request_body,
    }
}
