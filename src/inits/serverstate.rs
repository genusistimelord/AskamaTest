#![allow(dead_code)]
use crate::forms::{User, UserOnline, UserOnlineDisplay};
use axum::{http::Request, middleware::Next, response::IntoResponse};
use axum_session::SessionPgPool;
use axum_session_auth::AuthSession;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub online: Arc<RwLock<HashMap<i64, UserOnline>>>,
    pub clean_timer: Arc<RwLock<DateTime<Utc>>>,
}

impl ServerState {
    pub async fn new() -> ServerState {
        ServerState {
            online: Arc::new(RwLock::new(HashMap::new())),
            clean_timer: Arc::new(RwLock::new(Utc::now() + Duration::hours(1))),
        }
    }

    pub async fn clean_list(&self) {
        let now = std::time::Instant::now();
        let dur = std::time::Duration::new(5, 0);

        let removals: Vec<i64> = {
            self.online
                .read()
                .await
                .iter()
                .filter_map(|s| {
                    if now.duration_since(s.1.time) >= dur {
                        Some(s.1.id)
                    } else {
                        None
                    }
                })
                .collect()
        };

        if !removals.is_empty() {
            let mut online = self.online.write().await;

            for i in removals {
                online.remove(&i);
            }
        }
    }

    pub async fn set_active(&self, user: &User) {
        if user.anonymous {
            return;
        }

        let mut online = self.online.write().await;

        if let Some(u) = online.get_mut(&(user.id as i64)) {
            u.time = std::time::Instant::now();
        } else {
            let u = UserOnline {
                id: user.id as i64,
                display: user.display.clone(),
                time: std::time::Instant::now(),
            };

            online.insert(user.id as i64, u);
        }
    }

    #[inline]
    pub async fn users_online(&self) -> Vec<UserOnlineDisplay> {
        self.online
            .read()
            .await
            .iter()
            .map(|x| UserOnlineDisplay {
                display: x.1.display.clone(),
                id: x.1.id,
            })
            .collect::<Vec<UserOnlineDisplay>>()
    }
}

pub async fn online_updater<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let extensions = req.extensions();
    let state: &ServerState = extensions.get().expect("Could not load state");
    let session: &AuthSession<User, i64, SessionPgPool, PgPool> =
        extensions.get().expect("Could not load user session");

    let timer = { *state.clean_timer.read().await };

    if timer < Utc::now() {
        state.clean_list().await;
        *state.clean_timer.write().await = Utc::now() + Duration::hours(1);
    }

    if let Some(user) = &session.current_user {
        state.set_active(user).await
    }

    next.run(req).await
}
