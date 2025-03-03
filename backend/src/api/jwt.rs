use crate::app_state::AppState;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use std::sync::Arc;
use axum::body::Body;
use axum::http::header;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    // subject
    pub sub: String,
    // issued at
    pub iat: usize,
    // expiration time
    pub exp: usize,
}

pub async fn auth(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = cookie_jar
        .get("auth_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_val| {
                    if auth_val.starts_with("Bearer ") {
                        Some(auth_val[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = match token {
        Some(token) => token,
        None => {
            info!("No JWT found, please provide one");
            let error_response = json!({
                "message": "No JWT found, please provide one",
            });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    };

    let claims = match decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data.claims,
        Err(e) => {
            info!("Invalid JWT: {}", e);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid or expired JWT" })),
            ));
        }
    };

    let user_id = match claims.sub.parse::<i32>() {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to parse user ID: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Invalid token format" })),
            ));
        }
    };

    let user = match state.relational_storage.get_user_by_id(&user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            info!("No user found for id: {}", user_id);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "User not found" })),
            ));
        }
        Err(e) => {
            error!("Database error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch user from database" })),
            ));
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}