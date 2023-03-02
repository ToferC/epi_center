use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct OrganizationQuery;

#[Object]
impl OrganizationQuery {

    // Organizations

    #[graphql(name = "allOrganizations")]
    /// Returns a vector of all organizations
    pub async fn all_organizations(&self, _context: &Context<'_>) -> Result<Vec<Organization>> {
        
        Organization::get_all()
    }

    #[graphql(name = "getCountOrganizations")]
    /// Accepts argument "count" and returns a vector of {count} organizations
    pub async fn get_count_organizations(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Organization>> {
        
        Organization::get_count(count)
    }

    #[graphql(name = "organizationByName")]
    pub async fn organization_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Organization>> {

        Organization::get_by_name(name)
    }

    // OrgTiers

    #[graphql(name = "allOrgTiers")]
    /// Returns a vector of all  org tiers
    pub async fn all_org_tiers(&self, _context: &Context<'_>) -> Result<Vec<OrgTier>> {

        OrgTier::get_all()
    }

    #[graphql(name = "getOrgTiers")]
    /// Accepts argument "count" and returns a vector of {count} org tiers
    pub async fn get_org_tiers(&self, _context: &Context<'_>, count: i64) -> Result<Vec<OrgTier>> {
        OrgTier::get_count(count)
    }

    #[graphql(name = "orgTierByName")]
    pub async fn org_tier_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<OrgTier>> {

        OrgTier::get_by_name(&name)
    }

}