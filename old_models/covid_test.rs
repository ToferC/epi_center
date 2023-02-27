use chrono::prelude::*;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl};
use uuid::Uuid;

use crate::config_variables::{DATE_FORMAT};
use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "covid_tests"]
pub struct CovidTest {
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub test_name: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[Object]
impl CovidTest {
    pub async fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id)
    }

    pub async fn test_name(&self) -> FieldResult<String> {
        Ok(self.test_name.to_owned())
    }

    pub async fn test_type(&self) -> FieldResult<String> {
        Ok(self.test_type.to_owned())
    }

    pub async fn date_taken(&self) -> FieldResult<String> {
        Ok(self.date_taken.format(DATE_FORMAT).to_string())
    }

    pub async fn test_result(&self) -> FieldResult<bool> {
        Ok(self.test_result)
    }
}

impl CovidTest {
    pub fn create(conn: &PgConnection, test: &NewCovidTest) -> FieldResult<CovidTest> {
        let res = diesel::insert_into(covid_tests::table)
            .values(test)
            .get_result(conn);
        
        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject, Insertable)]
#[table_name = "covid_tests"]
pub struct NewCovidTest {
    pub public_health_profile_id: Uuid,
    pub test_name: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

impl NewCovidTest {
    pub fn new(
        public_health_profile_id: Uuid,
        test_name: String,
        test_type: String, // TestType
        date_taken: NaiveDateTime,
        test_result: bool,
    ) -> Self {
        NewCovidTest {
            public_health_profile_id,
            test_name,
            test_type,
            date_taken,
            test_result,
        }
    }

    pub fn from(
        public_health_profile_id: Uuid,
        slim_test: &SlimCovidTest,
    ) -> Self {
        NewCovidTest {
            public_health_profile_id,
            test_name: slim_test.test_name.to_owned(),
            test_type: slim_test.test_type.to_owned(),
            date_taken: slim_test.date_taken,
            test_result: slim_test.test_result,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject, SimpleObject)]
#[graphql(input_name = "SlimCovidTestInput")]
pub struct SlimCovidTest {
    pub test_name: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TestType {
    Molecular,
    Other,
}