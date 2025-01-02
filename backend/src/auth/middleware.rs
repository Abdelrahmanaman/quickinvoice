use crate::auth::jwt::dec_jwt;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::error;
use serde_json::json;
pub async fn validator_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    // Decode and validate the JWT using your existing `decode_jwt` function
    match dec_jwt(token) {
        Ok(claims) => {
            // Attach the user ID to the request for use in handlers
            req.extensions_mut().insert(claims.sub);
            Ok(req)
        }
        Err(e) => {
            error!("Failed to decode token: {}", e); // Log the error

            // Return a structured JSON error response
            let error_message = match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token has expired",
                jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                _ => "Unauthorized",
            };

            // Construct the JSON error response
            let json_error = json!({
                "message": error_message,
                "error": e.to_string(),
            });

            // Return both the error and the original request
            Err((
                actix_web::error::ErrorUnauthorized(json_error.to_string()),
                req,
            ))
        }
    }
}
