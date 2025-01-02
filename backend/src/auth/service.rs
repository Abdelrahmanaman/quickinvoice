use crate::auth::jwt::create_jwt;
use crate::config::{ACCESS_TOKEN_TIME, REFRESH_TOKEN_TIME};
use crate::db::DbPool;
use crate::models::user::{NewUser, User};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::HttpResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use redis::{Commands, RedisResult};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct AuthService {
    pub db_pool: DbPool,
    pub redis_client: redis::Client,
}

impl AuthService {
    pub fn new(db_pool: DbPool, redis_client: redis::Client) -> Self {
        AuthService {
            db_pool,
            redis_client,
        }
    }

    pub fn register(&self, new_user: NewUser) -> Result<HttpResponse, String> {
        use crate::schema::users::dsl::*;
        let mut conn = self.db_pool.get().map_err(|e| e.to_string())?;

        // Check if the email already exists
        let email_exists = users
            .filter(email.eq(&new_user.email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;

        if email_exists.is_some() {
            return Err("Email already exists".to_string());
        }

        // Hash the password
        let hashed_password =
            hash(new_user.password_hash, DEFAULT_COST).map_err(|e| e.to_string())?;

        // Insert the new user
        let user = diesel::insert_into(users)
            .values(&NewUser {
                password_hash: hashed_password,
                ..new_user
            })
            .get_result::<User>(&mut conn)
            .map_err(|e| e.to_string())?;

        // Generate a JWT token
        let jwt = create_jwt(&user.id.to_string()).map_err(|e| e.to_string())?;

        // Generate a refresh token and store it in Redis
        let refresh_token = self
            .create_refresh_token(&user.id.to_string())
            .map_err(|e| e.to_string())?;

        // Create JWT and refresh token cookies
        let jwt_cookie = self.create_jwt_cookie(&jwt);
        let refresh_cookie = self.create_refresh_cookie(&refresh_token);

        Ok(HttpResponse::Created()
            .cookie(jwt_cookie)
            .cookie(refresh_cookie)
            .json(serde_json::json!({
                "message": "User created successfully",
            })))
    }

    pub fn login(&self, user_email: String, password: String) -> Result<HttpResponse, String> {
        use crate::schema::users::dsl::*;
        let mut conn = self.db_pool.get().map_err(|e| e.to_string())?;

        // Find the user by email
        let user = users
            .filter(email.eq(&user_email))
            .first::<User>(&mut conn)
            .map_err(|e| e.to_string())?;

        // Verify the password
        let is_valid = verify(password, &user.password_hash).map_err(|e| e.to_string())?;
        if !is_valid {
            return Err("Invalid credentials".to_string());
        }

        // Generate a JWT token
        let jwt = create_jwt(&user.id.to_string()).map_err(|e| e.to_string())?;

        // Generate a refresh token and store it in Redis
        let refresh_token = self
            .create_refresh_token(&user.id.to_string())
            .map_err(|e| e.to_string())?;

        // Create JWT and refresh token cookies
        let jwt_cookie = self.create_jwt_cookie(&jwt);
        let refresh_cookie = self.create_refresh_cookie(&refresh_token);

        Ok(HttpResponse::Ok()
            .cookie(jwt_cookie)
            .cookie(refresh_cookie)
            .json(serde_json::json!({
                "message": "Logged in successfully",
            })))
    }

    pub fn refresh_jwt(&self, refresh_token: &str) -> Result<HttpResponse, String> {
        // Step 1: Validate the refresh token
        let user_id = self
            .validate_refresh_token(refresh_token)
            .map_err(|e| e.to_string())?;

        // Step 2: Generate a new JWT
        let jwt = create_jwt(&user_id).map_err(|e| e.to_string())?;

        // Create a JWT cookie
        let jwt_cookie = self.create_jwt_cookie(&jwt);

        Ok(HttpResponse::Ok()
            .cookie(jwt_cookie)
            .json(serde_json::json!({
                "message": "Token refreshed successfully",
            })))
    }

    pub fn logout(&self, refresh_token: &str) -> Result<HttpResponse, String> {
        // Invalidate the refresh token in Redis
        self.invalidate_refresh_token(refresh_token)
            .map_err(|e| e.to_string())?;

        // Create expired cookies for JWT and refresh token
        let jwt_cookie = self.create_jwt_cookie("");
        let refresh_cookie = self.create_refresh_cookie("");

        Ok(HttpResponse::Ok()
            .cookie(jwt_cookie)
            .cookie(refresh_cookie)
            .json(serde_json::json!({
                "message": "Logged out successfully",
            })))
    }

    pub fn create_refresh_token(&self, user_id: &str) -> RedisResult<String> {
        let mut conn = self.redis_client.get_connection()?;
        let refresh_token = Uuid::new_v4().to_string();

        // Store the refresh token in Redis with expiration time
        let () = conn.set_ex(
            &refresh_token,
            user_id,
            REFRESH_TOKEN_TIME.num_seconds() as u64,
        )?;
        Ok(refresh_token)
    }

    pub fn validate_refresh_token(&self, refresh_token: &str) -> RedisResult<String> {
        let mut conn = self.redis_client.get_connection()?;
        let user_id: String = conn.get(refresh_token)?;
        Ok(user_id)
    }

    pub fn invalidate_refresh_token(&self, refresh_token: &str) -> RedisResult<()> {
        let mut conn = self.redis_client.get_connection()?;
        let () = conn.del(refresh_token)?;
        Ok(())
    }

    fn create_jwt_cookie(&self, jwt: &str) -> Cookie<'static> {
        // Convert `chrono::DateTime<Utc>` to `time::OffsetDateTime`
        let expires =
            OffsetDateTime::from_unix_timestamp((Utc::now() + ACCESS_TOKEN_TIME).timestamp())
                .expect("Failed to convert timestamp");

        Cookie::build("jwt", jwt.to_owned()) // Use `to_owned()` to create an owned `String`
            .http_only(true)
            .secure(true) // Enable in production (HTTPS only)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(expires) // Use `OffsetDateTime` for expiration
            .finish()
    }

    fn create_refresh_cookie(&self, refresh_token: &str) -> Cookie<'static> {
        // Convert `chrono::DateTime<Utc>` to `time::OffsetDateTime`
        let expires =
            OffsetDateTime::from_unix_timestamp((Utc::now() + REFRESH_TOKEN_TIME).timestamp())
                .expect("Failed to convert timestamp");

        Cookie::build("refresh_token", refresh_token.to_owned()) // Use `to_owned()` to create an owned `String`
            .http_only(true)
            .secure(true) // Enable in production (HTTPS only)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(expires) // Use `OffsetDateTime` for expiration
            .finish()
    }
}
