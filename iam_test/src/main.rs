// src/main.rs
use openidconnect::core::{
    CoreClient, CoreProviderMetadata, CoreResponseType,
};
use openidconnect::{
    AuthenticationFlow, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
};
use reqwest::blocking::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Configure Keycloak details
    let issuer_url = IssuerUrl::new("http://localhost:8080/realms/myrealm".to_string())?;
    let client_id = ClientId::new("myclient".to_string());
    let client_secret = ClientSecret::new("my-client-secret".to_string());
    let redirect_url = RedirectUrl::new("http://localhost:8000/auth/callback".to_string())?;

    // Fetch the OpenID Connect provider metadata
    let provider_metadata: CoreProviderMetadata = {
        let url = issuer_url.as_str().to_string() + "/.well-known/openid-configuration";
        let response = Client::new().get(&url).send()?;
        let text = response.text()?;
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