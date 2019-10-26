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

    if request.ends_with("\r\n") == false
    {
        return Err("Bad request!")?
    }

    match method
    {
        "GET" | "HEAD" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" => (),
        // TODO: There is probably a cleaner way to parse requests with a body.
        "POST" | "PUT" | "PATCH" => {
            // If the request's method should have a body, find the start of the body
            // as indicated with the CRLF.
            let body_start = match request.find("\r\n")
            {
                Some(i) => i + 2,
                None => Err("Bad request!")?,
            };
            let body_end = match request.rfind("\r\n")
            {
                Some(i) => i,
                None => Err("Bad request!")?,
            };
            //  If the request only has one CRLF, then the body is empty / missing so return an error
            if body_start >= body_end
            {
                return Err("Bad request!")?;
            }

            body = Some(&request[body_start .. body_end]);
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
    use super::*;
    use std::path::Path;

    /// Verify that the `parse_request()` function correctly parses valid HTTP GET requests
    /// by returning a `Request` struct containing the HTTP request's details.
    #[test]
    fn test_parse_request_get_valid()
    {
        // Test the parsing of a simple GET request containing no HTTP headers.
        let mut request = "GET / HTTP/1.1\r\n";

        let mut result = parse_request(request).unwrap();
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
        request =
        "GET / HTTP/1.1
        Host: www.example.com
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
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
        request =
        "GET /some/path/ HTTP/1.1
        Host: www.example.com
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
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
        request =
        "GET /some/path/ HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
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
        let mut bad_request = "GET HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        // Test that an error is raised for unsupported HTTP versions
        bad_request = "GET /some/path HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Test that an error is raised when space characters are absent
        bad_request = "GET /some/pathHTTP/1.1\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Test that an error is raised when a newline is missing between the request line
        // and headers.
        bad_request = "GET /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_request()` function correctly parses valid HTTP HEAD requests
    /// by returning a `Request` struct containing the HTTP request's details.
    #[test]
    fn test_parse_request_head_valid()
    {
        // Test the parsing of a simple HEAD request containing no HTTP headers.
        let mut request = "HEAD / HTTP/1.1\r\n";
        let mut result = parse_request(request).unwrap();
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
        request = "HEAD /some/path HTTP/1.1\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "HEAD",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a simple HEAD request with HTTP headers.
        request = "HEAD / HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "HEAD",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a simple HEAD request with HTTP headers and a non root path.
        request = "HEAD /some/path HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
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
    /// Verify that the `parse_http_request()` function returns an error for invalid HTTP HEAD requests.
    #[test]
    fn test_parse_http_request_head_invalid()
    {
        // Test that an error is raised when no path is included
        let mut bad_request = "HEAD HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "HEAD / HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "HEAD /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_http_request()` function correctly parses a HTTP DELETE request
    /// by returning a `HttpRequest` struct with the parsed contents of the request.
    #[test]
    fn test_parse_http_request_delete_valid()
    {
        // Test the parsing of a simple DELETE request containing no HTTP headers.
        let mut request = "DELETE / HTTP/1.1\r\n";
        let mut result = parse_request(request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "DELETE",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a DELETE request with a non root path.
        request = "DELETE /some/path HTTP/1.1\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "DELETE",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a DELETE request with a non root path and HTTP headers.
        request = "DELETE /some/path HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "DELETE",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_http_request()` function returns an error for invalid HTTP DELETE requests.
    #[test]
    fn test_parse_http_request_delete_invalid()
    {
        // Test that an error is raised when no path is included
        let mut bad_request = "DELETE HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "DELETE / HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "DELETE /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_http_request()` function correctly parses a CONNECT HTTP request
    /// by return an `HttpRequest` struct containing the parsed contents of the request.
    #[test]
    fn test_parse_http_request_connect_valid()
    {
        // Test the parsing of a simple CONNECT request containing no HTTP headers.
        let mut request = "CONNECT / HTTP/1.1\r\n";
        let mut result = parse_request(request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "CONNECT",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a CONNECT request with a non root path.
        request = "CONNECT /some/path HTTP/1.1\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "CONNECT",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a CONNECT request with a non root path and HTTP headers.
        request = "CONNECT /some/path HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "CONNECT",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_http_request()` function returns a error for any invalid CONNECT HTTP requests.
    #[test]
    fn test_parse_http_request_connect_invalid()
    {
        // Test that an error is raised when no path is included
        let mut bad_request = "CONNECT HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "CONNECT / HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "CONNECT /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_http_request()` function correctly parses OPTIONS HTTP requests
    /// by return an `HttpRequest` struct containing the parsed contents for the request.
    #[test]
    fn test_parse_http_request_options_valid()
    {
        // Test the parsing of a simple OPTIONS request containing no HTTP headers.
        let mut request = "OPTIONS / HTTP/1.1\r\n";
        let mut result = parse_request(request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "OPTIONS",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a OPTIONS request with a non root path.
        request = "OPTIONS /some/path HTTP/1.1\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "OPTIONS",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a OPTIONS request with a non root path and HTTP headers.
        request = "OPTIONS /some/path HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "OPTIONS",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_http_request()` function returns an error for invalid OPTIONS HTTP requests.
    #[test]
    fn test_parse_http_request_options_invalid()
    {
        // Verify that an error is raised when no path is included in the request line.
        let mut bad_request = "OPTIONS HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is raised for unsupported versions of HTTP.
        bad_request = "OPTIONS / HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is raised if a new line is missing between the request
        // line and the HTTP headers.
        bad_request = "OPTIONS /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_http_request()` function correctly parses a TRACE HTTP request
    /// by returning a `HttpRequest` struct containing the parsed contents of the request.
    #[test]
    fn test_parse_http_request_trace_valid()
    {
        // Test the parsing of a simple TRACE request containing no HTTP headers.
        let mut request = "TRACE / HTTP/1.1\r\n";
        let mut result = parse_request(request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "TRACE",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a TRACE request with a non root path.
        request = "TRACE /some/path HTTP/1.1\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "TRACE",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a TRACE request with a non root path and HTTP headers.
        request = "TRACE /some/path HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "TRACE",
            uri: Path::new("/some/path"),
            http_version: "HTTP/1.1",
            body: None,
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_http_request()` function returns an error for invalid TRACE HTTP requests.
    #[test]
    fn test_parse_http_request_trace_invalid()
    {
        // Test that an error is raised when no path is included
        let mut bad_request = "TRACE HTTP/1.1\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "TRACE / HTTP/2.0\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "TRACE /some/path HTTP/1.1Host: www.example.com\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }

    /// Verify that the `parse_http_request()` function correctly parses a POST HTTP request
    /// by returning a `HttpRequest` struct containing the parsed contents of the request.
    #[test]
    fn test_parse_http_request_post_valid()
    {
        // Test the parsing of a simple POST request containing no HTTP headers.
        let mut request = "POST / HTTP/1.1\r\n{id: 2345, message: \"Hello\"}\r\n";
        let mut result = parse_request(request).unwrap();
        let mut expected_result = HttpRequest {
            http_method: "POST",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: Option::from("{id: 2345, message: \"Hello\"}"),
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a POST request with a more elaborate path and no HTTP headers.
        request = "POST /messages HTTP/1.1\r\n{id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "POST",
            uri: Path::new("/messages"),
            http_version: "HTTP/1.1",
            body: Option::from("{id: 2345, message: \"Hello\"}"),
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);

        // Test the parsing of a POST request containing a simple path and HTTP headers.
        request = "POST / HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        \r\n{id: 2345, message: \"Hello\"}\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "POST",
            uri: Path::new("/"),
            http_version: "HTTP/1.1",
            body: Option::from("{id: 2345, message: \"Hello\"}"),
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
        
        // Test the parsing of a POST request containing a more elaborate path and HTTP headers.
        request = "POST /messages HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        \r\n{id: 2345, message: \"Hello\"}\r\n";

        result = parse_request(request).unwrap();
        expected_result = HttpRequest {
            http_method: "POST",
            uri: Path::new("/messages"),
            http_version: "HTTP/1.1",
            body: Option::from("{id: 2345, message: \"Hello\"}"),
        };

        assert_eq!(result.http_method, expected_result.http_method);
        assert_eq!(result.uri, expected_result.uri);
        assert_eq!(result.http_version, expected_result.http_version);
        assert_eq!(result.body, expected_result.body);
    }

    /// Verify that the `parse_http_request()` function returns an error for invalid POST HTTP requests.
    #[test]
    fn test_parse_http_request_post_invalid()
    {
        // Verify that an error is raised when no path is included
        let mut bad_request = "POST HTTP/1.1\r\n{id: 2345, message: \"Hello\"}\r\n";
        let mut result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is raised for unsupported versions of HTTP.
        bad_request = "POST / HTTP/2.0\r\n{id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "POST / HTTP/1.0\r\n{id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        bad_request = "POST / HTTP/0.9\r\n{id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is returned if a new line is missing between the request
        // line and the HTTP headers.
        bad_request = "POST / HTTP/1.1Host: www.example.com
        {id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is returned if the CRLF between the headers and the body is missing.
        bad_request = "POST /messages HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        {id: 2345, message: \"Hello\"}\r\n";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is returned if the body is not terminated with CRLF.
        bad_request = "POST /messages HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        \r\n{id: 2345, message: \"Hello\"}";
        result = parse_request(bad_request).is_err();
        assert!(result);

        // Verify that an error is returned if the CRLF between the headers and body is missing
        // and the body is not terminated with CRLF.
        bad_request = "POST /messages HTTP/1.1
        Host: www.example.com
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:69.0) Gecko/20100101 Firefox/69.0
        Accept: application/json
        Accept-Language: en-US
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        {id: 2345, message: \"Hello\"}";
        result = parse_request(bad_request).is_err();
        assert!(result);
    }
}