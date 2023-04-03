use std::fmt::Debug;

use chrono::{prelude::*};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods, PgTextExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::database::connection;

use crate::models::{Person, PublicationContributor, Organization};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[diesel(table_name = publications)]
pub struct Publication {
    pub id: Uuid,
    #[graphql(skip)]
    pub publishing_organization_id: Uuid,
    #[graphql(skip)]
    pub lead_author_id: Uuid, // Person
    pub title: String,
    pub subject_text: String,
    pub publication_status: PublicationStatus,
    pub url_string: Option<String>,
    pub publishing_id: Option<String>,
    pub submitted_date: Option<NaiveDateTime>,
    pub published_datestamp: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::PublicationStatus"]
pub enum PublicationStatus {
    Planning,
    InProgress,
    Draft,
    Submitted,
    Published,
    Rejected,
    Cancelled,
}

#[ComplexObject]
impl Publication {
    pub async fn lead_author(&self) -> Result<Person> {
        Person::get_by_id(&self.lead_author_id)
    }

    pub async fn publishing_organization(&self) -> Result<Organization> {
        Organization::get_by_id(&self.publishing_organization_id)
    }

    pub async fn contributors(&self) -> Result<Vec<Person>> {
        
        let ids = PublicationContributor::get_contributor_ids(&self.id)?;
        
        Person::get_by_ids(&ids)
    }
}

// Non Graphql
impl Publication {
    pub fn create(publication: &NewPublication) -> Result<Publication> {
        let mut conn = connection()?;

        let res = diesel::insert_into(publications::table)
        .values(publication)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(publication: &NewPublication) -> Result<Publication> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::publishing_organization_id.eq(&publication.publishing_organization_id)
                .and(publications::title.eq(&publication.title))
                .and(publications::lead_author_id.eq(&publication.lead_author_id))
                .and(publications::publishing_id.eq(&publication.publishing_id))
            )
            .distinct()
            .first(&mut conn);
        
        let publication = match res {
            Ok(p) => p,
            Err(e) => {
                // Publication not found
                println!("{:?}", e);
                let p = Publication::create(publication).expect("Unable to create publication");
                p
            }
        };
        Ok(publication)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = publications::table.load::<Publication>(&mut conn)?;
        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = publications::table
            .limit(count)
            .load::<Publication>(&mut conn)?;
        
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let res = publications::table.filter(publications::id.eq(id)).first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_ids(ids: &[Uuid]) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = publications::table.filter(publications::id.eq_any(ids))
            .load::<Publication>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_contributor_id(person_id: &Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;
        let res: Vec<Uuid> = publication_contributors::table
            .filter(publication_contributors::contributor_id.eq(person_id))
            .select(publication_contributors::publication_id)
            .load::<Uuid>(&mut conn)?;

        let publications = Publication::get_by_ids(&res)?;

        Ok(publications)
    }

    pub fn get_by_lead_author_id(id: &Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::lead_author_id.eq(id))
            .load::<Publication>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_publishing_organization_id(id: &Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::publishing_organization_id.eq(id))
            .load::<Publication>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_title_or_subject(title: &str) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::title.ilike(format!("%{}%", title)))
            .or_filter(publications::subject_text.ilike(format!("%{}%", title)))
            .load::<Publication>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(publications::table)
        .filter(publications::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject, InputObject)]
#[table_name = "publications"]
pub struct NewPublication {
    pub publishing_organization_id: Uuid,
    pub lead_author_id: Uuid, // Person
    pub title: String,
    pub subject_text: String,
    pub publication_status: PublicationStatus,
    pub url_string: Option<String>,
    pub publishing_id: Option<String>,
    pub submitted_date: Option<NaiveDateTime>,
    pub published_datestamp: Option<NaiveDateTime>,
}

impl NewPublication {

    pub fn new(
        publishing_organization_id: Uuid,
        lead_author_id: Uuid, // Person
        title: String,
        subject_text: String,
        publication_status: PublicationStatus,
        url_string: Option<String>,
        publishing_id: Option<String>,
        submitted_date: Option<NaiveDateTime>,
        published_datestamp: Option<NaiveDateTime>,

    ) -> Self {
        NewPublication {
            publishing_organization_id,
            lead_author_id,
            title,
            subject_text,
            publication_status,
            url_string,
            publishing_id,
            submitted_date,
            published_datestamp,
        }
    }
}
