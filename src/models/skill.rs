use std::collections::HashMap;

use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
//use juniper::{FieldResult};
use uuid::Uuid;

use async_graphql::*;

use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
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
        description_en: String,
        description_fr: String,
    ) -> Self {
        NewSkill {
            name_en,
            name_fr,
            acroynm_en,
            description_fr,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, SimpleObject)]
#[table_name = "skills"]
/// Should get this from an API or have standard data
/// Now pre-loaded as prt of context
pub struct Skill {
    pub id: Uuid,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
}

impl Skill {
    pub fn create(conn: &PgConnection, skill: &NewSkill) -> FieldResult<Skill> {
        let res = diesel::insert_into(skills::table)
            .values(skill)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_by_id(conn: &PgConnection, id: &Uuid) -> FieldResult<Skill> {
        let res = skills::table.filter(skills::id.eq(id))
            .first(conn);

        graphql_translate(res)
    }

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Skill> {
        let res = skills::table
            .load::<Skill>(conn)
            .expect("Unable to load skills");

        let mut skills: HashMap<Uuid, Skill> = HashMap::new();
        for c in res {
            skills.insert(c.id, c);
        };

        skills 
    }
}