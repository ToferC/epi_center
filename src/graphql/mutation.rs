use std::str::FromStr;

use async_graphql::*;
use uuid::Uuid;

use crate::models::{InsertableUser, LoginQuery,
    User, UserData, create_token, decode_token,
    verify_password, UserUpdate, hash_password, Person, NewPerson, PersonData};
use crate::common_utils::{UserRole,
    is_operator,
    is_admin, RoleGuard};
// use rdkafka::producer::FutureProducer;
// use crate::kafka::send_message;

pub struct Mutation;

// Mutation Example

#[Object]
impl Mutation {

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



    /*
    #[graphql(
        name = "PILQuery", 
        guard = "RoleGuard::new(UserRole::Operator)",
        visible = "is_operator",
    )]
    /// Receives a Vec<TravelData> containing details from a group of travllers
    /// and returns a Vec<TravelResponse> containing public health direction for the BSO
    /// relating to entry to Canada for public health reasons and referrals to mandatory
    /// random testing. Also includes IDs for Person, Trip, QuarantinePlan
    /// for further mutations.
    pub async fn travel_data_response(
        &self,
        context: &Context<'_>,
        data: Vec<TravelData>,
    ) -> FieldResult<Vec<PILResponse>> {

        let cbsa_id = context.data_opt::<Uuid>().expect("Unable to parse CBSA ID");

        let mut responses_to_cbsa: Vec<PILResponse> = Vec::new();

        let travel_group_id = Uuid::new_v4();

        for traveller in data {
            let response = traveller.process(&context, travel_group_id, *cbsa_id)
                .await?
                .into();
                
            responses_to_cbsa.push(response);

            /* 
            // Create Kafka producer and send message for subscription service
            let producer = context
                .data::<FutureProducer>()
                .expect("Can't get Kafka producer");
            */

            // Sent ArriveCan messages to Kafka
            let arrivecan_message = serde_json::to_string(&traveller)
                .expect("Can't serialize ArriveCan PIL message");

            /* 
            // Remove subscription until we set up Kafka service
            println!("Sending ArriveCan PIL Message to Subscription");
            send_message(producer, "arrivecan_pil", arrivecan_message, "CBSA".to_string()).await;
            */
        };        
        
        Ok(responses_to_cbsa)
    }
*/
    #[graphql(
        name = "createUser",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub async fn create_user(
        &self,
        _context: &Context<'_>,
        user_data: UserData,
    ) -> FieldResult<User> {
        let new_user = InsertableUser::from(user_data);

        let created_user = User::create(new_user);

        created_user
    }

    #[graphql(
        name = "updateUser",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub async fn update_user(
        &self,
        _context: &Context<'_>,
        user_data: UserUpdate,
    ) -> FieldResult<User> {
        let mut target_user = User::get_by_id(&user_data.id)?;

        if let Some(s) = user_data.name {
            target_user.name = s;
        };

        if let Some(s) = user_data.email {
            target_user.email = s;
        };

        if let Some(s) = user_data.password {
            target_user.hash = hash_password(&s)?;
        };

        if let Some(s) = user_data.role {
            target_user.role = s;
        };

        let updated_user = target_user.update();

        updated_user
    }

    pub async fn sign_in(
        &self,
        _context: &Context<'_>,
        input: LoginQuery,
    ) -> Result<String, Error> {
        let maybe_user = User::get_by_email(&input.email).ok();

        if let Some(user) = maybe_user {

            if let Ok(matching) = verify_password(&user.hash.to_string(), &input.password) {
                if matching {
                    let role = UserRole::from_str(user.role.as_str())
                        .expect("Cannot convert &str to UserRole");

                    // Return the token which would be accepted by the ArriveCan 
                    // app and used to authenticate actions
                    let token = create_token(user.id.to_string(), role);

                    println!("JWT: {}\nData{:?}", &token, decode_token(&token));

                    return Ok(token);
                }
            }
        }

        Err(Error::new("Can't authenticate a user"))
    }
}