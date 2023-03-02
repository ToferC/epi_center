use diesel::{RunQueryDsl, PgTextExpressionMethods};
use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, BoolExpressionMethods};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership, OrgOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityLevel, CapabilityCount};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {

    // People 
    #[graphql(name = "allPeople")]
    /// Accepts argument of "count" and returns a vector of {count} persons ordered by
    /// family name.D
    pub async fn all_people(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Person>> {

        Person::get_all()
    }

    #[graphql(name = "getPeople")]
    /// Accepts argument of "count" and returns a vector of {count} persons ordered by
    /// family name.D
    pub async fn get_people(
        &self, 
        _context: &Context<'_>,
        count: i64,
    ) -> Result<Vec<Person>> {

        Person::get_count(count)
    }

    #[graphql(name = "personById")]
    pub async fn person_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Person> {

        Person::get_by_id(&id)
    }

    #[graphql(name = "personByName")]
    pub async fn person_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Person>> {

        Person::get_by_name(&name)
    }

    // Capabilities

    pub async fn get_capabilities(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Capability>> {

        Capability::get_all()
    }

    pub async fn capability_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Capability> {

        Capability::get_by_id(&id)
    }

    pub async fn get_capabilities_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Capability>> {

        let skill_ids = Skill::get_skill_ids_by_name(name)?;

        Capability::get_by_skill_ids(skill_ids)
    }

    /// Return a count of the number of people who have a capability at each level of the capability
    pub async fn get_capability_counts(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_name(name)
    }

    // Skills

    pub async fn get_skills(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Skill>> {

        Skill::get_all()
    }

    pub async fn get_skill_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Skill> {

        Skill::get_by_id(&id)
    }

    pub async fn get_skill_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Skill>> {

        Skill::get_by_name(name)
    }

    // Teams
    #[graphql(name = "allTeams")]
    /// Returns a vector of all travel groups
    pub async fn all_teams(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Team>> {

        Team::get_all()
    }

    
    #[graphql(name = "teamByID")]
    /// Returns a specific travel group by its UUID
    pub async fn team_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Team> {

        Team::get_by_id(&id)
    }

    #[graphql(name = "teamByName")]
    pub async fn team_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Team>> {

        Team::get_by_name(name)
    }

    // Roles

    #[graphql(name = "getCountRole")]
    /// Accepts an argument of "count" and returns a vector of {count} role
    pub async fn get_count_role(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Role>> {
        
        Role::get_count(count)
    }

    #[graphql(name = "allRoles")]
    /// Returns a vector of all persons ordered by family name
    pub async fn all_roles(
        &self, 
        _context: &Context<'_>,) -> Result<Vec<Role>> {

        Role::get_all()
    }

    // Organizations

    #[graphql(name = "allOrganizations")]
    /// Returns a vector of all organizations
    pub async fn all_organizations(&self, _context: &Context<'_>) -> Result<Vec<Organization>> {
        
        Organization::get_all()
    }

    #[graphql(name = "getCountOrganizations")]
    /// Accepts argument "count" and returns a vector of {count} organizations
    pub async fn get_count_organizations(&self, _context: &Context<'_>, count: i64) -> Result<Vec<Organization>> {
        
        Organization::get_count(count)
    }

    #[graphql(name = "organizationByName")]
    pub async fn organization_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Organization>> {

        Organization::get_by_name(name)
    }

    // OrgTiers

    #[graphql(name = "allOrgTiers")]
    /// Returns a vector of all  org tiers
    pub async fn all_org_tiers(&self, _context: &Context<'_>) -> Result<Vec<OrgTier>> {

        OrgTier::get_all()
    }

    #[graphql(name = "getOrgTiers")]
    /// Accepts argument "count" and returns a vector of {count} org tiers
    pub async fn get_org_tiers(&self, _context: &Context<'_>, count: i64) -> Result<Vec<OrgTier>> {
        OrgTier::get_count(count)
    }

    #[graphql(name = "orgTierByName")]
    pub async fn org_tier_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<OrgTier>> {

        OrgTier::get_by_name(&name)
    }

    // TeamOwnerships

    #[graphql(name = "allTeamOwnership")]
    /// Returns a vector of all team ownerships
    pub async fn all_team_ownership_results(&self, context: &Context<'_>) -> Result<Vec<TeamOwnership>> {
        let mut conn = get_connection_from_context(context);

        let res = team_ownerships::table.load::<TeamOwnership>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "getTeamOwnership")]
    /// Accepts argument "count" and returns a vector of {count} team ownerships
    pub async fn get_team_ownership_results(&self, context: &Context<'_>, count: i64) -> Result<Vec<TeamOwnership>> {
        let mut conn = get_connection_from_context(context);

        let res = team_ownerships::table
            .limit(count)
            .load::<TeamOwnership>(&mut conn)?;

        Ok(res)
    }

    // Users / Admin

    #[graphql(
        name = "allUsers",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn all_users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let mut conn = get_connection_from_context(context);

        let res = users::table.load::<User>(&mut conn)?;

        Ok(res)
    }

    #[graphql(
        name = "getUserByEmail",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_email(&self, context: &Context<'_>, email: String) -> Result<User> {

        let res = User::get_by_email(&email)?;

        Ok(res)
    }

    #[graphql(
        name = "getUserById",
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_id(&self, context: &Context<'_>, id: Uuid) -> Result<User> {

        let res = User::get_by_id(&id)?;

        Ok(res)
    }
}