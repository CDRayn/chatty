use std::path::Path;
use std::error::Error;

pub struct HttpRequest<'a>
{
    http_method: &'a str,
    uri: &'a Path,
    http_version: &'a str,
    body: Option<&'a str>,
}

pub fn parse_request(request: &str) -> Result<HttpRequest, Box<dyn Error>>
{
    // Break the request line up into its different components
    // A request line looks like: Method SP Request-URI SP HTTP-Version CRLF
    let request_line = request.lines().next().unwrap();
    let mut parts= request_line.split_whitespace();
    let method = parts.next().ok_or("Method not specified!")?;

    let mut body = None;

    match method
    {
        "GET" | "HEAD" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" => (),
        "POST" | "PUT" | "PATCH" => {
            let body_start = match request.find("\r\n")
            {
                Some(i) => i,
                None => Err("Bad request!")?,
            };
            body = Some(&request[body_start..]);
        },
        _ => Err("Unsupported method!")?,
    }

    let uri = Path::new(parts.next().ok_or("URI not specified")?);
    let http_version = parts.next().ok_or("HTTP version not specified")?;

    if http_version != "HTTP/1.1"
    {
        Err("Only HTTP/1.1 is supported!")?;
    }

    Ok(
        HttpRequest
        {
            http_method: method,
            uri,
            http_version,
            body,
        }
    )
}

#[cfg(test)]
mod tests
{
    use crate::http::{parse_request, HttpRequest};
    use std::path::Path;

    #[test]
    fn test_parse_request_get_pos()
    {
        let get_request = "GET /some/path/ HTTP/1.1\n";

        let result = parse_request(get_request).unwrap();
        let expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/some/path/"),
            http_version: "HTTP/1.1",
            body: None,
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        let get_request = "GET / HTTP/1.1\nHost: www.example.com\nConnection: keep-alive\n";

        let result = parse_request(get_request).unwrap();
        let expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }
}