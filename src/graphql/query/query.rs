use async_graphql::*;

use crate::graphql::query::{CapabilityQuery, PersonQuery, TeamQuery, OrganizationQuery, UserQuery, RoleQuery};

use super::{PublicationQuery, TaskQuery, WorkQuery};

#[derive(Default, MergedObject)]
pub struct Query(
    CapabilityQuery,
    PersonQuery,
    TeamQuery,
    OrganizationQuery,
    UserQuery,
    RoleQuery,
    PublicationQuery,
    TaskQuery,
    WorkQuery,
);