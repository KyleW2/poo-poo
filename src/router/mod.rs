use super::response::{Response, Status, Body};
use super::request::Request;

pub struct Route {
    pub route: String,
    pub function: fn(&mut Request) -> Response,
}

pub struct Router {
    table: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        return Router { table: Vec::new() }
    }

    pub fn add(&mut self, route: String, function: fn(&mut Request) -> Response) {
        self.table.push(Route { route: route, function: function});
    }

    pub fn respond(&self, request: &mut Request) -> Response {
        if request.uri.route_split.len() <= 0 {
            return invalid_route();
        }

        for i in 0..self.table.len() {
            if self.table[i].route == request.uri.route_split[0] {
                request.uri.next();

                return (self.table[i].function)(request)
            }
        }

        return invalid_route()
    }
}

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