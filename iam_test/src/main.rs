mod auth;
mod user_management;

use user_management::{create_user, User, Credential};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Authentification
    auth::authenticate().await?;

    // Gestion des utilisateurs
    let base_url = "http://localhost:8080";
    let token = auth::get_admin_token().await?;

    let new_user = User {
        username: "etudiant".to_string(),
        enabled: true,
        email: "etudiant@its.com".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
        credentials: vec![Credential {
            type_: "password".to_string(),
            value: "vitrygtr".to_string(),
            temporary: false,
        }],
    };

    create_user(base_url, &token, new_user).await?;

    Ok(())
}