
use async_trait::async_trait;
use axum_session_auth::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;
use std::time::Instant;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub anonymous: bool,
    pub username: String,
    pub display: String,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub lastactive: chrono::NaiveDateTime,
    pub regdate: chrono::NaiveDateTime,
    pub timezone: String,
    pub loginattempts: i32,
    pub loginlasttry: chrono::NaiveDateTime,
    pub tokens: HashSet<String>,
    pub groups: Vec<Group>,
}


impl Default for User {
    fn default() -> Self {
        let d = chrono::NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
        let t = chrono::NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
        let mut tokens = HashSet::new();

        tokens.insert("category:view".to_owned());

        Self {
            id: 1,
            email: "spirean@spirean.com".into(),
            anonymous: true,
            username: "Guest".into(),
            display: "Guest".into(),
            password: None,
            first_name: "Guest".into(),
            last_name: "None".into(),
            lastactive: chrono::NaiveDateTime::new(d, t),
            regdate: chrono::NaiveDateTime::new(d, t),
            timezone: "UTC".into(),
            loginattempts: 0,
            loginlasttry: chrono::NaiveDateTime::new(d, t),
            tokens,
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub display: String,
    #[serde(skip)]
    pub tokens: HashSet<String>,
}

/// An anyhow::Result with default return type of ()

#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        matches!(perm, "Token::UseAdmin" | "Token::ModifyUser")
    }
}

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(
        _userid: i64,
        _pool: Option<&PgPool>,
    ) -> Result<User, anyhow::Error> {
        Ok(User::default())
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

impl User {
    pub fn check_rights(&self, rights: &[String]) -> bool {
        for right in rights {
            if !self.tokens.contains(right) {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UserOnline {
    pub id: i64,
    pub display: String,
    #[serde(skip)]
    pub time: Instant,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOnlineDisplay {
    pub id: i64,
    pub display: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SignInForm {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
    pub remeber_me: bool,
}
