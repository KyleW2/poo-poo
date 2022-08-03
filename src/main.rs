use std::net::TcpListener;
use std::io::prelude::*;

use web_spooder::request::Request;
use web_spooder::logger::Logger;
use web_spooder::router::Router;
use web_spooder::response::{Response, Status, Body};

fn api(request: &mut Request) -> Response {
    fn root(_request: &mut Request) -> Response {
        return Response {
            status: Status {
                version: "HTTP/1.1".to_string(),
                code: 200,
                text: "Ok".to_string(),
            },
            body: Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the API!</p>".to_string(),
            },
        }
    }

    fn poo(_request: &mut Request) -> Response {
        return Response {
            status: Status {
                version: "HTTP/1.1".to_string(),
                code: 200,
                text: "Ok".to_string(),
            },
            body: Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the Poo!</p>".to_string(),
            },
        }
    }

    fn pee(_request: &mut Request) -> Response {
        return Response {
            status: Status {
                version: "HTTP/1.1".to_string(),
                code: 200,
                text: "Ok".to_string(),
            },
            body: Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the Pee!</p>".to_string(),
            },
        }
    }

    let mut router: Router = Router::new();
    router.add("poo".to_string(), poo);
    router.add("pee".to_string(), pee);
    router.add("".to_string(), root);

    return router.respond(request)

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let logger: Logger = Logger::new("logs/".to_string());

    let mut router: Router = Router::new();
    router.add("api".to_string(), api);

    // Create router

    for stream in listener.incoming() {
        // Unwrap stream
        let mut stream = stream.unwrap();

        // Create request from stream
        let mut request = Request::new(&stream);
        request.uri.next();

        // Log it
        logger.log(&request);

        // Route via router
        let response = router.respond(&mut request);

        println!("{} {} {} {} {}", request.ip, request.uri.method, request.uri.route, response.status.code, response.status.text);

        // Send response
        stream.write(response.display().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}