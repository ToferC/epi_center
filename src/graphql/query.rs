use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, BoolExpressionMethods};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership, OrgOwnership,
    Team, Organization, Role, OrgTier};
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
    /// family name.
    pub async fn all_people(
        &self, 
        context: &Context<'_>,
        count: i64,
    ) -> Result<Vec<Person>> {

        let mut conn = get_connection_from_context(context);

        let res = persons::table
            .order(persons::family_name)
            .limit(count)
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "personById")]
    pub async fn person_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> Result<Person> {

        let mut conn = get_connection_from_context(context);

        let res = persons::table.filter(persons::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "personByName")]
    pub async fn person_by_name(
        &self, 
        context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Person>> {

        let mut conn = get_connection_from_context(context);

        let res = persons::table
            .filter(persons::family_name.like(format!("%{}%", name)).or(persons::given_name.like(format!("%{}%", name))))
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    // Teams
    #[graphql(name = "allTeams")]
    /// Returns a vector of all travel groups
    pub async fn all_teams(
        &self, 
        context: &Context<'_>,
    ) -> Result<Vec<Team>> {
        let mut conn = get_connection_from_context(context);

        let res = teams::table.load::<Team>(&mut conn)?;

        Ok(res)
    }

    
    #[graphql(name = "teamByID")]
    /// Returns a specific travel group by its UUID
    pub async fn team_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> Result<Team> {
        let mut conn = get_connection_from_context(context);

        let res = teams::table
        .filter(teams::id.eq(&id))
        .first(&mut conn)?;
        
        Ok(res)
    }

    #[graphql(name = "teamByEnglishName")]
    pub async fn team_by_english_name(
        &self, 
        context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Team>> {

        let mut conn = get_connection_from_context(context);

        let res = teams::table
            .filter(teams::name_en.like(format!("%{}%", name)))
            .load::<Team>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "teamByFrenchName")]
    pub async fn team_by_french_name(
        &self, 
        context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Team>> {

        let mut conn = get_connection_from_context(context);

        let res = teams::table
            .filter(teams::name_fr.like(format!("%{}%", name)))
            .load::<Team>(&mut conn)?;

        Ok(res)
    }

    // Roles

    #[graphql(name = "getRole")]
    /// Accepts an argument of "count" and returns a vector of {count} role
    pub async fn get_role(&self, context: &Context<'_>, count: i64) -> Result<Vec<Role>> {
        let mut conn = get_connection_from_context(context);

        let res = roles::table
            .limit(count)
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "allRoles")]
    /// Returns a vector of all persons ordered by family name
    pub async fn all_roles(
        &self, 
        context: &Context<'_>,) -> Result<Vec<Role>> {

        let mut conn = get_connection_from_context(context);

        let res = roles::table
            .order(roles::start_datestamp)
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    // Organizations

    #[graphql(name = "allOrganizations")]
    /// Returns a vector of all organization histories
    pub async fn all_organizations(&self, context: &Context<'_>) -> Result<Vec<Organization>> {
        let mut conn = get_connection_from_context(context);

        let res = organizations::table
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "getOrganizations")]
    /// Accepts argument "count" and returns a vector of {count} organization histories
    pub async fn get_organizations(&self, context: &Context<'_>, count: i64) -> Result<Vec<Organization>> {
        let mut conn = get_connection_from_context(context);

        let res = organizations::table
            .limit(count)
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "organizationByName")]
    pub async fn organization_by_name(
        &self, 
        context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Organization>> {

        let mut conn = get_connection_from_context(context);

        let res = organizations::table
            .filter(organizations::name_en.like(format!("%{}%", name)).or(organizations::name_fr.like(format!("%{}%", name))))
            .load::<Organization>(&mut conn)?;

        Ok(res)
    }

    // OrgTiers

    #[graphql(name = "allOrgTiers")]
    /// Returns a vector of all quarantine plans
    pub async fn all_org_tiers(&self, context: &Context<'_>) -> Result<Vec<OrgTier>> {
        let mut conn = get_connection_from_context(context);

        let res = org_tiers::table
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "getOrgTiers")]
    /// Accepts argument "count" and returns a vector of {count} quarantine plans
    pub async fn get_org_tiers(&self, context: &Context<'_>, count: i64) -> Result<Vec<OrgTier>> {
        let mut conn = get_connection_from_context(context);

        let res = org_tiers::table
            .limit(count)
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "orgTierByName")]
    pub async fn org_tier_by_name(
        &self, 
        context: &Context<'_>,
        name: String,
    ) -> Result<Vec<OrgTier>> {

        let mut conn = get_connection_from_context(context);

        let res = org_tiers::table
            .filter(org_tiers::name_en.like(&name).or(org_tiers::name_fr.like(format!("%{}%", name))))
            .load::<OrgTier>(&mut conn)?;

        Ok(res)
    }

    // TeamOwnerships
    #[graphql(name = "allTeamOwnership")]
    /// Returns a vector of all covid test results
    pub async fn all_team_ownership_results(&self, context: &Context<'_>) -> Result<Vec<TeamOwnership>> {
        let mut conn = get_connection_from_context(context);

        let res = team_ownerships::table.load::<TeamOwnership>(&mut conn)?;

        Ok(res)
    }

    #[graphql(name = "getTeamOwnership")]
    /// Accepts argument "count" and returns a vector of {count} covid test results
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