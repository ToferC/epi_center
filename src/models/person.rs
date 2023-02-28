use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::models::{Organization};

use crate::common_utils::{
    is_analyst, RoleGuard, UserRole};

use crate::graphql::graphql_translate;

use crate::database::connection;
use crate::schema::*;

use super::Role;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = persons)]
/// Referenced by Team
/// Referenced by ReportingRelationship
pub struct Person {
    pub id: Uuid,
    pub user_id: Uuid,
    pub family_name: String,
    pub given_name: String,

    pub organization_id: Uuid, // Organization 
    pub peoplesoft_id: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}


// Non Graphql
impl Person {
    pub fn create(person: &NewPerson) -> FieldResult<Person> {
        let mut conn = connection()?;
        let res = diesel::insert_into(persons::table)
        .values(person)
        .get_result(&mut conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(person: &NewPerson) -> FieldResult<Person> {
        let mut conn = connection()?;
        let res = persons::table
        .filter(persons::family_name.eq(&person.family_name))
        .distinct()
        .first(&mut conn);
        
        let person = match res {
            Ok(p) => p,
            Err(e) => {
                // Person not found
                println!("{:?}", e);
                let p = Person::create(person).expect("Unable to create person");
                p
            }
        };
        Ok(person)
    }

    pub fn get_by_id(id: &Uuid) -> FieldResult<Person> {
        let mut conn = connection()?;

        let res = persons::table
            .filter(persons::id.eq(id))
            .first(&mut conn);

        graphql_translate(res)
    }
    
    pub fn update(&self) -> FieldResult<Self> {
        let mut conn = connection()?;

        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[Object]
impl Person {

    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    pub async fn family_name(&self) -> Result<String> {
        Ok(self.family_name.to_owned())
    }
    
    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    pub async fn given_name(&self) -> Result<String> {
        Ok(self.given_name.to_owned())
    }

    pub async fn organization(&self) -> Result<Organization> {
        
        Organization::get_by_id(&self.organization_id)
    }

    pub async fn roles(&self) -> Result<Vec<Role>> {
        Role::get_by_person_id(self.id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Referenced by Roles, TeamOwnership, OrgOwnership
#[diesel(table_name = persons)]
pub struct NewPerson {
    pub user_id: Uuid,
    pub family_name: String,
    pub given_name: String,
    pub organization_id: Uuid, // Organization
    pub peoplesoft_id: String,
}

impl NewPerson {

    pub fn new(
        user_id: Uuid,
        family_name: String,
        given_name: String,
        organization_id: Uuid, // Organization
        peoplesoft_id: String,
    ) -> Self {
        NewPerson {
            user_id,
            family_name,
            given_name,
            organization_id,
            peoplesoft_id,
        }
    }
}
