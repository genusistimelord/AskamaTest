#[derive(sqlx::FromRow)]
pub struct SqlUser {
    pub id: i32,
    pub email_id: Option<i32>,
    pub anonymous: Option<bool>,
    pub username: Option<String>,
    pub display: Option<String>,
    pub password: Option<Vec<u8>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub lastactive: Option<chrono::NaiveDateTime>,
    pub regdate: Option<chrono::NaiveDateTime>,
    pub timezone: Option<String>,
    pub loginattempts: Option<i32>,
    pub loginlasttry: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow)]
pub struct SqlUserGroups {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct SqlUserPermission {
    pub id: i32,
    pub user_id: Option<i32>,
    pub token: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct SqlUserPermissionTokens {
    pub token: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct SqlUserEmail {
    pub id: i32,
    pub user_id: Option<i32>,
    pub email: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct SqlGroup {
    pub id: i32,
    pub name: Option<String>,
    pub display: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct SqlGroupPermission {
    pub id: i32,
    pub group_id: Option<i32>,
    pub token: Option<String>,
}
