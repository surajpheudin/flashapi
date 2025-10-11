use crate::server::http::HttpMethod;
use percent_encoding::percent_decode_str;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

type RequestQuery = HashMap<String, Vec<String>>;

#[derive(Debug, Serialize)]
pub struct QueryParams(HashMap<String, Vec<String>>);

impl QueryParams {
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0.get(key)?.first().map(|s| s.as_str())
    }

    pub fn get_all(&self, key: &str) -> Option<&[String]> {
        self.0.get(key).map(|s| s.as_slice())
    }
}

#[derive(Debug)]
pub struct Request {
    pub body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub path: String,
    pub raw_body: Vec<u8>,
    pub query: Option<QueryParams>,
}

// ------------------------------------------------------------
// Calculate request query
// ------------------------------------------------------------
fn parse_query_multi(query: &str) -> QueryParams {
    let mut map = HashMap::new();

    for pair in query.split("&").filter(|p| !p.is_empty()) {
        let mut split = pair.split("=");

        let key = split.next().unwrap_or("");
        let value = split.next().unwrap_or("");

        let value = percent_decode_str(value).decode_utf8_lossy().to_string();

        map.entry(String::from(key))
            .or_insert_with(Vec::new)
            .push(value);
    }

    return QueryParams(map);
}

pub(crate) async fn get_request_info(stream: &mut TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);
    let mut content_length = 0;

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut request_line = String::new();

    // First line (e.g. GET /path HTTP/1.1)
    buf_reader.read_line(&mut request_line).await.unwrap();

    // ------------------------------------------------------------
    // Calculate request method
    // ------------------------------------------------------------
    dbg!(&request_line);
    let mut splits = request_line.split(" ");
    let request_method = splits.next().unwrap_or("");
    let method = match request_method {
        "DELETE" => HttpMethod::Delete,
        "GET" => HttpMethod::Get,
        "PATCH" => HttpMethod::Patch,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "HEAD" => HttpMethod::Head,
        "OPTIONS" => HttpMethod::Options,
        _ => HttpMethod::Get,
    };

    // ------------------------------------------------------------
    // Calculate request path
    // ------------------------------------------------------------
    let request_path_and_query = splits.next().unwrap_or("");
    let mut request_path_and_query_splits = request_path_and_query.split("?");
    let path = request_path_and_query_splits
        .next()
        .unwrap_or("")
        .to_string();

    // ------------------------------------------------------------
    // Calculate request query
    // ------------------------------------------------------------
    let query: Option<QueryParams> = match request_path_and_query_splits.next() {
        Some(query) => Some(parse_query_multi(query)),
        None => None,
    };

    // ------------------------------------------------------------
    // Calculate request headers
    // ------------------------------------------------------------
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

    // ------------------------------------------------------------
    // Calculate request body in JSON format
    // ------------------------------------------------------------
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
        query,
    }
}
