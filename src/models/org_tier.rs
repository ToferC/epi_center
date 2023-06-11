use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods, PgTextExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::database::connection;
use crate::schema::*;

use super::{Organization, Person, OrgOwnership, SkillDomain, Team};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[diesel(table_name = org_tiers)]
#[diesel(belongs_to(Organization))]
/// Represents an organizational level starting at the top (CEO or President's office) as 0
/// and then increasing in tier number as you go deeper into the organization.
/// Used to model an organizational hierarchy independent of people
pub struct OrgTier {
    pub id: Uuid,

    #[graphql(visible = false)]
    pub organization_id: Uuid, // Organization
    pub tier_level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub primary_domain: SkillDomain,

    #[graphql(visible = false)]
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

#[ComplexObject]
impl OrgTier {

    pub async fn organization(&self) -> Result<Organization> {
        Organization::get_by_id(&self.organization_id)
    }

    pub async fn parent_organization_tier(&self) -> Result<Option<OrgTier>> {
        match self.parent_tier {
            Some(id) => Ok(Some(OrgTier::get_by_id(&id)?)),
            None => Ok(None),
        }
    }

    pub async fn child_organization_tier(&self) -> Result<Vec<OrgTier>> {
        OrgTier::get_child_org_tiers(&self.id)
    }

    pub async fn owner(&self) -> Result<Person> {
        let org_tier_ownership = OrgOwnership::get_by_org_tier_id(&self.id).unwrap();

        Person::get_by_id(&org_tier_ownership.owner_id)
    }

    pub async fn teams(&self) -> Result<Vec<Team>> {
        Team::get_by_org_tier_id(&self.id)
    }
}

// Non Graphql
impl OrgTier {
    pub fn create(org_tier: &NewOrgTier) -> Result<OrgTier> {
        let mut conn = connection()?;

        let res = diesel::insert_into(org_tiers::table)
            .values(org_tier)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(org_tier: &NewOrgTier) -> Result<OrgTier> {
        let mut conn = connection()?;

        let res = org_tiers::table
        .filter(org_tiers::name_en.eq(&org_tier.name_en))
        .distinct()
        .first(&mut conn);
        
        let org_tier = match res {
            Ok(p) => p,
            Err(e) => {
                // OrgTier not found
                if e.to_string() == "NotFound" {
                    println!("{:?}", e);
                }
                let p = OrgTier::create(org_tier).expect("Unable to create org_tier");
                p
            }
        };
        Ok(org_tier)
    }

    pub fn get_all() -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;
        let res = org_tiers::table
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;
        let res = org_tiers::table
            .limit(count)
            .order(org_tiers::tier_level)
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<OrgTier> {
        let mut conn = connection()?;

        let res = org_tiers::table
            .filter(org_tiers::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &str) -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;

        let res = org_tiers::table
            .filter(org_tiers::name_en.ilike(&name).or(org_tiers::name_fr.ilike(format!("%{}%", name))))
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_org_id(id: &Uuid) -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;

        let res = org_tiers::table
            .filter(org_tiers::organization_id.eq(id))
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_top_by_org_id(id: &Uuid) -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;

        let res = org_tiers::table
            .filter(org_tiers::organization_id.eq(id))
            .filter(org_tiers::parent_tier.is_null())
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_child_org_tiers(id: &Uuid) -> Result<Vec<OrgTier>> {
        let mut conn = connection()?;

        let res: Vec<Self> = org_tiers::table
            .filter(org_tiers::parent_tier.eq(id))
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_ids(ids: &Vec<Uuid>) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let org_tier_ownership = org_tiers::table
            .filter(org_tiers::id.eq_any(ids))
            .load::<OrgTier>(&mut conn)?;

        Ok(org_tier_ownership)
    }
    
    pub fn update(&self) -> Result<OrgTier> {
        let mut conn = connection()?;

        let res = diesel::update(org_tiers::table)
            .filter(org_tiers::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[diesel(table_name = org_tiers)]
pub struct NewOrgTier {
    pub organization_id: Uuid, // Organization
    pub tier_level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub primary_domain: SkillDomain,
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
}

impl NewOrgTier {

    pub fn new(
        organization_id: Uuid, // Organization
        tier_level: i32,
        name_en: String,
        name_fr: String,
        primary_domain: SkillDomain,
        parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    ) -> Self {
        NewOrgTier {
            organization_id,
            tier_level,
            name_en,
            name_fr,
            primary_domain,
            parent_tier,
        }
    }
}
