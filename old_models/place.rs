use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl};
use async_graphql::*;
use uuid::Uuid;
use std::collections::HashMap;

use crate::graphql::graphql_translate;
use crate::models::Country;
use crate::schema::*;
use crate::get_country_by_id;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub country_id: Uuid,
}

impl Place {
    pub fn create(conn: &PgConnection, place: &NewPlace) -> FieldResult<Self> {
        let res = diesel::insert_into(places::table)
            .values(place)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Place> {
        let res = places::table
            .load::<Place>(conn)
            .expect("Unable to load countries");

        let mut new_map: HashMap<Uuid, Place> = HashMap::new();
        for v in res {
            new_map.insert(v.id, v);
        };

        new_map 
    }
}

#[Object]
impl Place {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id)
    }

    pub async fn name(&self) -> FieldResult<String> {
        Ok(self.name.to_owned())
    }

    pub async fn country(&self, context: &Context<'_>) -> FieldResult<Country> {

        get_country_by_id(context, self.country_id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
#[table_name = "places"]
pub struct NewPlace {
    pub place_name: String,
    pub country_id: Uuid,
}

impl NewPlace {
    pub fn new(place_name: String, country_id: Uuid) -> Self {
        NewPlace { place_name, country_id }
    }
}