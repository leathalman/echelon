use crate::app_state::AppState;
use crate::storage::model::DBUser;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
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

    // issue JWT

    // generate OTP

    // send email

    // create password hash

    // make new user
}

// POST /api/auth/login
pub async fn auth_login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // check that user exists with email
    let email_result = state
        .relational_storage
        .get_user_by_email(&payload.email)
        .await;

    match email_result {
        Ok(response) => {
            if let Some(user) = response {
                // check password
                let parsed_hash = PasswordHash::new(&user.password_hash);

                match parsed_hash {
                    Ok(parsed_hash) => {
                        // see if password hash is correct
                        let password_correct = Argon2::default()
                            .verify_password(&payload.password.as_bytes(), &parsed_hash)
                            .is_ok();

                        if password_correct {
                            let user_response = json!({
                                "message": "Login successful"
                            });

                            Ok((StatusCode::OK, Json(user_response)))
                        } else {
                            info!("Incorrect password");
                            let error_response = json!({
                                "message": "Incorrect email or password",
                            });
                            Err((StatusCode::UNAUTHORIZED, Json(error_response)))
                        }
                    }
                    Err(e) => {
                        // unable to parse hash
                        error!("{}", e.to_string());
                        let error_response = json!({
                            "message": "Unable to login due to a server error",
                        });
                        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                    }
                }
            } else {
                // user does not exist with email
                info!("User does not exist with email: {}", payload.email);
                let error_response = json!({
                    "message": "Incorrect email or password",
                });
                Err((StatusCode::UNAUTHORIZED, Json(error_response)))
            }
        }
        Err(e) => {
            // SQL error, failed to execute SQL SELECT
            error!("{}", e.to_string());
            let error_response = json!({
                "message": "Unable to login due to a server error",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }

    // check that password is correct

    // issue status code only for now
}

// POST /api/auth/reset-password
// issue JWT? does that need to be an endpoint?
