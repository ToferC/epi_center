use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::database::connection;

use super::{Publication, Person};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "publication_contributors"]
/// Data structure connecting persons in heirarchical relationship
pub struct PublicationContributor {
    pub id: Uuid,
    pub publication_id: Uuid, // Publication
    pub contributor_id: Uuid, // Person
    pub contributor_role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl PublicationContributor {
    pub async fn publication(&self) -> Result<Publication> {
        Publication::get_by_id(&self.publication_id)
    }

    pub async fn contributor(&self) -> Result<Person> {
        Person::get_by_id(&self.contributor_id)
    }

    pub async fn contributor_role(&self) -> Result<String> {
        Ok(self.contributor_role.clone())
    }
}


// Non Graphql
impl PublicationContributor {
    pub fn create(publication_contributor: &NewPublicationContributor) -> Result<PublicationContributor> {
        let mut conn = connection()?;

        let res = diesel::insert_into(publication_contributors::table)
            .values(publication_contributor)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(publication_contributor: &NewPublicationContributor) -> Result<PublicationContributor> {
        let mut conn = connection()?;

        let res = publication_contributors::table
        .filter(publication_contributors::publication_id.eq(&publication_contributor.publication_id)
            .and(publication_contributors::contributor_id.eq(&publication_contributor.contributor_id)))
        .distinct()
        .first(&mut conn);
        
        let publication_contributor = match res {
            Ok(p) => p,
            Err(e) => {
                // PublicationContributor not found
                println!("{:?}", e);
                let p = PublicationContributor::create(publication_contributor).expect("Unable to create publication_contributor");
                p
            }
        };
        Ok(publication_contributor)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let persons = publication_contributors::table.load::<PublicationContributor>(&mut conn)?;
        Ok(persons)
    }

    pub fn get_contributor_ids(publication_id: &Uuid) -> Result<Vec<Uuid>> {

        let mut conn = connection()?;
        let res: Vec<Uuid> = publication_contributors::table
            .filter(publication_contributors::publication_id.eq(publication_id))
            .select(publication_contributors::contributor_id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let person = publication_contributors::table
            .filter(publication_contributors::id.eq(id))
            .first(&mut conn)?;
        Ok(person)
    }

    pub fn get_by_contributor_id(contributor_id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = publication_contributors::table
            .filter(publication_contributors::contributor_id.eq(contributor_id))
            .load::<PublicationContributor>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(publication_contributors::table)
        .filter(publication_contributors::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "publication_contributors"]
pub struct NewPublicationContributor {
    pub publication_id: Uuid, // Publication
    pub contributor_id: Uuid, // Person
    pub contributor_role: String,
}

impl NewPublicationContributor {

    pub fn new(
        publication_id: Uuid,
        contributor_id: Uuid,
        contributor_role: String,
    ) -> Self {
        NewPublicationContributor {
            publication_id,
            contributor_id,
            contributor_role,
        }
    }
}
