use crate::forms::User;
use crate::{
    extensions::{Flashes, FullState},
    forms::UserOnlineDisplay,
};
//use askama::Template;
use askama_derive::*;
use askama_shared::Template;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
//use axum_macros::debug_handler;
use axum_sessions_auth::Authentication;
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

impl IntoResponse for MainTemplate {
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

pub fn main_routes() -> Router {
    Router::new().route("/main", get(main_page))
}

async fn main_page(state: FullState) -> MainTemplate {
    let online_user = state.state.users_online().await;

    MainTemplate {
        current_user: state.auth.current_user.unwrap_or_default(),
        online: online_user,
        flashes: state.flashes,
    }
}
