use crate::forms::User;
use crate::{
    extensions::{Flashes, FullState},
    forms::UserOnlineDisplay,
    inits::SystemState,
};
use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
//use axum_macros::debug_handler;
use axum_session_auth::Authentication;
use serde::*;

#[derive(Template, Debug, Deserialize, Serialize)]
#[template(path = "main/main.html")]
struct MainTemplate {
    #[serde(default)]
    current_user: User,
    #[serde(default)]
    online: Vec<UserOnlineDisplay>,
    #[serde(default)]
    flashes: Flashes,
}

pub fn main_routes() -> Router<SystemState> {
    Router::new().route("/main", get(main_page))
}

async fn main_page(state: FullState) -> impl IntoResponse {
    let online_user = state.state.users_online().await;
    let flashes = state.flashes.to_owned();
    let current_user = state.auth.current_user.to_owned().unwrap_or_default();

    (
        state,
        MainTemplate {
            current_user,
            online: online_user,
            flashes,
        },
    )
        .into_response()
}
