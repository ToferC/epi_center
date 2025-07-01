use std::collections::HashMap;

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
use crate::models::Capability;

use crate::database::connection;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, AsChangeset, SimpleObject, PartialEq)]
#[graphql(complex)]
#[table_name = "skills"]
/// Should get this from an API or have standard data
/// Now pre-loaded as prt of context
pub struct Skill {
    pub id: Uuid,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub domain: SkillDomain,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

#[ComplexObject]
impl Skill {
    pub async fn capabilities(&self) -> Result<Vec<Capability>> {
        Capability::get_by_skill_id(self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::SkillDomain"]
pub enum SkillDomain {
    Combat,
    Strategy,
    Intelligence,
    InformationTechnology,
    HumanResources,
    Finance,
    Communications,
    Administration,
    Engineering,
    Medical,
    Management,
    Leadership,
    JointOperations,
}

impl Distribution<SkillDomain> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkillDomain {
        match rng.gen_range(0..20) {
            0..=3 => SkillDomain::Combat,
            4..=5 => SkillDomain::Strategy,
            6 => SkillDomain::Intelligence,
            7 => SkillDomain::InformationTechnology,
            8 => SkillDomain::HumanResources,
            9 => SkillDomain::Finance,
            10 => SkillDomain::Communications,
            11 => SkillDomain::Administration,
            12..=14 => SkillDomain::Engineering,
            15..=16 => SkillDomain::Medical,
            17 => SkillDomain::Management,
            18 => SkillDomain::Leadership,
            19 => SkillDomain::JointOperations,
            _ => SkillDomain::Strategy,
        }
    }
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

    pub fn get_name_by_id(id: &Uuid) -> Result<String> {
        let mut conn = connection()?;

        let res = skills::table.filter(skills::id.eq(id))
            .select(skills::name_en)
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

    pub fn get_top_skill_id_by_name(name: String) -> Result<Uuid> {
        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::name_en.ilike(format!("%{}%", name)).or(skills::name_fr.ilike(format!("%{}%", name))))
            .select(skills::id)
            .first::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_ids_by_name(name: String) -> Result<Vec<Uuid>> {
        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::name_en.ilike(format!("%{}%", name)).or(skills::name_fr.ilike(format!("%{}%", name))))
            .select(skills::id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_ids_by_domain(domain: SkillDomain) -> Result<Vec<Uuid>> {
        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::domain.eq(domain))
            .select(skills::id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_domain(domain: SkillDomain) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = skills::table
            .filter(skills::domain.eq(domain))
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

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "skills"]
/// Represents an insertable Skill
pub struct NewSkill {
    name_en: String,
    name_fr: String,
    description_en: String,
    description_fr: String,
    domain: SkillDomain,
}

impl NewSkill {
    pub fn new(
        name_en: String,
        name_fr: String,
        domain: SkillDomain,
    ) -> Self {
        NewSkill {
            name_en,
            name_fr,
            description_en: "Default EN".to_string(),
            description_fr: "Default FR".to_string(),
            domain,
        }
    }
}