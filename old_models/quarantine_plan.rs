use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use async_graphql::*;

use crate::common_utils::{is_analyst, RoleGuard, Role};
use crate::graphql::{graphql_translate, get_connection_from_context};
use crate::schema::*;
use crate::models::PostalAddress;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "quarantine_plans"]
/// Referenced by compliance check
/// References public_health_profile
/// Primary object for QuarantinePlan
pub struct QuarantinePlan {
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDate,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: Uuid, // PostalAddress
    pub active: bool,
}

#[Object]
impl QuarantinePlan {
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub async fn date_created(&self) -> FieldResult<String> {
        Ok(self.date_created.format("%Y-%m-%d").to_string())
    }

    pub async fn quarantine_required(&self) -> FieldResult<bool> {
        Ok(self.quarantine_required)
    }

    pub async fn confirmation_no_vulnerable(&self) -> FieldResult<bool> {
        Ok(self.confirmation_no_vulnerable)
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn postal_address_id(&self) -> FieldResult<Uuid> {
        Ok(self.postal_address_id)
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn quarantine_address(&self, context: &Context<'_>) -> FieldResult<PostalAddress> {
        let conn = get_connection_from_context(context);

        let res = postal_addresses::table
            .filter(postal_addresses::id.eq(self.postal_address_id))
            .first(&conn);

        graphql_translate(res)
    }

    pub async fn active(&self) -> FieldResult<bool> {
        Ok(self.active)
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn check_in_history(&self, context: &Context<'_>) -> FieldResult<Vec<CheckInResult>> {
        let conn = get_connection_from_context(context);

        let res = check_in_results::table
            .filter(check_in_results::quarantine_plan_id.eq(self.id))
            .load::<CheckInResult>(&conn);

        graphql_translate(res)
    }
}

impl QuarantinePlan {
    pub fn create(conn: &PgConnection, plan: &NewQuarantinePlan) -> FieldResult<QuarantinePlan> {
        let res = diesel::insert_into(quarantine_plans::table)
            .values(plan)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject, Insertable)]
#[table_name = "quarantine_plans"]
/// Insertable for QuarantinePlan
pub struct NewQuarantinePlan {
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDate,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: Uuid, // PostalAddress
    pub active: bool,
}

impl NewQuarantinePlan {
    pub fn new(
        public_health_profile_id: Uuid,
        date_created: NaiveDate,
        quarantine_required: bool,
        confirmation_no_vulnerable: bool,
        postal_address_id: Uuid, // PostalAddress
        active: bool,
    ) -> Self {
        NewQuarantinePlan {
            public_health_profile_id,
            date_created,
            quarantine_required,
            confirmation_no_vulnerable,
            postal_address_id,
            active,
        }
    }

    pub fn from(
        public_health_profile_id: Uuid,
        slim_plan: &SlimQuarantinePlan,
    ) -> Self {
        NewQuarantinePlan {
            public_health_profile_id,
            date_created: slim_plan.date_created,
            quarantine_required: false, // default
            confirmation_no_vulnerable: slim_plan.confirmation_no_vulnerable,
            postal_address_id: slim_plan.postal_address_id,
            active: false, // default
        } 
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject, SimpleObject)]
#[graphql(input_name = "SlimQuarantinePlanInput")]
/// Light data needed to create a quarantine plan
pub struct SlimQuarantinePlan {
    pub date_created: NaiveDate,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: Uuid, // PostalAddress
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, Queryable)]
/// Object for required check-ins for persons required to quarantine.
/// Designed for automated or human-conducted check-ins and verifications.
pub struct CheckInResult {
    pub id: Uuid,
    pub quarantine_plan_id: Uuid,
    pub verifier_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

impl CheckInResult {
    pub fn create(conn: &PgConnection, check_in: &NewCheckInResult) -> FieldResult<CheckInResult> { 
        let res = diesel::insert_into(check_in_results::table)
            .values(check_in)
            .get_result(conn);
        
        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject, Insertable)]
#[table_name = "check_in_results"]
pub struct NewCheckInResult {
    pub quarantine_plan_id: Uuid,
    pub verifier_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

impl NewCheckInResult {
    pub fn new(
        quarantine_plan_id: Uuid,
        verifier_id: Uuid,
        date_time: NaiveDateTime,
        check_in_complete: bool,
    ) -> Self {
        NewCheckInResult {
            quarantine_plan_id,
            verifier_id,
            date_time,
            check_in_complete,
        }
    }
}