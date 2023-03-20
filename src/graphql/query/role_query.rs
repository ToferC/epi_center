use async_graphql::*;

use crate::models::{Role};
use uuid::Uuid;

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {

    // Roles

    #[graphql(name = "roles")]
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

        Role::get_by_id(&id)
    }
}