use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Publication, Role, OrgTier, Capability, Skill, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct PublicationQuery;

#[Object]
impl PublicationQuery {

    // Publications

    #[graphql(name = "allPublications")]
    /// Returns a vector of all publications
    pub async fn all_publications(&self, _context: &Context<'_>) -> Result<Vec<Publication>> {
        
        Publication::get_all()
    }

    #[graphql(name = "publicationCount")]
    /// Accepts argument "count" and returns a vector of {count} publications
    pub async fn get_count_publications(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Publication>> {
        
        Publication::get_count(count)
    }

    #[graphql(name = "publicationByTitle")]
    /// Accepts argument "title" and returns a vector of publications with that title or subject
    pub async fn publication_by_title_or_subject(
        &self, 
        _context: &Context<'_>,
        title: String,
    ) -> Result<Vec<Publication>> {

        Publication::get_by_title_or_subject(&title)
    }
}