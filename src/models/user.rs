use crate::forms::*;
use sqlx::FromRow;
use std::collections::HashSet;

#[derive(sqlx::FromRow)]
pub struct SqlUser {
    pub id: i32,
    pub email_id: Option<i32>,
    pub anonymous: bool,
    pub username: String,
    pub display: String,
    pub password: Option<Vec<u8>>,
    pub first_name: String,
    pub last_name: String,
    pub lastactive: chrono::NaiveDateTime,
    pub regdate: chrono::NaiveDateTime,
    pub timezone: String,
    pub loginattempts: i32,
    pub loginlasttry: chrono::NaiveDateTime,
}

//Example showing how to convert a SQL User type into a Standard User
impl SqlUser {
    pub fn into_user(
        self,
        email: String,
        sql_user_perms: Option<Vec<SqlPermissionTokens>>,
        user_groups: Option<Vec<Group>>,
    ) -> crate::forms::User {
        crate::forms::User {
            id: self.id,
            email,
            anonymous: self.anonymous,
            username: self.username,
            display: self.display,
            password: if let Some(password) = self.password {
                String::from_utf8(password).ok()
            } else {
                None
            },
            first_name: self.first_name,
            last_name: self.last_name,
            lastactive: self.lastactive,
            regdate: self.lastactive,
            timezone: self.timezone,
            loginattempts: self.loginattempts,
            loginlasttry: self.lastactive,
            tokens: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
            groups: user_groups.unwrap_or_default(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct SqlUserPermissionTokens {
    pub token: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct SqlUserEmail {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
}

#[derive(FromRow, Clone)]
pub struct SqlGroup {
    pub id: i32,
    pub name: String,
    pub display: String,
}

impl SqlGroup {
    pub fn into_group(self, tokens: &[String]) -> crate::forms::Group {
        crate::forms::Group {
            id: self.id,
            display: self.display,
            name: self.name,
            tokens: tokens.iter().cloned().collect::<HashSet<String>>(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct SqlGroupPermission {
    pub id: i32,
    pub group_id: Option<i32>,
    pub token: Option<String>,
}

#[derive(FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}