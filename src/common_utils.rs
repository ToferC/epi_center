use std::cmp::Ordering;

use async_graphql::Guard;
use async_graphql::*;

#[derive(Eq, PartialEq, Display, EnumString, Copy, Clone, PartialOrd, Ord)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    User,
    Analyst,
    Operator,
    Admin,
}

pub struct RoleGuard {
    pub user_role: UserRole,
}

impl RoleGuard {
    pub fn new(user_role: UserRole) -> Self {
        Self { user_role }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, context: &Context<'_>) -> Result<(), async_graphql::Error> {
        
        if context.data_opt::<UserRole>() == Some(&UserRole::Admin) || context.data_opt::<UserRole>() == Some(&self.user_role) {
            Ok(())
        } else {
            let guard_error = context.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err(format!("Access denied: {} UserRole required", &self.user_role).into())
            }
        }
    }
}

/// Field will be visible to users with UserRole::Admin and
/// UserRole::Analyst
pub fn is_analyst(ctx: &Context<'_>) -> bool {
    if let Some(role) = ctx.data_opt::<UserRole>() {
        let result = match role.cmp(&UserRole::Analyst) {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => true,
        };
        return result
    } else {
        return false
    };
}

/// Field will be visible to users with UserRole::Admin and
/// UserRole::Analyst
pub fn is_operator(ctx: &Context<'_>) -> bool {
    if let Some(role) = ctx.data_opt::<UserRole>() {
        let result = match role.cmp(&UserRole::Operator) {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => true,
        };
        return result
    } else {
        return false
    };
}

/// Field will only be visible to users with UserRole::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<UserRole>() == Some(&UserRole::Admin)
}