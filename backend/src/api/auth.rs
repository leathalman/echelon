use crate::api::jwt::TokenClaims;
use crate::app_state::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::{Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub email: String,
    pub password: String,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

// POST /api/auth/sign-up
pub async fn auth_signup_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email_result = state
        .relational_storage
        .get_user_by_email(&payload.email)
        .await;

    match email_result {
        Ok(user) => {
            if user.is_some() {
                info!("User already exists with email: {}", payload.email);
                let error_response = json!({
                    "message": "User already exists with email",
                });
                Err((StatusCode::CONFLICT, Json(error_response)))
            } else {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();

                let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
                    Ok(hash) => hash.to_string(),
                    Err(e) => {
                        // failed to hash password
                        error!("{}", e.to_string());
                        let error_response = json!({
                            "message": "Failed to create user due to a server error",
                        });
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                    }
                };

                // make new user
                let query_result = state
                    .relational_storage
                    .create_user(
                        &payload.student_id,
                        &payload.email,
                        &password_hash,
                        &payload.first_name,
                        &payload.last_name,
                    )
                    .await;

                match query_result {
                    Ok(user) => {
                        let user_response = json!({
                            "user_id": user.id
                        });

                        Ok((StatusCode::CREATED, Json(user_response)))
                    }
                    Err(e) => {
                        // SQL error, failed to execute SQL SELECT
                        error!("{}", e.to_string());
                        let error_response = json!({
                            "message": "Failed to create user due to a server error",
                        });

                        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                    }
                }
            }
        }
        Err(e) => {
            // SQL error, failed to execute SQL SELECT
            error!("{}", e.to_string());
            let error_response = json!({
                "message": "Failed to create user due to a server error",
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

// POST /api/auth/login
pub async fn auth_login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check if user exists with email
    let user = match state
        .relational_storage
        .get_user_by_email(&payload.email)
        .await
    {
        Err(e) => {
            // SQL error, failed to execute SQL SELECT
            error!("Failed on SQL fetch: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to login due to a server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
        Ok(None) => {
            // User does not exist with email
            info!("User does not exist with email: {}", payload.email);
            let error_response = json!({
                "message": "Incorrect email or password",
            });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
        Ok(Some(user)) => user,
    };

    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Err(e) => {
            error!("Failed on password hash: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to login due to a server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
        Ok(parsed_hash) => parsed_hash,
    };

    // Verify password
    let password_correct = Argon2::default()
        .verify_password(&payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !password_correct {
        info!("Incorrect password");
        let error_response = json!({
            "message": "Incorrect email or password",
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let issued_at = now.timestamp() as usize;
    // TODO: does not use config string BTW
    // TODO: increase to 1 week for prod
    let expire_at = (now + chrono::Duration::days(1)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        iat: issued_at,
        exp: expire_at,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(e) => {
            error!("{}", e.to_string());
            let error_response = json!({
            "message": "Login failed due to server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let cookie = Cookie::build(("authToken", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(24))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"authToken": token}).to_string());

    match cookie.to_string().parse::<HeaderValue>() {
        Ok(cookie_header_value) => {
            response
                .headers_mut()
                .insert(header::SET_COOKIE, cookie_header_value);
            Ok(response)
        }
        Err(e) => {
            error!("Failed to parse cookie to header value: {}", e);
            let error_response = json!({
                "message": "Login failed due to server error",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

// GET /api/auth/logout
pub async fn auth_logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // make new cookie to invalid current cookie in the browser
    let cookie = Cookie::build(("authToken", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}


// POST /api/auth/reset-password
