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

pub struct URL {
    url: Vec<String>,
    pointer: usize,
}

impl URL {
    pub fn new(url: Vec<String>) -> Self {
        return Self { url: url, pointer: 0 }
    }
    pub fn len(&self) -> usize {
        return self.url.len()
    }

    pub fn next(&mut self) -> bool {
        self.pointer += 1;

        if self.pointer <= self.url.len() - 1 {
            return true;
        }

        return false;
    }

    pub fn has_next(&self) -> bool {
        if self.pointer <= self.url.len() - 1 {
            return true;
        }

        return false;
    }

    pub fn get_current(&self) -> &String {
        return &self.url[self.pointer];
    }
}

impl std::ops::Index<usize> for URL {
    type Output = String;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.url[i];
    }
}

impl std::fmt::Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.url)
    }
}

pub struct Line {
    pub method: Method,
    pub url: URL,
    pub protocol: String,
}

impl Line {
    pub fn from_string(s: String) -> Self {
        let method: Method;
        let url: URL;
        let protocol: String;

        let mut split: Vec<&str> = s.split(" ").collect();

        println!("{:?}", split);

        // Match request method
        method = match split.remove(0) {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => Method::INVALID,
        };

        let temp_url_string = split.remove(0);
        let mut url_vec = Vec::new();

        let temp_url_split: Vec<&str> = temp_url_string.split("/").collect();

        for i in 0..temp_url_split.len() {
            if temp_url_split[i] != "" && temp_url_split[i] != "" {
                url_vec.push(temp_url_split[i].to_string())
            }
        }

        url = URL::new(url_vec);

        protocol = split.remove(0).to_string();

        return Line { method: method, url: url, protocol: protocol }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method: {}\nRoute: {}\nProtocol: {}", self.method, self.url, self.protocol)
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
    pub line: Line,
    pub header: Header,
    pub body: Body,
}

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        // Read into buffer
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // Block empty requests
        if is_zero(&buffer) {
            return Self {
                ip: stream.peer_addr().unwrap(),
                line: Line { method: Method::INVALID, url: URL::new(vec![]), protocol: "".to_string() },
                header: Header { host: "".to_string(), agent: "".to_string(), accept: "".to_string() },
                body: Body { content_type: "".to_string(), content_length: 0, content: "".to_string() }
            }
        }

        // Create String
        let stream_as_string = String::from_utf8_lossy(&buffer[..]);

        // Make vars
        let mut host: String = String::new();
        let mut agent: String = String::new();
        let mut accept: String = String::new();
        let mut content_type: String = String::new();
        let mut content_length: i32 = -1;
        let mut content: String = String::new();

        

        // Split String
        let mut v: Vec<&str> = stream_as_string.split("\n").collect();

        // Parse the request line
        let current_string = v.remove(0).to_string();
        
        let line = Line::from_string(current_string);

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
            line: line,
            header: Header { host: host, agent: agent, accept: accept },
            body: Body { content_type: content_type, content_length: content_length, content: content },
        }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Request Line:\n{}\n\nHeader:\n{}\n\nBody:\n{}\n", self.line, self.header, self.body)
    }
}

fn is_zero(buf: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}