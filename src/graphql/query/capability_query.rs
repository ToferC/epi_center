use async_graphql::*;

use crate::models::{Capability, Skill, CapabilityCount, SkillDomain, CapabilityLevel};
use uuid::Uuid;

//use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct CapabilityQuery;

#[Object]
impl CapabilityQuery {

    // Capabilities
    /// Returns count number of Capabilities in the system
    pub async fn capabilities(
        &self, 
        _context: &Context<'_>,
        count: i64,
    ) -> Result<Vec<Capability>> {

        Capability::get_count(count)
    }

    /// Returns a capability by its Uuid
    pub async fn capability_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Capability> {

        Capability::get_by_id(&id)
    }

    /// Accepts a String "name" and returns a vector of capabilities that 
    /// match in EN or FR against it
    pub async fn capabilities_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Capability>> {

        Capability::get_by_name(&name)
    }

    /// Accepts a String "name" and a CapabilityLevel and returns matches against both
    pub async fn capabilities_by_name_and_level(
        &self, 
        _context: &Context<'_>,
        name: String,
        level: CapabilityLevel,
    ) -> Result<Vec<Capability>> {

        Capability::get_by_name_and_level(&name, level)
    }
       
    /// Return a count of the number of people who have a capability at each level of the capability
    pub async fn capability_counts_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_name(name)
    }

    /// Return a CapabilityCount by a specific SkillDomain (SCIENTIFIC, etc.)
    pub async fn capability_counts_by_domain(
        &self, 
        _context: &Context<'_>,
        domain: SkillDomain,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_domain(domain)
    }

    // Skills

    /// Returns vector of all skills
    pub async fn skills(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Skill>> {

        Skill::get_all()
    }

    /// Returns a specific skill by ID
    pub async fn skill_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Skill> {

        Skill::get_by_id(&id)
    }

    /// Returns a vector of skills matching some part of the name provided
    pub async fn skill_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Skill>> {

        Skill::get_by_name(name)
    }
}