use crate::{extensions::Flashes, forms::User, inits::ServerState};
use axum::extract::Extension;
use axum_flash::Flash;
use axum_macros::FromRequest;
use axum_sessions_auth::*;
use std::fmt::Debug;

#[derive(FromRequest)]
#[from_request(rejection_derive(!Display, !Error))]
pub struct FullState {
    #[from_request(via(Extension))]
    pub state: ServerState,
    pub auth: AuthSession<User>,
    pub token: axum_csrf::CsrfToken,
    pub flashes: Flashes,
    pub outgoingflash: Flash,
}
