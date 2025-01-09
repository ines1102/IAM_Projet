// src/server.rs
use std::net::TcpListener;
use std::io::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    println!("Server listening on port 8000...");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get_callback = b"GET /auth/callback";
    let get_logout = b"GET /logout";
    let get_login = b"GET /login";

    if buffer.starts_with(get_callback) {
        let contents = fs::read_to_string("response.html")?;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes())?;
        stream.flush()?;
    } else if buffer.starts_with(get_logout) {
        let response = format!(
            "HTTP/1.1 302 Found\r\nLocation: http://localhost:8080/realms/myrealm/protocol/openid-connect/logout?redirect_uri=http://localhost:8000/login\r\n\r\n"
        );

        stream.write(response.as_bytes())?;
        stream.flush()?;
    } else if buffer.starts_with(get_login) {
        let response = format!(
            "HTTP/1.1 302 Found\r\nLocation: http://localhost:8080/realms/myrealm/protocol/openid-connect/auth?response_type=code&client_id=myclient&redirect_uri=http://localhost:8000/auth/callback&scope=openid\r\n\r\n"
        );

        stream.write(response.as_bytes())?;
        stream.flush()?;
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = "404 - Not Found";
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line, contents.len(), contents
        );

        stream.write(response.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}