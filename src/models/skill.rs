use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
//use juniper::{Result};
use uuid::Uuid;

use async_graphql::*;

use crate::database::connection;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "skills"]
/// Represents an insertable Skill
pub struct NewSkill {
    name_en: String,
    name_fr: String,
    description_en: String,
    description_fr: String,
}

impl NewSkill {
    pub fn new(
        name_en: String,
        name_fr: String,
    ) -> Self {
        NewSkill {
            name_en,
            name_fr,
            description_en: "Default EN".to_string(),
            description_fr: "Default FR".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, AsChangeset, SimpleObject)]
#[table_name = "skills"]
/// Should get this from an API or have standard data
/// Now pre-loaded as prt of context
pub struct Skill {
    pub id: Uuid,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

impl Skill {
    pub fn create(skill: &NewSkill) -> Result<Skill> {

        let mut conn = connection()?;

        let res = diesel::insert_into(skills::table)
            .values(skill)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Skill> {
        let mut conn = connection()?;

        let res = skills::table.filter(skills::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = skills::table.load::<Skill>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_name(name: String) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::name_en.ilike(format!("%{}%", name)).or(skills::name_fr.ilike(format!("%{}%", name))))
            .load::<Skill>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_ids(ids: &Vec<Uuid>) -> Result<Vec<Self>> {

        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::id.eq_any(ids))
            .load::<Skill>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(skills::table)
            .filter(skills::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;
        
        Ok(res)
    }

    pub fn load_into_hash() -> HashMap<Uuid, Skill> {
        let mut conn = connection().expect("Unable to connect to DB");

        let res = skills::table
            .load::<Skill>(&mut conn)
            .expect("Unable to load skills");

        let mut skills: HashMap<Uuid, Skill> = HashMap::new();
        for c in res {
            skills.insert(c.id, c);
        };

        skills 
    }
}