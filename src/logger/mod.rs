use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

use super::request::Request;

pub struct Logger {
    write_path: String
}

impl Logger {
    pub fn new(write_path: String) -> Self {
        return Self { write_path: write_path }
    }

    pub fn log(&self, request: &Request) {
        // Create Path and Display
        let file_name: String = self.write_path.clone() + &SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string() + ".log";
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file =  match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        file.write_all(&request.to_string().as_bytes()).unwrap()
    }
}