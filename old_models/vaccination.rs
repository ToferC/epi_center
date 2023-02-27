use chrono::prelude::*;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;

use crate::common_utils::{RoleGuard, Role, is_analyst};

use crate::config_variables::DATE_FORMAT;
use crate::models::{Place, Vaccine};
use crate::graphql::graphql_translate;
use crate::schema::*;
use crate::{get_or_create_country_by_name, get_vaccine_by_id, 
    get_vaccine_by_name, get_place_by_id, get_or_create_place_by_name_and_country_id};


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Queryable, Identifiable)]
// Will assess Vaccine History against health rules engine
pub struct Vaccination {
    pub id: Uuid,
    pub vaccine_id: Uuid,
    pub dose_provider: String,
    pub location_provided_id: Uuid, // Place
    pub provided_on: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

// Graphql
#[Object]
impl Vaccination {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub async fn vaccine(&self, context: &Context<'_>) -> FieldResult<Vaccine> {
        get_vaccine_by_id(context, self.vaccine_id)
    }

    pub async fn location_provided(&self, context: &Context<'_>) -> FieldResult<Place> {
        get_place_by_id(context, self.location_provided_id)
    }

    pub async fn provided_on(&self) -> FieldResult<String> {
        Ok(self.provided_on.format(DATE_FORMAT).to_string())
    }

    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn public_health_profile_id(&self) -> FieldResult<Uuid> {
        Ok(self.public_health_profile_id)
    }
}

impl Vaccination {
    pub fn create(conn: &PgConnection, vaccination: &NewVaccination) -> FieldResult<Vaccination> {
        let res = diesel::insert_into(vaccinations::table)
            .values(vaccination)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_or_create(conn: &PgConnection, vaccination: &NewVaccination) -> FieldResult<Vaccination> {
        let res = vaccinations::table
            .filter(vaccinations::public_health_profile_id.eq(&vaccination.public_health_profile_id))
            .filter(vaccinations::provided_on.eq(&vaccination.provided_on))
            .filter(vaccinations::dose_provider.eq(&vaccination.dose_provider))
            .distinct()
            .first(conn);

        let vaccination = match res {
            Ok(v) => v,
            Err(e) => {
                // Vaccination not found
                println!("{:?}", e);
                let v = Vaccination::create(conn, vaccination).expect("Unable to create person");
                v
            }
        };
        Ok(vaccination)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable, InputObject)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "vaccinations"]
pub struct NewVaccination {
    pub vaccine_id: Uuid,
    pub dose_provider: String,
    pub location_provided_id: Uuid, // Place
    pub provided_on: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

impl NewVaccination {
    pub fn new(
        context: &Context<'_>,
        vaccine_name: String,
        dose_provider: String,
        location_provided_id: Uuid, // Place
        provided_on: NaiveDateTime,
        public_health_profile_id: Uuid,
    ) -> FieldResult<Self> {

        let vaccine = get_vaccine_by_name(
            context,
            vaccine_name)
            .expect("Unable to find vaccine by name");

        Ok(NewVaccination {
            vaccine_id: vaccine.id,
            dose_provider,
            location_provided_id,
            provided_on,
            public_health_profile_id,
        })
    }

    pub fn fake(
        vaccine_id: Uuid,
        dose_provider: String,
        location_provided_id: Uuid, // Place
        provided_on: NaiveDateTime,
        public_health_profile_id: Uuid,
    ) -> Self {

        NewVaccination {
            vaccine_id,
            dose_provider,
            location_provided_id,
            provided_on,
            public_health_profile_id,
        }
    }

    pub fn from(
        context: &Context<'_>,
        slim_vaccination: &SlimVaccination, 
        public_health_profile_id: Uuid
    ) -> FieldResult<Self> {
        let provided_country = get_or_create_country_by_name(context, slim_vaccination.country_provided.to_owned())
            .expect("Unable to find or create country");

        let location_provided = get_or_create_place_by_name_and_country_id(
            context,
            slim_vaccination.location_provided.to_owned(), provided_country.id)
            .expect("Unable to get or create origin country");

            let vaccine = get_vaccine_by_name(
                context,
                slim_vaccination.vaccine_name.to_owned())
            .expect("Unable to find vaccine by name");

        Ok(
            NewVaccination {
                vaccine_id: vaccine.id,
                dose_provider: slim_vaccination.dose_provider.to_owned(),
                location_provided_id: location_provided.id,
                provided_on: slim_vaccination.provided_on,
                public_health_profile_id,
            }
        )

    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, InputObject, SimpleObject)]
#[graphql(input_name = "SlimVaccinationInput")]
/// Basic text data used to create a Vaccination object
pub struct SlimVaccination {
    pub vaccine_name: String,
    pub dose_provider: String,
    pub location_provided: String,
    pub country_provided: String, // Place
    pub provided_on: NaiveDateTime,
}