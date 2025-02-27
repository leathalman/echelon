use std::sync::Arc;

use crate::app_state::AppState;
use crate::storage::model::DBUser;
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Error;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub subject: String,
    pub issued_at: usize,
    pub expire_at: usize,
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = match cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_val| {
                    if auth_val.starts_with("Bearer ") {
                        Some(auth_val[7..].to_owned())
                    } else {
                        None
                    }
                })
        }) {
        Some(value) => value,
        None => {
            info!("No JWT found, please provide one");
            let error_response = json!({
                "message": "No JWT found, please provide one",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
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
                Json(json!({ "message": "Invalid JWT" })),
            ));
        }
    };

    println!("{:?}", claims);

    // user id is a little different for me?
    // let user_id = match uuid::Uuid::parse_str(&claims.sub) {
    //     Ok(uuid) => uuid,
    //     Err(e) => {
    //         info!("Unable to parse user_id from claims: {}", e);
    //         return Err((
    //             StatusCode::UNAUTHORIZED,
    //             Json(json!({ "message": "Invalid JWT" })),
    //         ));
    //     }
    // };

    let user_id = match &claims.subject.parse::<i32>() {
        Ok(num) => num.clone(),
        Err(e) => {
            error!("{}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch user from database" })),
            ));
        }
    };

    let user = match state.relational_storage.get_user_by_id(&user_id).await {
        Ok(user) => user,
        Err(e) => {
            error!("{}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch user from database" })),
            ));
        }
    };

    if let Some(user) = user {
        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        info!("No user found for id: {}", user_id);
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to fetch user from database" })),
        ))
    }
}
