use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct CapabilityQuery;

#[Object]
impl CapabilityQuery {

    // Capabilities

    pub async fn get_capabilities(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Capability>> {

        Capability::get_all()
    }

    pub async fn capability_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Capability> {

        Capability::get_by_id(&id)
    }

    pub async fn get_capabilities_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Capability>> {

        let skill_ids = Skill::get_skill_ids_by_name(name)?;

        Capability::get_by_skill_ids(skill_ids)
    }

    /// Return a count of the number of people who have a capability at each level of the capability
    pub async fn get_capability_counts(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_name(name)
    }

    // Skills

    pub async fn get_skills(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Skill>> {

        Skill::get_all()
    }

    pub async fn get_skill_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Skill> {

        Skill::get_by_id(&id)
    }

    pub async fn get_skill_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Skill>> {

        Skill::get_by_name(name)
    }
}