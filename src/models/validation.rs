use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::database::connection;
use crate::models::{CapabilityLevel};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[diesel(table_name = validations)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Capability))]
// Represents ownership of a team by a person
/// Other people's validations of an individuals Capability
pub struct Validation {
    pub id: Uuid,
    pub validator_id: Uuid, // Person
    pub capability_id: Uuid, // Capability
    pub validated_level: CapabilityLevel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Non Graphql
impl Validation {
    pub fn create(validation: &NewValidation) -> Result<Validation> {

        let mut conn = connection()?;

        let res = diesel::insert_into(validations::table)
        .values(validation)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(validation: &NewValidation) -> Result<Validation> {

        let mut conn = connection()?;

        let res = validations::table
            .filter(validations::validator_id.eq(&validation.validator_id))
            .filter(validations::capability_id.eq(&validation.capability_id))
            .distinct()
            .first(&mut conn);
        
        let validation = match res {
            Ok(p) => p,
            Err(e) => {
                // Validation not found
                println!("{:?}", e);
                let p = Validation::create(validation).expect("Unable to create validation");
                p
            }
        };
        Ok(validation)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = validations::table
            .filter(validations::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = validations::table
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = validations::table
            .limit(count)
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_capability_id(id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = validations::table
            .filter(validations::capability_id.eq(id))
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_validator_id(id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = validations::table
            .filter(validations::validator_id.eq(id))
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    
    pub fn update(&self) -> Result<Self> {

        let mut conn = connection()?;

        let res = diesel::update(validations::table)
        .filter(validations::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ValidatedLevel {
    capability_level: CapabilityLevel,
    average: f32,
}

impl ValidatedLevel {
    pub fn new(
        capability_level: CapabilityLevel, 
        average: f32,
    ) -> Self {
        ValidatedLevel { 
            capability_level,
            average,
         }
    }

    pub fn return_validated_level(validations: &Vec<Validation>) -> Result<ValidatedLevel> {
        let mut results = Vec::new();
    
        for v in validations {
            let n = match v.validated_level {
                CapabilityLevel::Desired => 0,
                CapabilityLevel::Novice => 100,
                CapabilityLevel::Experienced => 200,
                CapabilityLevel::Expert => 300,
                CapabilityLevel::Specialist => 400,
            };
            results.push(n);
        };
    
        let average_value = results.iter().sum::<i64>() / results.len() as i64;
    
        let cap = match &average_value {
            00..=080 => CapabilityLevel::Desired,
            81..=180 => CapabilityLevel::Novice,
            181..=280 => CapabilityLevel::Experienced,
            281..=380 => CapabilityLevel::Expert,
            381..=480 => CapabilityLevel::Specialist,
            _ => CapabilityLevel::Desired,
        };
    
        let res = ValidatedLevel::new(
            cap,
            average_value as f32 / 100.0,
        );
    
        Ok(res)
    }
}



#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Linked from HealthProfile
/// Linked to Trip
#[diesel(table_name = validations)]
pub struct NewValidation {
    pub validator_id: Uuid, // Person
    pub capability_id: Uuid, // Capability
    pub validated_level: CapabilityLevel,
}

impl NewValidation {

    pub fn new(
        validator_id: Uuid, // Person
        capability_id: Uuid, // Capability
        validated_level: CapabilityLevel,
    ) -> Self {
        NewValidation {
           validator_id,
           capability_id,
           validated_level,
        }
    }
}
