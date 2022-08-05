use super::response::{Response, status, Body};
use super::request::{Request, URL};

pub struct Route {
    pub url: URL,
    pub function: fn(&mut Request) -> Response,
}

pub struct Router {
    table: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        return Router { table: Vec::new() }
    }

    pub fn add(&mut self, url: URL, function: fn(&mut Request) -> Response) {
        self.table.push(Route { url: url, function: function});
    }

    pub fn respond(&self, request: &mut Request) -> Response {
        if !request.line.url.has_next() {
            return invalid_route();
        }

        for i in 0..self.table.len() {
            if self.table[i].url.get_current() == request.line.url.get_current() {
                request.line.url.next();

                return (self.table[i].function)(request)
            }
        }

        return invalid_route()
    }
}

pub fn invalid_route() -> Response {
    return Response {
        status: status(404),
        body: Body {
            content_type: "HTML\n".to_string(),
            content: "<h1>404 Not found</h1><p>Bad gateway.<p>".to_string(),
        },
    }
}