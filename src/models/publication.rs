use std::fmt::Debug;

use chrono::{prelude::*};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::schema::*;
use crate::database::connection;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[table_name = "publications"]
pub struct Publication {
    pub id: Uuid,
    pub publishing_organization: Uuid,
    pub lead_author_id: Uuid, // Person
    pub title: String,
    pub subject: String,
    pub publication_status: PublicationStatus,
    pub url: Option<String>,
    pub publishing_id: Option<String>,
    pub submitted_date: NaiveDateTime,
    pub published_datestamp: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::PublicationStatus"]
pub enum PublicationStatus {
    Planning,
    InProgress,
    Submitted,
    Published,
    Cancelled,
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
            .filter(publications::created_by_person_id.eq(&publication.created_by_person_id)
                .and(publications::title_en.eq(&publication.title_en))
                .and(publications::assigned_to_person_id.eq(&publication.assigned_to_person_id))
                .and(publications::target_completion_date.eq(&publication.target_completion_date))
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

    pub fn get_by_id(id: Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let res = publications::table.filter(publications::id.eq(id)).first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_team_id(id: Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::team_id.eq(id))
            .load::<Publication>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_assigning_person_id(id: Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::created_by_person_id.eq(id))
            .load::<Publication>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_assigned_person_id(id: Uuid) -> Result<Vec<Publication>> {
        let mut conn = connection()?;

        let res = publications::table
            .filter(publications::assigned_to_person_id.eq(id))
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
    pub created_by_person_id: Uuid, // Person
    pub assigned_to_person_id: Option<Uuid>, // Person
    pub team_id: Uuid, // Team
    pub title_en: String,
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_datestamp: NaiveDateTime,
    pub target_completion_date: NaiveDateTime,
    pub publication_status: PublicationStatus,
    pub effort: f64,
}

impl NewPublication {

    pub fn new(
        created_by_person_id: Uuid, // Person
        assigned_to_person_id: Option<Uuid>, // Person
        team_id: Uuid, // Publication
        title_en: String,
        outcome_en: String,
        outcome_fr: String,
        start_datestamp: NaiveDateTime,
        target_completion_date: NaiveDateTime,
        publication_status: PublicationStatus,
        effort: f64,

    ) -> Self {
        NewPublication {
            created_by_person_id,
            assigned_to_person_id,
            team_id,
            title_en,
            outcome_en,
            outcome_fr,
            start_datestamp,
            target_completion_date,
            publication_status,
            effort,
        }
    }
}
