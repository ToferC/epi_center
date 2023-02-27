use async_graphql::Guard;
use async_graphql::*;

#[derive(Eq, PartialEq, Display, EnumString, Copy, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    Operator,
    Analyst,
    User,
}

pub struct RoleGuard {
    pub user_role: UserRole,
}

impl RoleGuard {
    pub fn new(user_role: UserRole) -> Self {
        Self { user_role }
    }
}

#[async_trait::async_trait]
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
    ctx.data_opt::<UserRole>() == Some(&UserRole::Admin) ||
    ctx.data_opt::<UserRole>() == Some(&UserRole::Analyst)
}

/// Field will be visible to users with UserRole::Admin and
/// UserRole::Analyst
pub fn is_operator(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<UserRole>() == Some(&UserRole::Admin) ||
    ctx.data_opt::<UserRole>() == Some(&UserRole::Operator)
}

/// Field will only be visible to users with UserRole::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<UserRole>() == Some(&UserRole::Admin)
}