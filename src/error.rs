use crate::response::Response;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum AxumError {
    #[error(transparent)]
    // 这里完全采用sqlx的错误信息
    Database(#[from] sqlx::Error),
    #[error("route not found")]
    RouteNotFound,

    #[error("record does not exist")]
    RecordNotFound,
}

impl IntoResponse for AxumError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            AxumError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AxumError::RouteNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AxumError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
        };
        (status_code, Json(Response::<()>::err(message))).into_response()
    }
}
