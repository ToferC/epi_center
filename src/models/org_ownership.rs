use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::database::connection;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = org_tier_ownerships)]
#[diesel(belongs_to(Person))]
/// Represents a relationship between a person (owner) and an organizational tier
/// Will be used to inform approvals and organizational authority
pub struct OrgOwnership {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub org_tier_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

// Non Graphql
impl OrgOwnership {
    pub fn create(org_tier_ownership: &NewOrgOwnership) -> Result<OrgOwnership> {
        let mut conn = connection()?;

        let res = diesel::insert_into(org_tier_ownerships::table)
            .values(org_tier_ownership)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(org_tier_ownership: &NewOrgOwnership) -> Result<OrgOwnership> {
        let mut conn = connection()?;

        let res = org_tier_ownerships::table
            .filter(org_tier_ownerships::org_tier_id.eq(&org_tier_ownership.org_tier_id))
            .distinct()
            .first(&mut conn);
        
        let org_tier_ownership = match res {
            Ok(p) => p,
            Err(e) => {
                // OrgOwnership not found
                println!("{:?}", e);
                let p = OrgOwnership::create(org_tier_ownership).expect("Unable to create org_tier_ownership");
                p
            }
        };
        Ok(org_tier_ownership)
    }

    pub fn find_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let org_tier_ownerships = org_tier_ownerships::table
            .load::<OrgOwnership>(&mut conn)?;
        Ok(org_tier_ownerships)
    }

    pub fn get_by_id(id: Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let org_tier_ownership = org_tier_ownerships::table
            .filter(org_tier_ownerships::id.eq(id))
            .first(&mut conn)?;
        Ok(org_tier_ownership)
    }

    pub fn get_by_org_tier_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = org_tier_ownerships::table
            .filter(org_tier_ownerships::org_tier_id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_org_tier_ids_by_owner_id(id: &Uuid) -> Result<Vec<Uuid>> {
        let mut conn = connection()?;

        let res = org_tier_ownerships::table
            .filter(org_tier_ownerships::owner_id.eq(id))
            .select(org_tier_ownerships::org_tier_id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(org_tier_ownerships::table)
            .filter(org_tier_ownerships::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject, InputObject)]
#[diesel(table_name = org_tier_ownerships)]
pub struct NewOrgOwnership {
    pub owner_id: Uuid,
    pub org_tier_id: Uuid,
}

impl NewOrgOwnership {

    pub fn new(
        owner_id: Uuid,
        org_tier_id: Uuid,
    ) -> Self {
        NewOrgOwnership {
            owner_id,
            org_tier_id,
        }
    }
}
