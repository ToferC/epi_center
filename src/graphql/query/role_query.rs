use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {

    // Roles

    #[graphql(name = "roleCount")]
    /// Accepts an argument of "count" and returns a vector of {count} role
    pub async fn get_count_role(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Role>> {
        
        Role::get_count(count)
    }

    #[graphql(name = "allRoles")]
    /// Returns a vector of all persons ordered by family name
    pub async fn all_roles(
        &self, 
        _context: &Context<'_>,) -> Result<Vec<Role>> {

        Role::get_all()
    }

    #[graphql(name = "roleById")]
    pub async fn role_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid,
    ) -> Result<Role> {

        Role::get_by_id(id)
    }
}