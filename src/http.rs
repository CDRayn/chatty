use std::path::Path;
use std::error::Error;

/// Represents a parsed incoming HTTP request
/// TODO: Add equality comparison implementation for struct
pub struct HttpRequest<'a>
{
    http_method: &'a str,
    uri: &'a Path,
    http_version: &'a str,
    body: Option<&'a str>,
}

/// Parse a HTTP request
///
/// # Parameters
///
/// - `request`: a reference to the `str` of data to parse as an HTTP request
///
/// # Returns
///
/// A `Result` which is:
///
/// - `OK`: A `HttpRequest` struct containing the information parsed from the HTTP request
/// - `Box`: Returns an error encapsulated in a `Box`.
/// TODO: replace the boxed error with an enum of possible error types.
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
            // If the request's method should have a body, find the start of the body with
            // as indicated with the CRLF.
            let body_start = match request.find("\r\n")
            {
                Some(i) => i,
                None => Err("Bad request!")?,
            };
            body = Some(&request[body_start..]);
        },
        // Return an error for any invalid method.
        _ => Err("Unsupported method!")?,
    }

    let uri = Path::new(parts.next().ok_or("URI not specified")?);
    let http_version = parts.next().ok_or("HTTP version not specified")?;

    // Return an error for any requests that aren't HTTP/1.1
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

    /// Verify that the `parse_request()` function correctly parses valid HTTP GET requests
    /// by returning a `Request` struct containing the HTTP request's details.
    #[test]
    fn test_parse_request_get_valid()
    {
        // Test the parsing of a simple GET request containing no HTTP headers.
        let mut get_request = "GET / HTTP/1.1\r\n";

        let mut result = parse_request(get_request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a simple GET request that contains HTTP headers.
        get_request =
        "GET / HTTP/1.1
        Host: www.example.com
        Connection: keep-alive\r\n";

        result = parse_request(get_request).unwrap();
        expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a GET request with a more complex resource path and HTTP headers.
        get_request =
        "GET /some/path/ HTTP/1.1
        Host: www.example.com
        Connection: keep-alive\r\n";

        result = parse_request(get_request).unwrap();
        expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a GET request with a larger number of HTTP headers
        get_request =
        "GET /some/path/ HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(get_request).unwrap();
        expected_result = HttpRequest {
            http_method: "GET",
            uri: Path::new("/some/path/"),
            http_version: "HTTP/1.1",
            body: None
        };
        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_request()` function correctly parses invalid HTTP GET requests
    /// by returning an error.
    #[test]
    fn test_parse_request_get_invalid()
    {
        // Test that an error is raised when no path is included
        let mut bad_get_request = "GET HTTP/1.1\r\n";
        let mut result = parse_request(bad_get_request).is_err();
        assert!(result);

        // Test that an error is raised for unsupported HTTP versions
        bad_get_request = "GET /some/path HTTP/2.0\r\n";
        result = parse_request(bad_get_request).is_err();
        assert!(result);

        // Test that an error is raised when space characters are absent
        bad_get_request = "GET /some/pathHTTP/1.1\r\n";
        result = parse_request(bad_get_request).is_err();
        assert!(result);

        // Test that an error is raised when a newline is missing between the request line
        // and headers.
        bad_get_request = "GET /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_get_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_request()` function correctly parses valid HTTP HEAD requests
    /// by returning a `Request` struct containing the HTTP request's details.
    #[test]
    fn test_parse_request_head_valid()
    {
        // Test the parsing of a simple HEAD request containing no HTTP headers.
        let mut head_request = "HEAD / HTTP/1.1\r\n";
        let mut result = parse_request(head_request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "HEAD",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a simple HEAD request with a more elaborate path.
        head_request = "HEAD /some/path HTTP/1.1\r\n";
        let result = parse_request(head_request).unwrap();
        let expected_result = HttpRequest {
            http_method: "HEAD",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }
}