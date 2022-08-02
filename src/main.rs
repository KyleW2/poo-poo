use std::net::TcpListener;

use web_spooder::request::Request;
use web_spooder::logger::Logger;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let logger: Logger = Logger::new("logs/".to_string());

    for stream in listener.incoming() {
        let request = Request::new(stream.unwrap());
        logger.log(&request);

        /* TODO: Implement something like this */
        // log(request);
        // route(request);
    }
}
