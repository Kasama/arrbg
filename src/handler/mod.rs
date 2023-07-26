use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub mod health;
pub mod torrentsapi;

struct AppError(anyhow::Error);
struct AppResult<T, E>(Result<T, E>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(axum::body::boxed(
                json!({ "error": self.0.to_string() }).to_string(),
            ))
            .unwrap_or_else(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("error: {}", self.0),
                )
                    .into_response()
            })
    }
}

impl<T, E> IntoResponse for AppResult<T, E>
where
    T: IntoResponse,
    E: Into<AppError>,
{
    fn into_response(self) -> Response {
        match self.0 {
            Ok(o) => o.into_response(),
            Err(e) => e.into().into_response(),
        }
    }
}

impl<T, E> From<Result<T, E>> for AppResult<T, E>
where
    E: Into<AppError>,
{
    fn from(value: Result<T, E>) -> Self {
        Self(value)
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
