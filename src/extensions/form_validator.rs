use axum::{
    body::BoxBody,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_session::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    SqlxErrors(#[from] sqlx::Error),

    #[error(transparent)]
    AskamaError(#[from] askama::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Format(#[from] std::fmt::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),

    #[error(transparent)]
    Session(#[from] SessionError),

    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    UTF8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    FutureSpawn(#[from] futures::task::SpawnError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<self::BoxBody> {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
        .into_response()
    }
}
