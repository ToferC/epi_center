use std::str::FromStr;

use async_graphql::*;
use uuid::Uuid;

use crate::models::{InsertableUser, LoginQuery,
    User, UserData, create_token, decode_token,
    verify_password, UserUpdate, hash_password, Person, NewPerson};
use crate::common_utils::{UserRole,
    is_operator,
    is_admin, RoleGuard};
// use rdkafka::producer::FutureProducer;
// use crate::kafka::send_message;

use crate::graphql::mutation::{UserMutation, PersonMutation};

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, PersonMutation);