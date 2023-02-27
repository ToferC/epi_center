use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::config_variables::DATE_FORMAT;
use crate::schema::*;
use crate::graphql::{graphql_translate, get_connection_from_context};
use crate::models::{Place, Person};
use crate::common_utils::{is_analyst, RoleGuard, Role};
use crate::{get_place_by_id, get_or_create_country_by_name, get_or_create_place_by_name_and_country_id};

/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "trips"]
pub struct Trip {
    pub id: Uuid,
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[Object]
impl Trip {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.into())
    }

    pub async fn trip_provider(&self) -> FieldResult<String> {
        Ok(self.trip_provider.to_owned())
    }

    pub async fn travel_mode(&self) -> FieldResult<String> {
        Ok(self.travel_mode.to_owned())
    }

    pub async fn travel_identifier(&self) -> FieldResult<String> {
        match &self.travel_identifier {
            Some(i) => Ok(i.to_owned()),
            None => Ok("None".to_string()),
        }
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn booking_id(&self) -> FieldResult<String> {
        match &self.booking_id {
            Some(i) => Ok(i.to_owned()),
            None => Ok("None".to_string()),
        }
    }

    pub async fn scheduled_departure_time(&self) -> FieldResult<String> {
        match self.scheduled_arrival_time {
            Some(t) => Ok(t.format(DATE_FORMAT).to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub async fn scheduled_arrival_time(&self) -> FieldResult<String> {
        match self.scheduled_departure_time {
            Some(t) => Ok(t.format(DATE_FORMAT).to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub async fn departure_time(&self) -> FieldResult<String> {
        match self.arrival_time {
            Some(t) => Ok(t.format(DATE_FORMAT).to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub async fn arrival_time(&self) -> FieldResult<String> {
        match self.departure_time {
            Some(t) => Ok(t.format(DATE_FORMAT).to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub async fn travel_intent(&self) -> FieldResult<String> {
        Ok(self.travel_intent.to_owned())
    }

    pub async fn trip_state(&self) -> FieldResult<String> {
        Ok(self.trip_state.to_owned())
    }

    pub async fn person(&self, context: &Context<'_>) -> FieldResult<Person> {

        let conn = get_connection_from_context(context);

        let res = persons::table.
            filter(persons::id.eq(self.person_id))
            .first(&conn);

        graphql_translate(res)
    }

    pub async fn origin(&self, context: &Context<'_>) -> FieldResult<Place> {

        get_place_by_id(context, self.origin_place_id)
    }

    pub async fn destination(&self, context: &Context<'_>) -> FieldResult<Place> {

        get_place_by_id(context, self.destination_place_id)
    }

    pub async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

// Non Graphql
impl Trip {
    pub fn create(conn: &PgConnection, trip: &NewTrip) -> FieldResult<Trip> {
        let res = diesel::insert_into(trips::table)
            .values(trip)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Insertable, Debug, InputObject)]
#[table_name = "trips"]
pub struct NewTrip {
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
}

impl<'a> NewTrip {
    pub fn default() -> Self {

        let depart: NaiveDateTime = Utc::now().naive_utc() + Duration::hours(8);
        let arrive: NaiveDateTime = Utc::now().naive_utc() - Duration::hours(4);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: Uuid::new_v4(),
            destination_place_id: Uuid::new_v4(), 
            
            travel_intent: "Entry".to_string(), 

            scheduled_departure_time: Some(depart + Duration::hours(4)), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            
            trip_state: "planned".to_string(),
            travel_group_id: Uuid::new_v4(),
            person_id: Uuid::new_v4(),
        }
    }

    pub fn fake(
        travel_group_id: &Uuid,
        person_id: &Uuid,
        origin_place_id: &Uuid,
        destination_place_id: &Uuid
    ) -> Self 
    {
        let depart: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: origin_place_id.to_owned(), 
            
            destination_place_id: destination_place_id.to_owned(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart + Duration::hours(4)), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 

            trip_state: "active".to_string(),
            travel_group_id: travel_group_id.to_owned(),
            person_id: person_id.to_owned(),
        }
    }

    pub fn new(
        context: &Context<'_>,
        trip_provider: String,
        travel_identifier: Option<String>,
        booking_id: Option<String>,
        travel_mode: String,
        origin_place_name: String,
        origin_country_name: String,
        destination_place_name: String,
        destination_country_name: String,
        travel_intent: String,
        scheduled_departure_time: Option<NaiveDateTime>,
        scheduled_arrival_time: Option<NaiveDateTime>,
        departure_time: Option<NaiveDateTime>,
        arrival_time: Option<NaiveDateTime>,
        trip_state: String,
        travel_group_id: Uuid,
        person_id: Uuid,
    ) -> Self 
    {        
        let origin_country = get_or_create_country_by_name(context, origin_country_name)
            .expect("Unable to find or create country");

        let origin = get_or_create_place_by_name_and_country_id(
            context,
            origin_place_name, origin_country.id)
            .expect("Unable to get or create origin country");

        let destination_country = get_or_create_country_by_name(context, destination_country_name)
            .expect("Unable to find or create country");

        let destination = get_or_create_place_by_name_and_country_id(
            context,
            destination_place_name, destination_country.id)
            .expect("Unable to get or create origin country");


        NewTrip { 
            trip_provider,
            travel_identifier,
            booking_id,
            travel_mode,
            origin_place_id: origin.id,
            destination_place_id: destination.id,
            travel_intent,
            scheduled_departure_time,
            scheduled_arrival_time,
            departure_time,
            arrival_time,
            trip_state,
            travel_group_id,
            person_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TripState {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelIntent {
    Entry,
    Exit,
    Transit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Should get this info from an API
/// Could be a struct with company info, contact info, API key, etc.
pub struct TravelProvider {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelMode {
    // Strings annotate the types of travel
    // I.e., "Rail", "Bus", "Private Vehicle", "Charter"
    Air(String),
    Sea(String),
    Land(String),
}