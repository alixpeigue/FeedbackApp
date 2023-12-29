use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApplicationError {
    NotFound,
    InternalError(anyhow::Error),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::InternalError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")),
        };
        (status_code, Json(json!({ "error": error_message }))).into_response()
    }
}

// impl From<sqlx::Error> for ApplicationError {
//     fn from(err: sqlx::Error) -> Self {
//         match err {
//             sqlx::Error::RowNotFound => Self::NotFound,
//             _ => Self::InternalError(err.into()),
//         }
//     }
// }

impl<E> From<E> for ApplicationError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        // match (err.into()) {
        //     sqlx::Error::RowNotFound => Self::NotFound
        // }
        // if let Ok(sqlx::Error::RowNotFound) = err.try_into() {
        //     Self::NotFound
        // } else {
        //     Self::InternalError(err.into())
        // }
        // match err {
        //     e => Self::NotFound,
        //     err => Self::InternalError(err.into()),
        // }
        // let a: sqlx::Error = err.into();
        Self::InternalError(err.into())
    }
}
