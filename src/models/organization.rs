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
use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, SimpleObject)]
#[table_name = "organizations"]
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

impl Organization {
    pub fn create(organization: &NewOrganization) -> Result<Organization> {
        let mut conn = connection()?;

        let res = diesel::insert_into(organizations::table)
            .values(organization)
            .get_result(&mut conn);

        graphql_translate(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Organization> {
        let mut conn = connection()?;

        let res = organizations::table.filter(organizations::id.eq(id))
            .first(&mut conn);

        graphql_translate(res)
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

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
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