use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {

    // Users / Admin

    #[graphql(
        name = "allUsers",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn all_users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let mut conn = get_connection_from_context(context);

        let res = users::table.load::<User>(&mut conn)?;

        Ok(res)
    }

    #[graphql(
        name = "getUserByEmail",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_email(&self, _context: &Context<'_>, email: String) -> Result<User> {

        let res = User::get_by_email(&email)?;

        Ok(res)
    }

    #[graphql(
        name = "getUserById",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_id(&self, _context: &Context<'_>, id: Uuid) -> Result<User> {

        let res = User::get_by_id(&id)?;

        Ok(res)
    }
}