/* 
use diesel::PgConnection;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::prelude::*;
use chrono::Utc;
use rand::Rng;
use async_graphql::*;
// use rdkafka::{producer::FutureProducer};

use crate::graphql::{get_connection_from_context};
use crate::schema::*;
// use crate::kafka::send_message;
// use crate::get_or_create_country_by_name;

use crate::models::{NewPerson, Person};

// Code for messaging layer

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Queryable)]
/// A struct representing the API response for a specific traveller
/// Likely to be part of a Vec<TravelResponse>
pub struct PILResponse {
    pub id: Uuid,
    pub post_status: String,
    pub trip_id: Uuid,
    pub person_id: Uuid,
    pub cbsa_id: String,
    pub response_code: String,
    pub random_testing_referral: bool,
    pub quarantine_required: bool,
    pub date_time: NaiveDateTime,
    pub details: Option<String>,
}

impl PILResponse {
    pub fn create(conn: &PgConnection, travel_response: &NewPILResponse) -> FieldResult<PILResponse> {
        let res = diesel::insert_into(travel_responses::table)
            .values(travel_response)
            .get_result(conn);

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, SimpleObject)]
#[table_name = "travel_responses"]
/// A struct representing the API response for a specific traveller
/// Likely to be part of a Vec<TravelResponse>
/// Will also be added to database for audit and data purposes.
pub struct NewPILResponse {
    pub post_status: String,
    pub trip_id: Uuid,
    pub person_id: Uuid,
    pub cbsa_id: String,
    pub response_code: String,
    pub random_testing_referral: bool,
    pub quarantine_required: bool,
    pub date_time: NaiveDateTime,
    pub details: Option<String>,
}

impl NewPILResponse {
    pub fn new(
            post_status: String,
            trip_id: Uuid,
            person_id: Uuid,
            cbsa_id: String,
            response_code: String,
            random_testing_referral: bool,
            quarantine_required: bool,
            details: String,
    ) -> Self {

        let details = if details != "".to_string() {
            Some(details)
        } else {
            None
        };

        NewPILResponse {
            post_status,
            trip_id,
            person_id,
            cbsa_id,
            response_code,
            random_testing_referral,
            quarantine_required,
            date_time: Utc::now().naive_utc(),
            details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject, SimpleObject)]
#[graphql(input_name = "TravelDataInput")]
/// Struct for data submitted by CBSA on query of ArriveCan.
/// Likely to be part of a Vec<TravelData> that will be 
/// combined into a TravelGroup.
pub struct TravelData {

    // Person (traveller) data, providing id if traveller is already
    // in the system, or a NewPerson struct if not
    pub family_name: String,
    pub given_name: String,
    /// Optional vector of strings
    pub additional_names: Option<Vec<String>>,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub travel_document_id: String,
    pub travel_document_issuer: String, // Country
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String,

    // Trip data
    pub trip_provider: String,
    /// Optional String. If None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    /// Optional String for booking ID if applicable
    pub booking_id: Option<String>,
    pub travel_mode: String,

    pub origin_name: String,
    pub origin_country_name: String,
    pub destination_name: String,
    pub destination_country_name: String,

    pub travel_intent: String,
    /// Optional NaiveDateTime
    pub scheduled_departure_time: Option<NaiveDateTime>,
    /// Optional NaiveDateTime
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    /// Optional NaiveDateTime
    pub departure_time: Option<NaiveDateTime>,
    /// Optional NaiveDateTime
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,

    /// PublicHealthProfile data
    /// May or may not have this detail. Will create if not.
    pub smart_healthcard_pk: Option<String>,

    /// Vec of SlimVaccinations as Vaccinations
    /// May already be in system. Likely need to do a validation
    /// By date_time and provider constraint
    /// Otherwise, add new vaccinations to system
    pub vaccination_required: bool,
    pub vaccinations: Option<Vec<SlimVaccination>>,

    /// CovidTest
    /// Very likely to be new each time we interact, but not
    /// necessarily for frequent travellers or workers
    pub covid_test_required: bool,
    pub covid_test: Option<SlimCovidTest>,

    /// QuarantinePlan
    /// Also likely to be unique for each traveller.
    /// Possible to be required or not required based on 
    /// environment. Consider making optional.
    pub quarantine_plan_required: bool,
    pub quarantine_plan: Option<SlimQuarantinePlan>,

    // Time of api post
    pub date_time: NaiveDateTime,

    // CBSA Officer ID
    pub cbsa_officer_id: String,
}

impl TravelData {
    pub async fn process(
            &self, 
            context: &Context<'_>,
            travel_group_id: Uuid,
            cbsa_id: Uuid,
        ) -> FieldResult<PILResponse> {

        // Connect to PostgresPool
        let conn = get_connection_from_context(context);

        /* Remove Kafka service
        // Create Kafka producer and send message for subscription service
        let producer = context
            .data::<FutureProducer>()
            .expect("Can't get Kafka producer");
        */

        // Identify country        
        let country = get_or_create_country_by_name(context, self.travel_document_issuer.to_owned())?;

        // Identify or create person
        let new_person = NewPerson::new(
            self.family_name.to_owned(),
            self.given_name.to_owned(),
            self.additional_names.to_owned(),
            self.birth_date,
            self.gender.to_owned(),
            self.travel_document_id.to_owned(),
            country.id, // Country
            travel_group_id,
            self.approved_access_level.to_owned(), // AccessLevel
            self.approved_access_granularity.to_owned(),
        );

        let person = Person::get_or_create(&conn, &new_person)?;

        // Add Trip Information
        let new_trip = NewTrip::new(
            context,
            self.trip_provider.to_owned(),
            self.travel_identifier.to_owned(),
            self.booking_id.to_owned(),
            self.travel_mode.to_owned(),
            self.origin_name.to_owned(),
            self.origin_country_name.to_owned(),
            self.destination_name.to_owned(),
            self.destination_country_name.to_owned(),
            self.travel_intent.to_owned(),
            self.scheduled_departure_time,
            self.scheduled_arrival_time,
            self.departure_time,
            self.arrival_time,
            self.trip_state.to_owned(),
            travel_group_id,
            person.id,
        );

        let trip = Trip::create(&conn, &new_trip).expect("Unable to create trip");

        // Add or get PublicHealthProfile
        let profile = NewPublicHealthProfile::new(
            person.id,
            self.smart_healthcard_pk.clone(),
        );

        let public_health_profile = PublicHealthProfile::get_or_create(&conn, &profile)
            .expect("Unable to find or create profile");

        // Add vaccinations
        if let Some(vaccinations) = &self.vaccinations {
            let mut vaccination_history: Vec<Vaccination> = Vec::new();
            
            for slim_v in vaccinations {
    
                let nv = NewVaccination::from(
                    context, 
                    &slim_v, 
                    public_health_profile.id).expect("Unable to create NewVaccination");
    
                let v = Vaccination::get_or_create(&conn, &nv)
                    .expect("Unable to find or create vaccination");
                vaccination_history.push(v);
            };
        }
        
        // Add Covid-Test if exists
        if let Some(t) = &self.covid_test {
            // Add CovidTest -> update to get or create
            
            let new_test = NewCovidTest::from(
            public_health_profile.id, 
            &t);
    
            let _covid_test = CovidTest::create(&conn, &new_test)
                .expect("Unable to create new covid test");
        }

        // Add QuarantinePlan if exists
        if let Some(p) = &self.quarantine_plan {
            // Add CovidTest -> update to get or create

            let new_plan = NewQuarantinePlan::from(
            public_health_profile.id,
            &p
            );
    
            let _quarantine_plan = QuarantinePlan::create(&conn, &new_plan)
                .expect("Unable to create new quarantine plan");
        }

        // KAFKA

        // Sent Person messages to Kafka
        let person_message = serde_json::to_string(&person)
            .expect("Can't serialize Person");

        /* Remove Subscription Service until we set up Kafka service
        println!("Sending Person Message to Subscription");
        send_message(producer, "people", person_message, "CBSA".to_string()).await;

        // Sent Trip messages to Kafka
        let trip_message = serde_json::to_string(&trip)
            .expect("Can't serialize Trip");

        println!("Sending Message to Subscription");
        send_message(producer, "trips", trip_message, "CBSA".to_string()).await;
        */

        // Call people_data_api
        // Determine if traveller is referred for mandatory testing
        let mut rng = rand::thread_rng();

        let mut random_testing_referral = false;

        if rng.gen::<f64>() < MANDATORY_TESTING_RATE {
            random_testing_referral = true;
        };

        // Build TravelResponse
        let new_tr = NewPILResponse::new(
            "OK".to_string(),
            trip.id,
            person.id,
            cbsa_id.to_string(),
            "I".to_string(),
            random_testing_referral,
            false,
            "None".to_string()
        );

        let travel_response = PILResponse::create(&conn, &new_tr)?;
        Ok(travel_response)
    }
}
*/