// Modelled off https://github.com/clifinger/canduma/blob/master/src/user

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::{self, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{schema::*};
use crate::common_utils::{is_admin, RoleGuard, UserRole};
use crate::models::hash_password;
use crate::database::connection;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInstance {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, Queryable, AsChangeset)]
pub struct User {
    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub id: Uuid,
    #[graphql(skip)]
    pub hash: String,

    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub email: String,
    pub role: String,

    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub name: String,
    pub access_level: String, // AccessLevelEnum
    pub created_at: NaiveDateTime,
    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]

    pub updated_at: NaiveDateTime,
    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]

    /// Access Level: Admin
    pub access_key: String,

    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Access Level: Admin
    pub approved_by_user_uid: Option<Uuid>,
}

impl User {

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let user = users::table
            .filter(users::id.eq(id))
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn get_by_email(email: &String) -> Result<Self> {
        let mut conn = connection()?;
        let user = users::table
            .filter(users::email.eq(email))
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: InsertableUser) -> Result<Self> {
        let mut conn = connection()?;
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(&mut self) -> Result<Self> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let user = diesel::update(users::table)
            .filter(users::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(user)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
    pub access_level: String, // AccessLevelEnum
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub access_key: String,
    pub approved_by_user_uid: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, InputObject)]
/// Input Struct to create a new user. Only accessible by Administrators.
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
    /// UserRole in system: USER, OPERATOR, ANALYST, ADMIN
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, InputObject)]
/// Input Struct to create a new user. Only accessible by Administrators.
pub struct UserUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    /// UserRole in system: USER, OPERATOR, ANALYST, ADMIN
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct SlimUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub access_level: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {

        let updated_at = chrono::Utc::now().naive_utc();

        let UserData {
            name,
            email,
            password,
            role,
            ..
        } = user_data;
        
        let hash = hash_password(&password)
            .expect("Unable to hash password")
            .to_string();

        Self {
            email,
            hash,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at,
            name,
            role,
            access_key: "".to_owned(),
            access_level: "detailed".to_owned(),
            approved_by_user_uid: None,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            id,
            email,
            role,
            access_level,
            ..
        } = user;

        Self {
            id,
            email,
            role,
            access_level,
        }
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub struct LoginQuery {
    pub email: String,
    pub password: String,
}