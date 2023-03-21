use async_graphql::*;

// use rdkafka::producer::FutureProducer;
// use crate::kafka::send_message;

use crate::graphql::mutation::{UserMutation, PersonMutation, 
    RoleMutation, CapabilityMutation};

#[derive(MergedObject, Default)]
pub struct Mutation(
    UserMutation, 
    PersonMutation,
    RoleMutation,
    CapabilityMutation,
);