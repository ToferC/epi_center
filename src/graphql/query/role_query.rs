use async_graphql::*;

use crate::models::{Role};
use uuid::Uuid;

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {

    // Roles

    #[graphql(name = "activeRoles")]
    /// Accepts an argument of "count" and returns a vector of {count} active role
    pub async fn get_active_role(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Role>> {
        
        Role::get_active(count)
    }

    #[graphql(name = "vacantRoles")]
    /// Accepts an argument of "count" and returns a vector of {count} active role
    pub async fn get_vacant_role(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Role>> {
        
        Role::get_vacant(count)
    }

    #[graphql(name = "allRoles")]
    /// Returns a vector of all persons ordered by family name
    pub async fn all_roles(
        &self, 
        _context: &Context<'_>,) -> Result<Vec<Role>> {

        Role::get_all_active()
    }

    #[graphql(name = "roleById")]
    pub async fn role_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid,
    ) -> Result<Role> {

        Role::get_by_id(&id)
    }

    #[graphql(name = "roleCount")]
    /// returns a count of the total roles in the system
    pub async fn role_count(
        &self, 
        _context: &Context<'_>,
    ) -> Result<i64> {

        Role::count()
    }
}