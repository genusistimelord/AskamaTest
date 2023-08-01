use crate::{
    extensions::{Flashes, FullState},
    forms::User,
};
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_session_auth::Authentication;


#[derive(Template, Default)]
#[template(path = "error/404.htm")]
struct Error404 {
    flashes: Flashes,
    current_user: User,
}

pub async fn handler_404(state: FullState) -> impl IntoResponse {
    let err = Error404 {
        flashes: Flashes::default(),
        current_user: state.auth.current_user.unwrap_or_default(),
    };
    (StatusCode::NOT_FOUND, err)
}
