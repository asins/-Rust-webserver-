use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                continue;
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

/**
 * Http request第一行的解析
 * @example
 *   GET /fanyi HTTP/1.1
 */
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

/**
 * Http request header的解析
 * @example
 *   Host: localhost
 */
fn process_header_line(s: &str) -> (String, String) {
    let mut header_item = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_item.next() {
        key = k.to_string();
    }
    if let Some(v) = header_item.next() {
        value = v.trim_start().to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_http() {
        let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost\r\nAccept: */*\r\nUser-Agent: curl/7.79.1\r\n\r\ntest");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("User-Agent".into(), "curl/7.79.1".into());
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
        assert_eq!("test", req.msg_body);
    }

    #[test]
    fn test_request_header_split() {
        let str = "Host: localhost";
        let mut header_item = str.split(":");
        let mut key = String::from("");
        let mut value = String::from("");

        if let Some(k) = header_item.next() {
            key = k.to_string();
        }
        if let Some(v) = header_item.next() {
            value = v.trim_start().to_string();
        }
        assert_eq!(key, "Host");
        assert_eq!(value, "localhost");
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
}
