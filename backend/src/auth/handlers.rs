use crate::db::DbPool;
use crate::models::user::{NewUser, RegisterUserRequest};
use crate::{auth::service::AuthService, models::user::LoginUserRequest};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use redis::Client;
use uuid::Uuid;

#[post("/register")]
pub async fn register(
    request: web::Json<RegisterUserRequest>,
    db_pool: web::Data<DbPool>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let request = request.into_inner();

    let new_user = NewUser {
        id: Uuid::new_v4(),
        first_name: request.first_name,
        last_name: request.last_name,
        email: request.email,
        password_hash: request.password_hash,
        api_key: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let auth_service = AuthService::new(db_pool.get_ref().clone(), redis_client.get_ref().clone());

    match auth_service.register(new_user) {
        Ok(response) => response,
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "message": e,
        })),
    }
}

#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginUserRequest>,
    db_pool: web::Data<DbPool>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let (email, password) = (credentials.email.clone(), credentials.password.clone());

    let auth_service = AuthService::new(db_pool.get_ref().clone(), redis_client.get_ref().clone());

    match auth_service.login(email, password) {
        Ok(response) => response,
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "message": e,
        })),
    }
}

#[get("/refresh")]
pub async fn refresh_token(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    // Extract the refresh token from the cookie
    let refresh_token_cookie = req.cookie("refresh_token");
    let refresh_token = match refresh_token_cookie {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Refresh token not found",
            }))
        }
    };

    // Call the AuthService to handle the refresh token
    let auth_service = AuthService::new(db_pool.get_ref().clone(), redis_client.get_ref().clone());

    match auth_service.refresh_jwt(&refresh_token) {
        Ok(response) => response,
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "message": e,
        })),
    }
}

#[get("/hello")]
pub async fn api_hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello from the API",
    }))
}

#[post("/logout")]
pub async fn logout(
    req: actix_web::HttpRequest,
    db_pool: web::Data<DbPool>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    // Extract the refresh token from the cookie
    let refresh_token_cookie = req.cookie("refresh_token");
    let token = match refresh_token_cookie {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Refresh token not found",
            }))
        }
    };

    // Call the AuthService to handle logout
    let auth_service = AuthService::new(db_pool.get_ref().clone(), redis_client.get_ref().clone());

    match auth_service.logout(&token) {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": e,
        })),
    }
}
