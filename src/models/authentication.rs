use crate::errors::CustomError;
use chrono::{Duration, prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{generate_unique_code, schema::{
    email_verification_code, password_reset_token}
};
use crate::database;

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{QueryDsl};

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, AsChangeset, Clone)]
#[table_name = "email_verification_code"]
pub struct EmailVerification {
    pub id: Uuid,
    pub email_address: String,
    pub activation_code: String,
    pub expires_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "email_verification_code"]
pub struct InsertableVerification {
    pub email_address: String,
    pub activation_code: String,
    pub expires_on: NaiveDateTime,
}

impl InsertableVerification {
    pub fn new(email_address: &String) -> Self {
        let expires_on = Utc::now().naive_utc() + Duration::minutes(30);
        let activation_code = generate_unique_code(5, false);

        InsertableVerification {
            email_address: email_address.to_owned(),
            activation_code,
            expires_on,
        }
    }
}

impl EmailVerification {
    pub fn create(e: &InsertableVerification) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let ev = diesel::insert_into(email_verification_code::table)
            .values(e)
            .on_conflict(email_verification_code::email_address)
            .do_update()
            .set((
                email_verification_code::activation_code.eq(&e.activation_code),
                email_verification_code::expires_on.eq(&e.expires_on),
            ))
            .get_result(&conn)?;
        Ok(ev)
    }

    pub fn find_by_email(email: &String) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let v = email_verification_code::table
            .filter(email_verification_code::email_address.eq(&email))
            .first(&conn)?;
        Ok(v)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let conn = database::connection()?;
        let res = diesel::delete(email_verification_code::table.filter(
            email_verification_code::id.eq(id)
        )).execute(&conn)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, AsChangeset, Clone)]
#[table_name = "password_reset_token"]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub email_address: String,
    pub reset_token: String,
    pub expires_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "password_reset_token"]
pub struct InsertablePasswordResetToken {
    pub email_address: String,
    pub reset_token: String,
    pub expires_on: NaiveDateTime,
}

impl InsertablePasswordResetToken {
    pub fn new(email_address: &String) -> Self {
        let expires_on = Utc::now().naive_utc() + Duration::minutes(30);
        let reset_token = generate_unique_code(36, false);

        InsertablePasswordResetToken {
            email_address: email_address.to_owned(),
            reset_token,
            expires_on,
        }
    }
}

impl PasswordResetToken {
    pub fn create(e: &InsertablePasswordResetToken) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let ev = diesel::insert_into(password_reset_token::table)
            .values(e)
            .on_conflict(password_reset_token::email_address)
            .do_update()
            .set((
                password_reset_token::reset_token.eq(&e.reset_token),
                password_reset_token::expires_on.eq(&e.expires_on),
            ))
            .get_result(&conn)?;
        Ok(ev)
    }

    pub fn find_by_token(token: &String) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let v = password_reset_token::table
            .filter(password_reset_token::reset_token.eq(&token))
            .first(&conn)?;
        Ok(v)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let conn = database::connection()?;
        let res = diesel::delete(password_reset_token::table.filter(
            password_reset_token::id.eq(id)
        )).execute(&conn)?;
        Ok(res)
    }
}