// use actix_web::{HttpResponse, ResponseError};
// use serde_json::json;

// #[derive(Debug)]
// pub enum AuthError {
//     InvalidCredentials,
//     TokenCreation,
//     TokenValidation,
//     RedisError(String),
//     DatabaseError(String),
// }

// impl std::fmt::Display for AuthError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
//             AuthError::TokenCreation => write!(f, "Failed to create token"),
//             AuthError::TokenValidation => write!(f, "Invalid token"),
//             AuthError::RedisError(e) => write!(f, "Redis error: {}", e),
//             AuthError::DatabaseError(e) => write!(f, "Database error: {}", e),
//         }
//     }
// }

// impl ResponseError for AuthError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             AuthError::InvalidCredentials => {
//                 HttpResponse::Unauthorized().json(json!({ "message": "Invalid credentials" }))
//             }
//             AuthError::TokenCreation => HttpResponse::InternalServerError()
//                 .json(json!({ "message": "Failed to create token" })),
//             AuthError::TokenValidation => {
//                 HttpResponse::Unauthorized().json(json!({ "message": "Invalid token" }))
//             }
//             AuthError::RedisError(e) => {
//                 HttpResponse::InternalServerError().json(json!({ "message": e }))
//             }
//             AuthError::DatabaseError(e) => {
//                 HttpResponse::InternalServerError().json(json!({ "message": e }))
//             }
//         }
//     }
// }
