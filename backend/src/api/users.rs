use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};
use crate::app_state::AppState;
use crate::storage::model::DBUser;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFilteredSchema {
    pub id: i32,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub university: String,
}

/// GET /api/users/
/// Authorized Endpoint -> JWT Required
/// // TODO: generalize error message return (look at auth for this)
pub async fn user_get_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<DBUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = match state
        .relational_storage
        .get_user_by_id(&user.id)
        .await {
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

    let filtered_user = UserFilteredSchema {
        id: query_result.id,
        student_id: query_result.student_id,
        first_name: query_result.first_name,
        last_name: query_result.last_name,
        email: query_result.email,
        university: query_result.university,
    };

    let json_response = json!({
        "user": filtered_user
    });

    Ok(Json(json_response))
}