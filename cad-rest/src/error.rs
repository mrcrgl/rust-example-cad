use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("user error: {0}")]
    User(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (code, body) = match self {
            ApiError::User(message) => {
                let code = StatusCode::BAD_REQUEST;
                (
                    code,
                    ErrorResponse {
                        code: code.to_string(),
                        message,
                    },
                )
            }
        };

        let mut response = axum::Json(body).into_response();
        *response.status_mut() = code;
        response
    }
}
