use crate::{
    extensions::{Flashes, FullState},
    forms::User,
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_sessions_auth::Authentication;

#[derive(Template, Default)]
#[template(path = "error/404.htm")]
struct Error404 {
    flashes: Flashes,
    current_user: User,
}

impl IntoResponse for Error404 {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(body) => {
                let headers = [(
                    http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static(Self::MIME_TYPE),
                )];

                (headers, body).into_response()
            }
            Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub async fn handler_404(state: FullState) -> impl IntoResponse {
    let err = Error404 {
        flashes: Flashes::default(),
        current_user: state.auth.current_user.unwrap_or_default(),
    };
    (StatusCode::NOT_FOUND, err)
}
