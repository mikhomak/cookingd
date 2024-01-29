use async_graphql::{async_trait, Context, Guard, Error};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    User,
    Anon,
}

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
   pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}