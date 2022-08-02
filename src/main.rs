use std::net::TcpListener;
use std::io::prelude::*;

use web_spooder::request::Request;
use web_spooder::logger::Logger;
use web_spooder::routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let logger: Logger = Logger::new("logs/".to_string());

    for stream in listener.incoming() {
        // Unwrap stream
        let mut stream = stream.unwrap();

        // Create request from stream
        let request = Request::new(&stream);

        // Log it
        logger.log(&request);

        // Route
        let response = match request.uri.route {
            _ => routes::invalid_route(),
        };

        // Send response
        stream.write(response.display().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}