use chrono::prelude::*;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl};
use uuid::Uuid;
use std::collections::HashMap;

use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "vaccines"]
pub struct Vaccine {
    pub id: Uuid,
    pub vaccine_name: String,
    pub manufacturer: String,
    pub vaccine_type: String,
    pub required_doses: i32,
    pub approved: bool,
    pub approved_on: NaiveDate,
    pub details: String,
}

#[Object]
impl Vaccine {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub async fn name(&self) -> FieldResult<String> {
        Ok(self.vaccine_name.clone())
    }

    pub async fn manufacturer(&self) -> FieldResult<String> {
        Ok(self.manufacturer.clone())
    }

    pub async fn vaccine_type(&self) -> FieldResult<String> {
        Ok(self.vaccine_type.clone())
    }

    pub async fn required_doses(&self) -> FieldResult<i32> {
        Ok(self.required_doses)
    }

    pub async fn approved(&self) -> FieldResult<bool> {
        Ok(self.approved)
    }

    pub async fn approved_on(&self) -> FieldResult<String> {
        Ok(self.approved_on.format("%Y-%m-%d").to_string())
    }

    pub async fn details(&self) -> FieldResult<String> {
        Ok(self.details.clone())
    }
}

impl Vaccine {
    pub fn create(conn: &PgConnection, vaccine: &NewVaccine) -> FieldResult<Vaccine> {
        let res = diesel::insert_into(vaccines::table)
            .values(vaccine)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Vaccine> {
        let res = vaccines::table
            .load::<Vaccine>(conn)
            .expect("Unable to load countries");

        let mut new_map: HashMap<Uuid, Vaccine> = HashMap::new();
        for v in res {
            new_map.insert(v.id, v);
        };

        new_map 
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "vaccines"]
pub struct NewVaccine {
    pub vaccine_name: String,
    pub manufacturer: String,
    pub vaccine_type: String,
    pub required_doses: i32,
    pub approved: bool,
    pub approved_on: NaiveDate,
    pub details: String,
}

impl NewVaccine {
    pub fn new(
        vaccine_name: String,
        manufacturer: String,
        vaccine_type: String,
        required_doses: i32,
        approved: bool,
        approved_on: NaiveDate,
        details: String,
    ) -> Self {
        NewVaccine {
            vaccine_name,
            manufacturer,
            vaccine_type,
            required_doses,
            approved,
            approved_on,
            details,
        }
    }
}