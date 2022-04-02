use async_trait::async_trait;
use axum::{
    body::BoxBody,
    extract::{Form, FromRequest, RequestParts},
    http::{Response, StatusCode},
    response::IntoResponse,
    BoxError,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

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
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<self::BoxBody> {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::SqlxErrors(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ServerError::AskamaError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
        .into_response()
    }
}
