use std::net::TcpListener;

use web_spooder::request::Request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let request = Request::new(stream.unwrap());

        println!("{}", request.method);
        println!("{}", request.route);
        println!("{}", request.content);
    }
}
