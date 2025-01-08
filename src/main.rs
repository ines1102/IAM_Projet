use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web_httpauth::middleware::HttpAuthentication;
use reqwest;

async fn index(id: Identity) -> impl Responder {
    if let Some(user_id) = id.identity() {
        HttpResponse::Ok().body(format!("Hello, {}!", user_id))
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false), // En production, utilisez `true`
            ))
            .wrap(HttpAuthentication::bearer(authenticate))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn authenticate(req: actix_web::dev::ServiceRequest, credentials: actix_web_httpauth::extractors::bearer::BearerAuth) -> Result<actix_web::dev::ServiceRequest, actix_web::Error> {
    let token = credentials.token();
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8080/auth/realms/example/protocol/openid-connect/userinfo")
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        Ok(req)
    } else {
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}