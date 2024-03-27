use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::header::InvalidHeaderValue;
use serde_json::{json, Value};
use tracing::error;

#[allow(dead_code)]
pub enum HttpSuccess<'a> {
    Created(Value),
    OK(Value),
    Success(&'a str),
}

impl HttpSuccess<'_> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Created(_) => StatusCode::CREATED,
            Self::OK(_) | Self::Success(_) => StatusCode::OK,
        }
    }
}

impl IntoResponse for HttpSuccess<'_> {
    fn into_response(self) -> Response {
        match self {
            Self::Created(ref v) | Self::OK(ref v) => {
                (self.status_code(), Json(json!({ "data": v }))).into_response()
            }
            Self::Success(ref v) => (
                self.status_code(),
                Json(json!(
                    {
                        "result": {
                            "type": "success",
                            "message": v
                        }
                    }
                )),
            )
                .into_response(),
        }
    }
}

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum HttpError<'a> {
    #[error("Invalid request parameters: {0}")]
    BadRequest(&'a str),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Resource {0} not found")]
    NotFound(&'a str),

    #[error("The server encountered an unexpected error")]
    Sqlx(#[from] sqlx::Error),

    #[error("The server encountered an unexpected error")]
    Anyhow(#[from] anyhow::Error),

    #[error("The server encountered an unexpected error")]
    StdIOError(#[from] std::io::Error),

    #[error("The server encountered an unexpected error")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

impl HttpError<'_> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Sqlx(_) | Self::Anyhow(_) | Self::StdIOError(_) | Self::InvalidHeaderValue(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for HttpError<'_> {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => {
                error!("SQLx error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                error!("Generic error: {:?}", e);
            }

            _ => (),
        }

        (
            self.status_code(),
            Json(json!(
                {
                    "result": {
                        "type": "error",
                        "message": &self.to_string(),
                    }
                }
            )),
        )
            .into_response()
    }
}
