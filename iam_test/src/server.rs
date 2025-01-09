// src/server.rs
use std::net::TcpListener;
use std::io::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /auth/callback";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("response.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = "404 - Not Found";
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line, contents.len(), contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}