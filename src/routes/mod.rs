use super::response::{Response, Status, Body};

pub fn invalid_route() -> Response {
    return Response {
        status: Status {
            version: "HTTP/1.1".to_string(),
            code: 404,
            text: "Bad gateway".to_string(),
        },
        body: Body {
            content_type: "HTML\n".to_string(),
            content: "<h1>404 Not found</h1><p>Bad gateway.<p>".to_string(),
        },
    }
}