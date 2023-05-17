use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::common_utils::{RoleGuard, UserRole, is_admin};
use crate::database::connection;
use crate::models::{CapabilityLevel};

use super::{Person, Capability};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[diesel(table_name = validations)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Capability))]
#[graphql(complex)]
// Represents ownership of a team by a person
/// Other people's validations of an individuals Capability
pub struct Validation {
    pub id: Uuid,
    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    pub validator_id: Uuid, // Person
    pub capability_id: Uuid, // Capability
    pub validated_level: CapabilityLevel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Validation {
    pub async fn validator(&self) -> Result<Person> {
        Person::get_by_id(&self.validator_id)
    }
}

// Non Graphql
impl Validation {
    pub fn create(validation: &NewValidation) -> Result<Validation> {

        let mut conn = connection()?;

        let res:Validation = diesel::insert_into(validations::table)
            .values(validation)
            .get_result(&mut conn)?;

        // Update Capability with validation

        let mut capability = Capability::get_by_id(&res.capability_id)?;

        capability.update_from_validation(&res.validated_level)?;
        
        Ok(res)
    }

    pub fn batch_create(validations: Vec<NewValidation>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(validations::table)
            .values(validations)
            .execute(&mut conn)?;
        
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

        let res: Validation = diesel::update(validations::table)
            .filter(validations::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;

        let mut capability = Capability::get_by_id(&res.capability_id)?;

        capability.update_from_validation(&res.validated_level)?;
        
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

    pub fn get_value_from_capability_level(capability_level: &CapabilityLevel) -> i64 {
            let n = match capability_level {
                CapabilityLevel::Desired => 0,
                CapabilityLevel::Novice => 100,
                CapabilityLevel::Experienced => 200,
                CapabilityLevel::Expert => 300,
                CapabilityLevel::Specialist => 400,
            };
            
            n
    }

    pub fn get_capability_level_from_value(value: &i64) -> CapabilityLevel {

        let cap = match value {
            00..=070 => CapabilityLevel::Desired,
            71..=170 => CapabilityLevel::Novice,
            171..=270 => CapabilityLevel::Experienced,
            271..=370 => CapabilityLevel::Expert,
            371..=470 => CapabilityLevel::Specialist,
            _ => CapabilityLevel::Desired,
        };

        cap
    }

    pub fn return_validated_level(validation_values: &Vec<i64>) -> Result<ValidatedLevel> {
    
        let average_value = validation_values.iter().sum::<i64>() / validation_values.len() as i64;
    
        let cap = match &average_value {
            00..=070 => CapabilityLevel::Desired,
            71..=170 => CapabilityLevel::Novice,
            171..=270 => CapabilityLevel::Experienced,
            271..=370 => CapabilityLevel::Expert,
            371..=470 => CapabilityLevel::Specialist,
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
