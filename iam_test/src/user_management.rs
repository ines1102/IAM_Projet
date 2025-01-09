use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
    pub enabled: bool,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub credentials: Vec<Credential>,
}

#[derive(Serialize, Debug)]
pub struct Credential {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
    pub temporary: bool,
}

pub async fn create_user(base_url: &str, token: &str, user: User) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    // Prépare la requête POST pour créer un utilisateur
    let response = client
        .post(&format!("{}/admin/realms/myrealm/users", base_url))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&user)
        .send()
        .await?;

    // Vérifie si la requête a réussi
    if response.status().is_success() {
        println!("User created successfully");
        Ok(())
    } else {
        // Récupère le texte de la réponse en cas d'échec
        let error_text = response.text().await?;
        Err(format!("Failed to create user: {}", error_text).into())
    }
}