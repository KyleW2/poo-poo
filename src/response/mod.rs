pub struct Status {
    pub version: String,
    pub code: i32,
    pub text: String,
}

pub fn status(code: i32) -> Status {
    match code {
        200 => Status{ version: "HTTP/1.1".to_string(), code: code, text: "OK".to_string() },
        201 => Status{ version: "HTTP/1.1".to_string(), code: code, text: "Created".to_string() },
        400 => Status{ version: "HTTP/1.1".to_string(), code: code, text: "Bad Request".to_string() },
        401 => Status{ version: "HTTP/1.1".to_string(), code: code, text: "Unauthorized".to_string() },
        404 => Status{ version: "HTTP/1.1".to_string(), code: code, text: "Not Found".to_string() },
        _ => Status{ version: "HTTP/1.1".to_string(), code: 404, text: "Not Found".to_string() }
    }
}

impl Status {
    pub fn display(&self) -> String {
        let mut display: String = String::new();

        display.push_str(&self.version);
        display.push_str(" ");
        display.push_str(&self.code.to_string());
        display.push_str(" ");
        display.push_str(&self.text);

        return display
    }
}

pub struct Body {
    pub content_type: String,
    pub content: String,
}

impl Body {
    pub fn display(&self) -> String {
        let mut display: String = String::new();

        display.push_str("Content-Type: ");
        display.push_str(&self.content_type);
        display.push_str("\n");
        display.push_str(&self.content);

        return display
    }
}

pub struct Response {
    pub status: Status,
    pub body: Body,
}

impl Response {
    pub fn display(&self) -> String {
        let mut display: String = String::new();
        display.push_str(&self.status.display());
        display.push_str("\n");
        display.push_str(&self.body.display());

        return display 
    }
}