use async_graphql::*;

use crate::models::{Affiliation};
use uuid::Uuid;

/*
use crate::common_utils::{RoleGuard, is_admin, UserRole};
*/

#[derive(Default)]
pub struct AffiliationQuery;

#[Object]
impl AffiliationQuery {

    // Affiliations

    #[graphql(name = "allAffiliations")]
    /// Returns a vector of all affiliations
    pub async fn all_affiliations(&self, _context: &Context<'_>) -> Result<Vec<Affiliation>> {
        
        Affiliation::get_all()
    }

    #[graphql(name = "affiliationById")]
    /// Accepts id and returns a affiliations
    pub async fn affiliation_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid,
    ) -> Result<Affiliation> {

        Affiliation::get_by_id(&id)
    }
}