use std::{
    io::{Read, Write, BufRead},
    path::PathBuf,
    net::TcpStream,
};

pub struct Request {
    method: String,
    path: String,
    user_agent: String,
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(
        method: String,
        path: String,
        user_agent: String,
        version: String,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
    ) -> Self {
        Request {
            method,
            path,
            user_agent,
            version,
            headers,
            body,
        }
    }

    pub fn from_stream(_stream: &mut std::net::TcpStream) -> Self {
        let mut buffer = [0; 1024];
        _stream.read(&mut buffer).unwrap();

        let request: String = String::from_utf8(buffer.to_vec()).unwrap();
        println!("Request: {:?}", request);

        // Creating a new Cursor wrapped around the request byte slice
        let mut reader = std::io::Cursor::new(request);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("Request line: {:?}", line);

        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let mut headers = Vec::new();

        loop {
            let mut line = String:: new();
            reader.read_line(&mut line).unwrap();
            println!("Header line: {:?}", line);

            if line == "\r\n" {
                break;
            }

            let line_parts: Vec<&str> = line.split(": ").collect();
            headers.push(( line_parts[0].to_string(), line_parts[1].trim().to_string() ));
        }
        println!("Headers: {:?}", headers);

        let content_length = match headers.iter().find(|(k, _)| k.to_lowercase() == "content-length") {
            Some((_, v)) => v.parse::<usize>().unwrap(),
            None => 0,
        };

        let mut body = vec![0; content_length];
        reader.read_exact(body.as_mut_slice()).unwrap();

        let user_agent = match headers.iter().find(|(k, _)| k.to_lowercase() == "user-agent") {
            Some((_, v)) => v.to_string(),
            None => "".to_string(),
        };
        

        Request::new(
            line_parts[0].to_string(),
            line_parts[1].to_string(),
            user_agent,
            line_parts[2].to_string(),
            headers,
            body,
        )
    }
}

pub fn handle_request(request: Request, mut _stream: std::net::TcpStream, dir: PathBuf) {
    println!("Connection established!");

    match request.method.as_str() {
        "GET" => handle_get(request, &mut _stream, dir),
        "POST" => handle_post(request, &mut _stream, dir),
        _ => {
            _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
            _stream.flush().unwrap();
        }
    }
}

fn handle_get(request: Request, _stream: &mut TcpStream, dir: PathBuf) {
    match request.path.as_str() {
        "/" => {
            println!("Root path requested");
            _ = _stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
        }
        "/user-agent" => {
            println!("User-Agent requested");
            _stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
            _stream.write(b"Content-Type: text/plain\r\n").unwrap();
            _stream
                .write(
                    format!(
                        "Content-Length: {}\r\n",
                        request.user_agent.as_bytes().len()
                    )
                    .as_bytes(),
                )
                .unwrap();
            _stream.write(b"\r\n").unwrap();
            _stream.write(request.user_agent.as_bytes()).unwrap();
            _stream.flush().unwrap();
        }
        _ if request.path.starts_with("/echo/") => {
            let path = request.path;
            println!("Echoing: {}", path);
            // Experimental API, not recommended for production
            let (_, data) = path.split_once("/echo/").unwrap();
            println!("Data: {}", data);
            let content_length = format!("Content-Length: {}\r\n", data.len());
            let response = vec![
                "HTTP/1.1 200 OK",
                "Content-Type: text/plain",
                &content_length,
                "",
            ];
            _ = _stream.write_all(response.join("\r\n").as_bytes());
            _ = _stream.write_all(data.as_bytes());
        }
        _ if request.path.starts_with("/files/") => { // If file exists in the directory, respond with 200OK, otherwise 404 Not Found
            let path = request.path;
            let file = path.replace("/files/", "").trim().to_string();
            println!("File requested: {}", file);
            let file_path = dir.join(file);
            println!("File path: {:?}", file_path);

            if file_path.exists() && file_path.is_file() {
                println!("File exists");
                let content = std::fs::read(file_path).unwrap();

                _stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
                _stream
                    .write(b"Content-Type: application/octet-stream\r\n")
                    .unwrap();
                _stream
                    .write(format!("Content-Length: {}\r\n", content.len()).as_bytes())
                    .unwrap();
                _stream.write(b"\r\n").unwrap();
                _stream.write(&content).unwrap();
                _stream.flush().unwrap();

            } else {
                println!("File not found");
                _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                _stream.flush().unwrap();
            }
        }
        _ => {
            let path = request.path;
            println!("404 Not Found: {}", path);
            _ = _stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
        }
    }
}

fn handle_post(request: Request, _stream: &mut TcpStream, dir: PathBuf) {
    println!("POST request received");

    if request.path.starts_with("/files") {
        let file = request.path.replace("/files/", "").trim().to_string();
        println!("File requested: {}", file);
        let file_path = dir.join(file);
        println!("File path: {:?}", file_path);

        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(&request.body).unwrap();
        _stream.write(b"HTTP/1.1 201 Created\r\n\r\n").unwrap();
    } else {
        _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
        _stream.flush().unwrap();
    }
}