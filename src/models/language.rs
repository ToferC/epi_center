use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use rand::{
    distributions::{Standard, Distribution}, 
    Rng
};

use async_graphql::*;

use crate::database::connection;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, AsChangeset, SimpleObject, PartialEq)]
#[table_name = "language_datas"]
/// A representation of a persons' language proficiency
pub struct LanguageData {
    pub id: Uuid,
    pub person_id: Uuid,
    pub language_name: LanguageName,
    pub reading: Option<LanguageLevel>,
    pub writing: Option<LanguageLevel>,
    pub speaking: Option<LanguageLevel>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::LanguageName"]
/// A selection of languages (to be expanded)
pub enum LanguageName {
    English,
    French,
    Arabic,
    Chinese,
    Spanish,
    German,
    Japanese,
    Korean,
    Italian,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::LanguageLevel"]
/// Government of Canada language proficiency ratings
/// A == basic, B == beginner, C == intermediate, E = advanced or exempt
/// X == no proficiency
pub enum LanguageLevel {
    A,
    B,
    C,
    E,
    X,
}

impl Distribution<LanguageLevel> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LanguageLevel {
        match rng.gen_range(0..=10) {
            0 => LanguageLevel::X,
            1..=3 => LanguageLevel::A,
            4..=6 => LanguageLevel::B,
            7..=8 => LanguageLevel::C,
            9..=10 => LanguageLevel::E,
            _ => LanguageLevel::X,
        }
    }
}

impl LanguageData {
    pub fn create(language: &NewLanguageData) -> Result<LanguageData> {

        let mut conn = connection()?;

        let res = diesel::insert_into(language_datas::table)
            .values(language)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn batch_create(language_datas: Vec<NewLanguageData>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(language_datas::table)
            .values(language_datas)
            .execute(&mut conn)?;
        
        Ok(res)
    }

    pub fn get_by_person_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = language_datas::table.filter(language_datas::person_id.eq(id))
            .load::<LanguageData>(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = language_datas::table.load::<LanguageData>(&mut conn)?;
        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(language_datas::table)
            .filter(language_datas::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "language_datas"]
/// Represents an insertable Language
pub struct NewLanguageData {
    pub person_id: Uuid,
    pub language_name: LanguageName,
    pub reading: Option<LanguageLevel>,
    pub writing: Option<LanguageLevel>,
    pub speaking: Option<LanguageLevel>,
}

impl NewLanguageData {
    pub fn new(
        person_id: Uuid,
        language_name: LanguageName,
        reading: Option<LanguageLevel>,
        writing: Option<LanguageLevel>,
        speaking: Option<LanguageLevel>,
    ) -> Self {
        NewLanguageData {
            person_id,
            language_name,
            reading,
            writing,
            speaking,
        }
    }

    pub fn dummy(person_id: Uuid, language_name: LanguageName) -> Self {
        let reading: LanguageLevel = rand::random();
        let writing: LanguageLevel = rand::random();
        let speaking: LanguageLevel = rand::random();

        NewLanguageData {
            person_id,
            language_name,
            reading: Some(reading),
            writing: Some(writing),
            speaking: Some(speaking),
        }
    }
}