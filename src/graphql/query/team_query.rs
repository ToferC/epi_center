use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct TeamQuery;

#[Object]
impl TeamQuery {

    // Teams
    #[graphql(name = "allTeams")]
    /// Returns a vector of all travel groups
    pub async fn all_teams(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Team>> {

        Team::get_all()
    }

    #[graphql(name = "teamByID")]
    /// Returns a specific travel group by its UUID
    pub async fn team_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Team> {

        Team::get_by_id(&id)
    }

    #[graphql(name = "teamByName")]
    pub async fn team_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Team>> {

        Team::get_by_name(name)
    }
}