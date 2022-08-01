use std::io::prelude::*;
use std::net::TcpStream;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    INVALID,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Method::GET => write!(f, "GET"),
            &Method::POST => write!(f, "POST"),
            &Method::PUT => write!(f, "PUT"),
            &Method::DELETE => write!(f, "DELETE"),
            &Method::INVALID => write!(f, "INVALID"),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub route: String,
    pub protocol: String,
    pub host: String,
    pub agent: String,
    pub accept: String,
    pub content_type: String,
    pub content_length: i32,
    pub content: String,
}

impl Request {
    pub fn new(mut stream: TcpStream) -> Self {
        // Make vars
        let method: Method;
        let route: String;
        let protocol: String;
        let mut host: String = String::new();
        let mut agent: String = String::new();
        let mut accept: String = String::new();
        let mut content_type: String = String::new();
        let mut content_length: i32 = -1;
        let mut content: String = String::new();

        // Read into buffer
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // Create String
        let stream_as_string = String::from_utf8_lossy(&buffer[..]);

        // Split String
        let mut v: Vec<&str> = stream_as_string.split("\n").collect();

        // Parse the request line
        let current_string = v.remove(0).to_string();
        let current_split: Vec<&str> = current_string.split(" ").collect();

        // Match request method
        method = match current_split[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => Method::INVALID,
        };

        route = current_split[1].to_string();
        protocol = current_split[2].to_string();

        // Parse rest
        while v.len() > 0 {
            let current_string = v.remove(0).to_string();
            let current_split: Vec<&str> = current_string.split(" ").collect();
            match current_split[0] {
                "Host:" => host = current_split[1].trim().to_string(),
                "Agent:" => agent = current_split[1].trim().to_string(),
                "Accept:" => accept = current_split[1].trim().to_string(),
                "Content-Type:" => content_type = current_split[1].trim().to_string(),
                "Content-Length:" => content_length = current_split[1].trim().to_string().parse::<i32>().unwrap(),
                _ => (),
            }
            if current_string.starts_with("{") {
                content = current_string
            }
        }

        return Self {
            method: method,
            route: route,
            protocol: protocol,
            host: host,
            agent: agent,
            accept: accept,
            content_type: content_type,
            content_length: content_length,
            content: content,
        }
    }
}