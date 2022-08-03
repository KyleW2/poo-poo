use std::net::TcpListener;
use std::io::prelude::*;

use super::request::Request;
use super::logger::Logger;
use super::router::Router;

pub struct App {
    listener: TcpListener,
    logger: Logger,
    main_router: Router,
}

impl App {
    pub fn new(main_router: Router) -> Self {
        return App {
            listener: TcpListener::bind("127.0.0.1:6969").unwrap(),
            logger: Logger::new("logs/".to_string()),
            main_router: main_router,
        }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            // Unwrap stream
            let mut stream = stream.unwrap();
    
            // Create request from stream
            let mut request = Request::new(&stream);
            request.uri.next();
    
            // Log it
            self.logger.log(&request);
    
            // Route via router
            let response = self.main_router.respond(&mut request);
    
            println!("{} {} {} {} {}", request.ip, request.uri.method, request.uri.route, response.status.code, response.status.text);
    
            // Send response
            stream.write(response.display().as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}