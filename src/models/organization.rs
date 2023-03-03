use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::dsl::count;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
//use juniper::{Result};
use uuid::Uuid;

use async_graphql::*;

use crate::database::connection;
use crate::schema::*;

use crate::models::{CapabilityCount, CapabilityLevel, Affiliation, SkillDomain};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, SimpleObject)]
#[table_name = "organizations"]
#[graphql(complex)]
/// Should get this from an API or have standard data
/// Now pre-loaded as prt of context
pub struct Organization {
    pub id: Uuid,
    pub name_en: String,
    pub name_fr: String,
    pub acroynm_en: String,
    pub acronym_fr: String,
    pub org_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

#[ComplexObject]
impl Organization {
    async fn get_affiliations(&self) -> Result<Vec<Affiliation>> {
        Affiliation::get_by_organization_id(self.id)
    }
    
    async fn get_capability_counts(&self) -> Result<Vec<CapabilityCount>> {
        let mut conn = connection().unwrap();

        let res: Vec<(String, SkillDomain, CapabilityLevel, i64)> = capabilities::table
            .filter(capabilities::organization_id.eq(self.id))
            .group_by((capabilities::domain, capabilities::self_identified_level, capabilities::name_en))
            .select((capabilities::name_en, capabilities::domain, capabilities::self_identified_level, count(capabilities::id)))
            .order_by((capabilities::name_en, capabilities::self_identified_level))
            .load::<(String, SkillDomain, CapabilityLevel, i64)>(&mut conn)?;

    // Convert res into CapabilityCountStruct
    let mut counts: Vec<CapabilityCount> = Vec::new();

    for r in res {
        let count = CapabilityCount::from(r);
        counts.push(count);
    }

    Ok(counts)
    }
}

impl Organization {
    pub fn create(organization: &NewOrganization) -> Result<Organization> {
        let mut conn = connection()?;

        let res = diesel::insert_into(organizations::table)
            .values(organization)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Organization> {
        let mut conn = connection()?;

        let res = organizations::table.filter(organizations::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Organization>> {
        let mut conn = connection()?;

        let res = organizations::table
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Organization>> {
        let mut conn = connection()?;

        let res = organizations::table
            .limit(count)
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: String) -> Result<Vec<Organization>> {
        let mut conn = connection()?;

        let res = organizations::table
            .filter(organizations::name_en.ilike(format!("%{}%", name)).or(organizations::name_fr.ilike(format!("%{}%", name))))
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    pub fn load_into_hash() -> HashMap<Uuid, Organization> {
        let mut conn = connection().expect("Unable to make connection");

        let res = organizations::table
            .load::<Organization>(&mut conn)
            .expect("Unable to get organizations");

        let mut organizations: HashMap<Uuid, Organization> = HashMap::new();
        for c in res {
            organizations.insert(c.id, c);
        };

        organizations 
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[table_name = "organizations"]
/// Represents an insertable Organization
pub struct NewOrganization {
    pub name_en: String,
    pub name_fr: String,
    pub acronym_en: String,
    pub acronym_fr: String,
    pub org_type: String,
}

impl NewOrganization {
    pub fn new(
        name_en: String,
        name_fr: String,
        acronym_en: String,
        acronym_fr: String,
        org_type: String,

    ) -> Self {
        NewOrganization {
            name_en,
            name_fr,
            acronym_en,
            acronym_fr,
            org_type,
        }
    }
}