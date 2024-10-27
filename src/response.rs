use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new(message: String, data: Option<T>) -> Self {
        Self { message, data }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(String::from("OK"), data)
    }

    pub fn err(message: String) -> Self {
        Self::new(message, None)
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
