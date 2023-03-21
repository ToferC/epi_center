use async_graphql::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::{Person, NewPerson};
use crate::common_utils::{UserRole,
    is_operator, RoleGuard};
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
        data: NewPerson,
    ) -> Result<Person> {
        
        let person = Person::create(&data)?;

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
        data: PersonData,
    ) -> Result<Person> {
        
        let mut person = Person::get_by_id(&data.id)?;

        if let Some(id) = data.user_id {
            person.user_id = id;
        };

        if let Some(s) = data.family_name {
            person.family_name = s;
        };

        if let Some(s) = data.given_name {
            person.given_name = s;
        };

        if let Some(s) = data.email {
            person.email = s;
        };

        if let Some(s) = data.phone {
            person.phone = s;
        };

        if let Some(s) = data.work_address {
            person.work_address = s;
        };

        if let Some(s) = data.city {
            person.city = s;
        };

        if let Some(s) = data.province {
            person.province = s;
        };

        if let Some(s) = data.postal_code {
            person.postal_code = s;
        };

        if let Some(s) = data.organization_id {
            person.organization_id = s;
        };

        if let Some(s) = data.peoplesoft_id {
            person.peoplesoft_id = s;
        };

        if let Some(s) = data.orcid_id {
            person.orcid_id = s;
        };

        
        if let Some(s) = data.retired_at {
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