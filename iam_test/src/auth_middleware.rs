use actix_web::{dev::ServiceRequest, Error, HttpResponse};
use actix_web::dev::Transform;
use futures::future::{ok, Ready};
use actix_service::{Service, ServiceFactory};
use std::task::{Context, Poll};
use std::pin::Pin;

// Middleware d'authentification
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Récupère le jeton d'authentification du header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        if let Some(token) = token {
            // Valide le jeton ici
            if token == "valid_token" { // Remplacez par une véritable validation
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            } else {
                // Retourne une réponse 401 Unauthorized si le jeton est invalide
                Box::pin(async move {
                    Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
                })
            }
        } else {
            // Retourne une réponse 401 Unauthorized si le header Authorization est manquant
            Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            })
        }
    }
}