use async_graphql::*;
use uuid::Uuid;

use crate::models::{Organization, OrgTier};

/*
use crate::common_utils::{RoleGuard, is_admin, UserRole};
*/

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

    #[graphql(name = "organizations")]
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

    #[graphql(name = "organizationById")]
    pub async fn organization_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid,
    ) -> Result<Organization> {

        Organization::get_by_id(&id)
    }

    // OrgTiers

    #[graphql(name = "allOrgTiers")]
    /// Returns a vector of all  org tiers
    pub async fn all_org_tiers(&self, _context: &Context<'_>) -> Result<Vec<OrgTier>> {

        OrgTier::get_all()
    }

    #[graphql(name = "orgTiersByOrgId")]
    /// Returns a vector of org tiers for a specific org ID
    pub async fn org_tiers_by_org_id(&self, _context: &Context<'_>, id: Uuid) -> Result<Vec<OrgTier>> {

        OrgTier::get_by_org_id(&id)
    }

    #[graphql(name = "OrgTiers")]
    /// Accepts argument "count" and returns a vector of {count} org tiers
    pub async fn get_org_tiers(&self, _context: &Context<'_>, count: i64) -> Result<Vec<OrgTier>> {
        OrgTier::get_count(count)
    }

    #[graphql(name = "orgTierById")]
    pub async fn org_tier_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid,
    ) -> Result<OrgTier> {

        OrgTier::get_by_id(&id)
    }

    #[graphql(name = "orgTierByName")]
    pub async fn org_tier_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<OrgTier>> {

        OrgTier::get_by_name(&name)
    }

    // Not done this yet
    #[graphql(name = "orgChart")]
    pub async fn org_chart(
        &self,
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Vec<String>> {

        // Data format: name,imageUrl,area,profileUrl,office,tags,isLoggedUser,positionName,id,parentId,size

        let data = Vec::new();

        let _org_tiers = OrgTier::get_by_org_id(&id)?;

        let _query = "query {
            allOrgTiers {
              nameEn
              owner {
                id
                givenName
                familyName
                activeRoles {
                  titleEnglish
                }
              }
              parentOrganizationTier {
                owner {
                  id
                }
              }
            }
          }";


        Ok(data)
    }

}