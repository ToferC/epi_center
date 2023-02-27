use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::graphql::{graphql_translate, get_connection_from_context};
use super::{Trip, Person};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, PartialOrd, Identifiable)]
#[serde(rename_all= "snake_case")]
#[table_name = "travel_groups"]
/// People travelling together in a single voyage
/// May include several trips per person with distinct origins
/// and destinations.
/// Referenced through Person, Trip and links to voyage
pub struct TravelGroup {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}

#[Object]
impl TravelGroup {
    pub async fn id(&self) -> Uuid {
        self.id
    }

    pub async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub async fn trips(&self, context: &Context<'_>) -> FieldResult<Vec<Trip>> {
        let conn = get_connection_from_context(context);

        let res = trips::table.
            filter(trips::travel_group_id.eq(self.id))
            .order_by(trips::arrival_time)
            .order_by(trips::person_id)
            .load::<Trip>(&conn);

        graphql_translate(res)
    }

    pub async fn people(&self, context: &Context<'_>) -> FieldResult<Vec<Person>> {
        let conn = get_connection_from_context(context);

        let res = persons::table.
            filter(persons::travel_group_id.eq(self.id))
            .load::<Person>(&conn);

        graphql_translate(res)
    }
}

/// Non-Graphql
impl TravelGroup {
    pub fn create_travel_group(conn: &PgConnection, travel_group: &NewTravelGroup) -> FieldResult<TravelGroup> {
        
        let res = diesel::insert_into(travel_groups::table)
            .values(travel_group)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Insertable, Debug, InputObject)]
#[table_name = "travel_groups"]
pub struct NewTravelGroup {
    pub id: Uuid,
}

impl NewTravelGroup {
    pub fn new() -> Self {
        NewTravelGroup {
            id: Uuid::new_v4()
        }
    }
}