use crate::app_state::AppState;
use crate::storage::model::DBUser;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFilteredSchema {
    pub id: i32,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub university: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub university: String,
}

/// GET /api/users/
/// Authorized Endpoint -> JWT Required
/// // TODO: generalize error message return (look at auth for this)
pub async fn user_get_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = match state.relational_storage.get_user_by_id(&user.id).await {
        Err(e) => {
            // SQL error, failed to execute SQL SELECT
            error!("Failed on SQL fetch: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to fetch user details to a server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
        Ok(None) => {
            // User does not exist with email
            info!("User does not exist with id: {}", user.id);
            let error_response = json!({
                "message": "No user found",
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Ok(Some(user)) => user,
    };

    // TODO: decide on a better way to handle None type values for User
    let filtered_user = UserFilteredSchema {
        id: query_result.id,
        student_id: query_result
            .student_id
            .unwrap_or_else(|| String::from("null")),
        first_name: query_result
            .first_name
            .unwrap_or_else(|| String::from("null")),
        last_name: query_result
            .last_name
            .unwrap_or_else(|| String::from("null")),
        email: query_result.email,
        university: query_result
            .university
            .unwrap_or_else(|| String::from("null")),
    };

    let json_response = json!({
        "user": filtered_user
    });

    Ok(Json(json_response))
}

/// PUT /api/users/
pub async fn user_update_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
    Json(payload): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match state
        .relational_storage
        .update_user(
            &user.id,
            &payload.student_id,
            &payload.first_name,
            &payload.last_name,
            &payload.university,
        )
        .await
    {
        Err(e) => {
            // SQL error, failed to execute SQL SELECT
            error!("Failed on SQL fetch: {}", e.to_string());
            let error_response = json!({
                "message": "Unable to update user due to a server error",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
        Ok(_) => {
            // User does not exist with email
            let json_response = json!({
                "message": "Successfully updated user"
            });
            Ok((StatusCode::CREATED, Json(json_response)))
        }
    }
}
