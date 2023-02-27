use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::common_utils::{
    is_analyst, RoleGuard, Role};

use crate::schema::*;
use crate::graphql::{graphql_translate, get_connection_from_context};
use crate::models::{Country, Trip};
use crate::get_country_by_id;

use super::PublicHealthProfile;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "persons"]
/// Referenced by PublicHealthProfile
/// Referenced by Trip
/// Need to add disaggregated data on vulnerable populations
/// and do this ethically.
pub struct Person {
    pub id: Uuid,
    pub family_name: String,
    pub given_name: String,
    pub additional_names: Option<Vec<String>>,
    pub birth_date: NaiveDate,
    pub gender: String,

    pub travel_document_id: String,
    pub travel_document_issuer_id: Uuid, // Country

    pub travel_group_id: Uuid,
    
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
    pub created_at: NaiveDateTime,
}


// Non Graphql
impl Person {
    pub fn create(conn: &PgConnection, person: &NewPerson) -> FieldResult<Person> {
        let res = diesel::insert_into(persons::table)
        .values(person)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, person: &NewPerson) -> FieldResult<Person> {
        let res = persons::table
        .filter(persons::travel_document_id.eq(&person.travel_document_id))
        .filter(persons::family_name.eq(&person.family_name))
        .filter(persons::travel_document_issuer_id.eq(&person.travel_document_issuer_id))
        .filter(persons::birth_date.eq(&person.birth_date))
        .distinct()
        .first(conn);
        
        let person = match res {
            Ok(p) => p,
            Err(e) => {
                // Person not found
                println!("{:?}", e);
                let p = Person::create(conn, person).expect("Unable to create person");
                p
            }
        };
        Ok(person)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[Object]
impl Person {
    
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn family_name(&self) -> FieldResult<String> {
        Ok(self.family_name.to_owned())
    }
    
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn given_name(&self) -> FieldResult<String> {
        Ok(self.given_name.to_owned())
    }
    
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn additional_names(&self) -> FieldResult<Option<Vec<String>>> {
        Ok(self.additional_names.to_owned())
    }
    
    pub async fn birth_date(&self) -> FieldResult<String> {
        Ok(self.birth_date.format("%Y-%m-%d").to_string())
    }
    
    pub async fn approved_access_level(&self) -> FieldResult<String> {
        Ok(self.approved_access_level.to_owned())
    }
    
    pub async fn approved_access_granularity(&self) -> FieldResult<String> {
        Ok(self.approved_access_granularity.to_owned())
    }
    
    pub async fn travel_document_issuer(&self, context: &Context<'_>) -> FieldResult<Country> {
        get_country_by_id(context, self.travel_document_issuer_id)
    }
    
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    /// This is personally identifiable information and can only be accessed
    /// by Analyst or Admin roles.
    pub async fn travel_document_id(&self) -> FieldResult<String> {
        Ok(self.travel_document_id.to_owned())
    }
    
    pub async fn public_health_profile(&self, context: &Context<'_>) -> FieldResult<PublicHealthProfile> {
        let conn = get_connection_from_context(context);
        
        let res = public_health_profiles::table
        .filter(public_health_profiles::person_id.eq(self.id))
        .first(&conn);
        
        graphql_translate(res)
    }
    
    pub async fn trips(&self, context: &Context<'_>) -> FieldResult<Vec<Trip>> {
        let conn = get_connection_from_context(context);
        
        let res = trips::table.
        filter(trips::person_id.eq(self.id))
        .order_by(trips::arrival_time)
        .order_by(trips::person_id)
        .load::<Trip>(&conn);
        
        graphql_translate(res)
    }
    
    pub async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "persons"]
pub struct NewPerson {
    pub family_name: String,
    pub given_name: String,
    pub additional_names: Option<Vec<String>>,
    pub birth_date: NaiveDate,
    pub gender: String,

    pub travel_document_id: String,
    pub travel_document_issuer_id: Uuid, // Country

    pub travel_group_id: Uuid,

    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
}

impl NewPerson {

    pub fn new(
        family_name: String,
        given_name: String,
        additional_names: Option<Vec<String>>,
        birth_date: NaiveDate,
        gender: String,
        travel_document_id: String,
        travel_document_issuer_id: Uuid, // Country
        travel_group_id: Uuid,
        approved_access_level: String, // AccessLevel
        approved_access_granularity: String,
    ) -> Self {
        NewPerson {
            family_name,
            given_name,
            additional_names,
            birth_date,
            gender,
            travel_document_id,
            travel_document_issuer_id,
            travel_group_id,
            approved_access_level,
            approved_access_granularity,
        }
    }

    pub fn fake(travel_document_issuer_id: Uuid, travel_group_id: Uuid) -> NewPerson {

        let mut rng = thread_rng();
        let random_year = rng.gen_range(1945..2002);
        let random_month = rng.gen_range(1..13);
        let random_day = rng.gen_range(1..29);

        let dob: NaiveDate = Utc.ymd(random_year, random_month, random_day).naive_utc();
        
        NewPerson {
            family_name: "Doe".to_string(),
            given_name: "Jane".to_string(),
            additional_names: None,
            birth_date: dob,
            gender: "female".to_string(),
            travel_document_id: "HDFSHFKJHD372840".to_string(),
            travel_document_issuer_id,
            travel_group_id,
            approved_access_level: "medical_records".to_string(),
            approved_access_granularity: "aggregated".to_string(),
        }
    }
}
