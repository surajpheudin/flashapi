use std::collections::HashMap;

pub(crate) fn match_route(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let mut params: HashMap<String, String> = HashMap::new();

    let pattern_parts: Vec<_> = pattern.trim_matches('/').split('/').collect();
    let path_parts: Vec<_> = path.trim_matches('/').split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    for (p, actual) in pattern_parts.iter().zip(path_parts.iter()) {
        if p.starts_with(":") {
            params.insert(p[1..].to_string(), actual.to_string());
        } else if p != actual {
            return None;
        }
    }

    return Some(params);
}
