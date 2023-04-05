use async_graphql::*;

use crate::models::{Person};
use uuid::Uuid;

/*
use crate::common_utils::{RoleGuard, is_admin, UserRole};
*/

#[derive(Default)]
pub struct PersonQuery;

#[Object]
impl PersonQuery {

    // People 
    #[graphql(name = "allPeople")]
    /// Accepts argument of "count" and returns a vector of {count} persons ordered by
    /// family name.D
    pub async fn all_people(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Person>> {

        Person::get_all()
    }

    #[graphql(name = "People")]
    /// Accepts argument of "count" and returns a vector of {count} persons ordered by
    /// family name
    pub async fn get_people(
        &self, 
        _context: &Context<'_>,
        count: i64,
    ) -> Result<Vec<Person>> {

        Person::get_count(count)
    }

    #[graphql(name = "peopleCount")]
    /// Accepts argument of "count" and returns a vector of {count} persons ordered by
    /// family name
    pub async fn people_count(
        &self, 
        _context: &Context<'_>,
    ) -> Result<i64> {

        Person::count()
    }

    

    #[graphql(name = "personById")]
    pub async fn person_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Person> {

        Person::get_by_id(&id)
    }

    #[graphql(name = "personByName")]
    pub async fn person_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Person>> {

        Person::get_by_name(&name)
    }
}