use async_graphql::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::{Capability, NewCapability, CapabilityLevel};
use crate::common_utils::{UserRole,
    is_operator, RoleGuard};
// use rdkafka::producer::FutureProducer;
// use crate::kafka::send_message;

#[derive(Default)]
pub struct CapabilityMutation;

// Mutation

#[Object]
impl CapabilityMutation {

    #[graphql(
        name = "createCapability", 
        guard = "RoleGuard::new(UserRole::Operator)",
        visible = "is_operator",
    )]
    pub async fn create_capability(
        &self,
        _context: &Context<'_>,
        data: NewCapability,
    ) -> Result<Capability> {
        
        let capability = Capability::create(&data)?;

        Ok(capability)
    }

    #[graphql(
        name = "updateCapability", 
        guard = "RoleGuard::new(UserRole::Operator)",
        visible = "is_operator",
    )]
    /// An operator may update a capability.
    /// To update a validated_level, you must include the Uuid of the validator and their 
    /// current validated level. If the validator's level is equal or greater than the 
    /// level they are validating, the system will update validated_level.
    /// Need to update Capability to also track the validator_uuid 
    pub async fn update_capability(
        &self,
        _context: &Context<'_>,
        data: CapabilityData,
    ) -> Result<Capability> {
        
        let mut capability = Capability::get_by_id(&data.id)?;

        if let Some(s) = data.self_identified_level {
            capability.self_identified_level = s;
        };

        if let Some(s) = data.validated_level {
            capability.validated_level = Some(s);
        }

        if let Some(s) = data.retired_at {
            capability.retired_at = Some(s);
        };
        
        capability.update()

    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
#[graphql(complex)]
/// InputObject for Capability with Option fields - only include the ones you want to update
/// Capabilities will be updated regularly based on validations to a person's capability
/// Note that name_en and name_fr are linked to and derived from a skill and cannot be updated independently
pub struct CapabilityData {
    pub id: Uuid,
    pub self_identified_level: Option<CapabilityLevel>,
    pub validated_level: Option<CapabilityLevel>,
    pub retired_at: Option<NaiveDateTime>,
}