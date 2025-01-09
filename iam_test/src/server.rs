use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    println!("Server listening on port 8000...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut socket).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(socket: &mut tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..]);

    let (status_line, filename) = if request.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", "index.html")
    } else if request.starts_with("GET /create-user ") {
        ("HTTP/1.1 200 OK", "create_user.html")
    } else if request.starts_with("GET /list-users ") {
        ("HTTP/1.1 200 OK", "list_users.html")
    } else if request.starts_with("GET /update-user/") {
        ("HTTP/1.1 200 OK", "update_user.html")
    } else if request.starts_with("GET /delete-user/") {
        ("HTTP/1.1 200 OK", "delete_user.html")
    } else if request.starts_with("POST /create-user ") {
        let body = get_request_body(&request);
        let fields: Vec<&str> = body.split('&').collect();
        let username = fields.iter().find(|&&s| s.starts_with("username=")).unwrap().split('=').nth(1).unwrap();
        let email = fields.iter().find(|&&s| s.starts_with("email=")).unwrap().split('=').nth(1).unwrap();
        let first_name = fields.iter().find(|&&s| s.starts_with("first_name=")).unwrap().split('=').nth(1).unwrap();
        let last_name = fields.iter().find(|&&s| s.starts_with("last_name=")).unwrap().split('=').nth(1).unwrap();
        let password = fields.iter().find(|&&s| s.starts_with("password=")).unwrap().split('=').nth(1).unwrap();

        // Obtenir le jeton d'accès et créer l'utilisateur
        let token = get_token("myclient", "GLBtXaKgIhbmvyScc2RjpCYruJUbOptE", "etudiant", "vitrygtr").await?;
        create_user(&token, username, email, first_name, last_name, password).await?;

        ("HTTP/1.1 302 Found", "list_users.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = tokio::fs::read_to_string(filename).await?;
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    socket.write_all(response.as_bytes()).await?;
    socket.flush().await?;
    Ok(())
}

fn get_request_body(request: &str) -> &str {
    request.split("\r\n\r\n").nth(1).unwrap_or("")
}

async fn get_token(client_id: &str, client_secret: &str, username: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = "http://localhost:8080/realms/myrealm/protocol/openid-connect/token";

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "password"),
        ("username", username),
        ("password", password),
    ];

    let response = client.post(url)
        .form(&params)
        .send()
        .await?;

    let json_response: serde_json::Value = response.json().await?;
    let token = json_response["access_token"].as_str().ok_or("Failed to get access token")?.to_string();

    Ok(token)
}

async fn create_user(token: &str, username: &str, email: &str, first_name: &str, last_name: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "http://localhost:8080/admin/realms/myrealm/users";

    let user_data = json!({
        "username": username,
        "email": email,
        "firstName": first_name,
        "lastName": last_name,
        "enabled": true,
        "credentials": [{
            "type": "password",
            "value": password,
            "temporary": false
        }]
    });

    let response = client.post(url)
        .bearer_auth(token)
        .json(&user_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("User created successfully");
    } else {
        println!("Failed to create user: {}", response.text().await?);
    }

    Ok(())
}