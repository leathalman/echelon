// POST /api/auth/login

// POST /api/auth/reset-password

use crate::app_state::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::error;

// do we want to add naming and such here?
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub email: String,
    pub password: String,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
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
                error!("User already exists with email: {}", payload.email);
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
            error!("{}", e.to_string());
            let error_response = json!({
                "message": "Failed to create user due to a server error",
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }

    // issue JWT

    // generate OTP

    // send email

    // create password hash

    // make new user
}
