use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{schema::*, database};
use crate::models::{Person, Organization};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[table_name = "affiliations"]
pub struct Affiliation {
    pub id: Uuid,
    pub person_id: Uuid,
    pub organization_id: Uuid,
    pub home_org_id: Uuid,
    pub affiliation_role: String,

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Affiliation {
    pub async fn person(&self) -> Result<Person> {
        Person::get_by_id(&self.person_id)
    }

    pub async fn organization(&self) -> Result<Organization> {
        Organization::get_by_id(&self.organization_id)
    }

    pub async fn home_organization(&self) -> Result<Organization> {
        Organization::get_by_id(&self.home_org_id)
    }
}

// Non Graphql
impl Affiliation {
    pub fn create(affiliation: &NewAffiliation) -> Result<Affiliation> {
        let mut conn = database::connection()?;

        let res = diesel::insert_into(affiliations::table)
            .values(affiliation)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(affiliation: &NewAffiliation) -> Result<Affiliation> {
        let mut conn = database::connection()?;

        let res = affiliations::table
        .filter(affiliations::person_id.eq(&affiliation.person_id).and(affiliations::organization_id.eq(&affiliation.organization_id)))
        .distinct()
        .first(&mut conn);
        
        let affiliation = match res {
            Ok(p) => p,
            Err(e) => {
                // Affiliation not found
                println!("{:?}", e);
                let p = Affiliation::create(affiliation).expect("Unable to create affiliation");
                p
            }
        };
        Ok(affiliation)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = affiliations::table.load::<Affiliation>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = database::connection()?;
        let res = affiliations::table.filter(affiliations::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_person_id(person_id: Uuid) -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = affiliations::table.filter(affiliations::person_id.eq(person_id))
            .load::<Affiliation>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_organization_id(organization_id: Uuid) -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = affiliations::table.filter(affiliations::organization_id.eq(organization_id))
            .load::<Affiliation>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_home_organization_id(organization_id: Uuid) -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = affiliations::table
            .filter(affiliations::home_org_id.eq(organization_id))
            .load::<Affiliation>(&mut conn)?;
        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = database::connection()?;

        let res = diesel::update(affiliations::table)
        .filter(affiliations::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject, InputObject)]
#[table_name = "affiliations"]
pub struct NewAffiliation {
    pub person_id: Uuid,
    pub organization_id: Uuid,
    pub home_org_id: Uuid,
    pub affiliation_role: String,
    pub end_date: Option<NaiveDateTime>
}

impl NewAffiliation {

    pub fn new(
        person_id: Uuid,
        organization_id: Uuid,
        home_org_id: Uuid,
        affiliation_role: String,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        NewAffiliation {
            person_id,
            organization_id,
            home_org_id,
            affiliation_role,
            end_date,
        }
    }
}
