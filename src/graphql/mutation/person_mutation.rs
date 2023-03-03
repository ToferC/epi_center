use std::str::FromStr;

use async_graphql::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::{InsertableUser, LoginQuery,
    User, UserData, create_token, decode_token,
    verify_password, UserUpdate, hash_password, Person, NewPerson};
use crate::common_utils::{UserRole,
    is_operator,
    is_admin, RoleGuard};
use crate::schema::persons;
// use rdkafka::producer::FutureProducer;
// use crate::kafka::send_message;

#[derive(Default)]
pub struct PersonMutation;

// Mutation Example

#[Object]
impl PersonMutation {

    #[graphql(
        name = "createPerson", 
        guard = "RoleGuard::new(UserRole::Operator)",
        visible = "is_operator",
    )]
    pub async fn create_person(
        &self,
        _context: &Context<'_>,
        person_data: NewPerson,
    ) -> Result<Person> {
        
        let person = Person::create(&person_data)?;

        Ok(person)
    }

    #[graphql(
        name = "updatePerson", 
        guard = "RoleGuard::new(UserRole::Operator)",
        visible = "is_operator",
    )]
    pub async fn update_person(
        &self,
        _context: &Context<'_>,
        person_data: PersonData,
    ) -> Result<Person> {
        
        let mut person = Person::get_by_id(&person_data.id)?;

        if let Some(id) = person_data.user_id {
            person.user_id = id;
        };

        if let Some(s) = person_data.family_name {
            person.family_name = s;
        };

        if let Some(s) = person_data.given_name {
            person.given_name = s;
        };

        if let Some(s) = person_data.email {
            person.email = s;
        };

        if let Some(s) = person_data.phone {
            person.phone = s;
        };

        if let Some(s) = person_data.work_address {
            person.work_address = s;
        };

        if let Some(s) = person_data.city {
            person.city = s;
        };

        if let Some(s) = person_data.province {
            person.province = s;
        };

        if let Some(s) = person_data.postal_code {
            person.postal_code = s;
        };

        if let Some(s) = person_data.organization_id {
            person.organization_id = s;
        };

        if let Some(s) = person_data.peoplesoft_id {
            person.peoplesoft_id = s;
        };

        if let Some(s) = person_data.orcid_id {
            person.orcid_id = s;
        };

        if let Some(s) = person_data.updated_at {
            person.updated_at = s;
        };

        if let Some(s) = person_data.retired_at {
            person.retired_at = Some(s);
        };


        Ok(person)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, InputObject)]
#[graphql(complex)]
#[diesel(table_name = persons)]
/// InputObject for Person with Option fields - only include the ones you want to update
pub struct PersonData {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub work_address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,

    pub organization_id: Option<Uuid>, // Organization 
    pub peoplesoft_id: Option<String>,
    pub orcid_id: Option<String>,

    pub updated_at: Option<NaiveDateTime>,
    pub retired_at: Option<NaiveDateTime>,
}