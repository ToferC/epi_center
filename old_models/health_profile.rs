//use juniper::FieldResult;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use async_graphql::*;
use crate::common_utils::{is_analyst, RoleGuard, Role};

use crate::models::{Vaccination,
    QuarantinePlan, CovidTest};
use crate::graphql::{graphql_translate, get_connection_from_context};
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable, Queryable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
pub struct PublicHealthProfile {
    pub id: Uuid,
    pub person_id: Uuid,
    pub smart_healthcard_pk: Option<String>,
}

// GraphQL Implementation
#[Object]
impl PublicHealthProfile {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    /// Returns the unique UID for the person associated to 
    /// the PublicHealthProfile.
    /// Authorized roles: Analyst, Admin
    pub async fn person_id(&self) -> FieldResult<Uuid> {
        Ok(self.person_id.clone())
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn smart_healthcard_pk(&self) -> FieldResult<String> {
        match &self.smart_healthcard_pk {
            Some(key) => Ok(key.to_owned()),
            None => Ok("NA".to_string())
        }
    }

    pub async fn vaccination_history(&self, context: &Context<'_>) -> FieldResult<Vec<Vaccination>> {
        let conn = get_connection_from_context(context);

        let res = vaccinations::table
            .filter(vaccinations::public_health_profile_id.eq(self.id))
            .load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    pub async fn testing_history(&self, context: &Context<'_>) -> FieldResult<Vec<CovidTest>> {
        let conn = get_connection_from_context(context);

        let res = covid_tests::table
            .filter(covid_tests::public_health_profile_id.eq(self.id))
            .load::<CovidTest>(&conn);

        graphql_translate(res)
    }

    pub async fn quarantine_plans(&self, context: &Context<'_>) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = get_connection_from_context(context);

        let res = quarantine_plans::table
            .filter(quarantine_plans::public_health_profile_id.eq(self.id))
            .load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }
}

impl PublicHealthProfile {
    pub fn create(conn: &PgConnection, profile: &NewPublicHealthProfile) -> FieldResult<PublicHealthProfile> { 
        let res = diesel::insert_into(public_health_profiles::table)
            .values(profile)
            .get_result(conn);
        
        graphql_translate(res)
    }

    pub fn get_or_create(conn: &PgConnection, profile: &NewPublicHealthProfile) -> FieldResult<PublicHealthProfile> {
        let res = public_health_profiles::table
            .filter(public_health_profiles::smart_healthcard_pk.eq(&profile.smart_healthcard_pk))
            .filter(public_health_profiles::person_id.eq(profile.person_id))
            .distinct()
            .first(conn);

        let profile = match res {
            Ok(p) => p,
            Err(e) => {
                // Profile not found
                println!("{:?}", e);
                let p = PublicHealthProfile::create(conn, profile).expect("Unable to create person");
                p
            }
        };
        Ok(profile)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "public_health_profiles"]
pub struct NewPublicHealthProfile {
    pub person_id: Uuid,
    pub smart_healthcard_pk: Option<String>,
}

impl NewPublicHealthProfile {
    pub fn new(
        person_id: Uuid,
        smart_healthcard_pk: Option<String>,
    ) -> Self {
        NewPublicHealthProfile {
            person_id,
            smart_healthcard_pk,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
//#[DieselType = "Access_level_enum"]
pub enum AccessLevelEnum {
    Adminstrator,
    Analyst,
    Employee,
    Research,
    Open,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}