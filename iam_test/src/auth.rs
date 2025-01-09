use openidconnect::core::{
    CoreClient, CoreProviderMetadata, CoreResponseType,
};
use openidconnect::{
    AuthenticationFlow, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
    error_description: String,
}

pub async fn authenticate() -> Result<(), Box<dyn Error>> {
    // Configure Keycloak details
    let issuer_url = IssuerUrl::new("http://localhost:8080/realms/myrealm".to_string())?;
    let client_id = ClientId::new("myclient".to_string());
    let client_secret = ClientSecret::new("my-client-secret".to_string());
    let redirect_url = RedirectUrl::new("http://localhost:8000/auth/callback".to_string())?;

    // Fetch the OpenID Connect provider metadata
    let provider_metadata: CoreProviderMetadata = {
        let url = format!("{}/.well-known/openid-configuration", issuer_url.as_str());
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        serde_json::from_str(&text)?
    };

    // Create an OpenID Connect client
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        client_id,
        Some(client_secret),
    )
    .set_redirect_uri(redirect_url);

    // Generate the authorization URL
    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .url();

    println!("Access this URL to authenticate: {}", auth_url);
    println!("CSRF token: {}", csrf_token.secret());
    println!("Nonce: {}", nonce.secret());

    // Provide a logout URL
    let logout_url = format!("http://localhost:8080/realms/myrealm/protocol/openid-connect/logout?redirect_uri=http://localhost:8000");
    println!("Access this URL to logout: {}", logout_url);

    Ok(())
}

pub async fn get_admin_token() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let params = [
        ("client_id", "myclient"),
        ("client_secret", "GLBtXaKgIhbmvyScc2RjpCYruJUbOptE"), // Assurez-vous que ce client_secret est correct
        ("username", "etudiant"), // Utilisez le compte 'etudiant'
        ("password", "vitrygtr"), // Mot de passe pour 'etudiant'
        ("grant_type", "password"),
    ];

    println!("Params: {:?}", params); // Affiche les paramètres pour le débogage

    let res = client
        .post("http://localhost:8080/realms/myrealm/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await?;

    println!("Status: {}", res.status()); // Affiche le statut de la réponse pour le débogage

    if res.status().is_success() {
        let token_response: TokenResponse = res.json().await?;
        Ok(token_response.access_token)
    } else {
        let error_response: ErrorResponse = res.json().await?;
        Err(format!("Failed to get token: {} - {}", error_response.error, error_response.error_description).into())
    }
}