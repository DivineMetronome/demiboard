use crate::util::multipart::MultipartError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;

#[derive(Debug)]

// TODO: add more error types
pub enum RequestError {
    Teapot,
    NotFound,
    Unauthorized,
    Internal(Box<dyn std::error::Error>),
    BadRequest(Box<dyn std::error::Error>),
}
impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Self::Internal(_) => format!("Internal error"),
            Self::BadRequest(info) => format!("Bad request: {}", info),
            Self::NotFound => "Not found".to_owned(),
            Self::Unauthorized => "Unauthorized".to_owned(),
            Self::Teapot => "Something fishy is going on".to_owned(),
        };
        write!(f, "{}", message)
    }
}
impl std::error::Error for RequestError {}

impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        let response = json!({
            "success": false,
            "message": self.to_string()
        });
        let status = match self {
            Self::Internal(err) => {
                eprintln!("Internal error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Teapot => StatusCode::IM_A_TEAPOT,
        };
        HttpResponse::build(status).json(response)
    }
}

impl From<sqlx::Error> for RequestError {
    fn from(error: sqlx::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<MultipartError> for RequestError {
    fn from(error: MultipartError) -> Self {
        match error {
            MultipartError::Internal(_) => Self::Internal(error.into()),
            _ => Self::BadRequest(error.into()),
        }
    }
}
