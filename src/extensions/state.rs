use crate::{
    extensions::Flashes,
    forms::*,
    inits::{ServerState, SystemState},
    models::{SqlUserEmail, SqlUser},
};
use axum::{
    extract::Extension,
    http::Method,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use axum_flash::Flash;
use axum_macros::FromRequestParts;
use axum_session_auth::{AuthSession, SessionPgPool};
use sqlx::{PgPool, Row};
use std::convert::Infallible;

#[derive(FromRequestParts, Clone)]
#[from_request(state(SystemState))]
pub struct FullState {
    pub auth: AuthSession<User, i64, SessionPgPool, PgPool>,
    pub flashes: Flashes,
    pub method: Method,
    pub outgoingflash: Flash,
    #[from_request(via(Extension))]
    pub pool: PgPool,
    #[from_request(via(Extension))]
    pub state: ServerState,
    pub token: axum_csrf::CsrfToken,
}

// allows FullState to be called directly for handling into parts on its parts. 
impl IntoResponseParts for FullState {
    type Error = Infallible;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let res = self.outgoingflash.into_response_parts(res)?;
        self.token.into_response_parts(res)
    }
}

impl IntoResponse for FullState {
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}

// Used to get the Max number of objects so we can do pagination.
impl FullState {
    //this gets the closest esitimate of Rows within a fragmented Table if error returns 0.
    pub async fn get_row_count(&self, table_name: &str) -> i32 {
        sqlx::query(&format!("SELECT Count(*) FROM {}", table_name))
            .try_map(|row: sqlx::postgres::PgRow| row.try_get::<i64, _>(0))
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0) as i32
    }
}

// Used to get the limit of users per a pagination page.
impl FullState {
    pub async fn paginate_users(&self, page: u64, limit: u64) -> Vec<User> {
        match sqlx::query_as::<_, SqlUser>(
            "SELECT * FROM users order by username limit $2 offset ($2 * $1) - $2;",
        )
        .bind(page as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        {
            Ok(sql_users) => {
                let mut users: Vec<User> = Vec::with_capacity(sql_users.len());

                for sql_user in sql_users {
                    let email = if let Some(email_id) = sql_user.email_id {
                        sqlx::query_as::<_, SqlUserEmail>(
                            "SELECT * FROM public.emails email WHERE email.id = $1",
                        )
                        .bind(email_id)
                        .fetch_one(&self.pool)
                        .await
                        .map(|sqlemail| sqlemail.email)
                        .unwrap_or_else(|_| "spirean@spirean.com".into())
                    } else {
                        "spirean@spirean.com".into()
                    };
                    users.push(sql_user.into_user(email, None, None));
                }

                users
            }
            Err(_) => Vec::<User>::new(),
        }
    }
}