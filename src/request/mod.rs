use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};

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

pub struct URI {
    pub method: Method,
    pub route: String,
    pub route_split: Vec<String>,
    pub protocol: String,
}

impl URI {
    pub fn next(&mut self) {
        self.route_split.remove(0);
    }
}

impl std::fmt::Display for URI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method: {}\nRoute: {}\nProtocol: {}", self.method, self.route, self.protocol)
    }
}

pub struct Header {
    pub host: String,
    pub agent: String,
    pub accept: String,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Host: {}\nAgent: {}\nAccept: {}", self.host, self.agent, self.accept)
    }
}

pub struct Body {
    pub content_type: String,
    pub content_length: i32,
    pub content: String,
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Content-Type: {}\nContent-Length: {},\nContent:\n{}", self.content_type, self.content_length, self.content)
    }
}

pub struct Request {
    pub ip: SocketAddr,
    pub uri: URI,
    pub header: Header,
    pub body: Body,
}

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        // Make vars
        let method: Method;
        let route: String;
        let mut route_split: Vec<String> = Vec::new();
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

        let mut temp_route_split: Vec<&str> = route.split("/").collect();
        temp_route_split.push("/");

        for i in 0..temp_route_split.len() {
            route_split.push(temp_route_split[i].to_string())
        }

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
            ip: stream.peer_addr().unwrap(),
            uri: URI { method: method, route: route, route_split: route_split, protocol: protocol },
            header: Header { host: host, agent: agent, accept: accept },
            body: Body { content_type: content_type, content_length: content_length, content: content },
        }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "URI:\n{}\n\nHeader:\n{}\n\nBody:\n{}\n", self.uri, self.header, self.body)
    }
}