use super::*;

use async_graphql::{Context, Object};

use cfg_if::cfg_if;

#[derive(Default)]
pub struct BankAccountGraphQlQuery {}

#[derive(Default)]
pub struct BankAccountGraphQlMutation {}

cfg_if! {
    if #[cfg(feature = "postgres")] {
        #[Object]
        impl BankAccountGraphQlQuery {
            #[instrument(skip(self, ctx))]
            /// Get a bank account by its ID
            async fn bank_account_query<'ctx>(
                &self,
                ctx: &Context<'ctx>,
                id: String,
            ) -> async_graphql::Result<BankAccountView> {
                let view_repo = ctx.data::<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>()?;
                let view = match view_repo.load(&id).await? {
                    Some(view) => view,
                    None => {
                        return Err(async_graphql::Error::new("Bank account not found"));
                    }
                };
                tracing::debug!("Loaded view in GraphQL response: {:?}", view);
                Ok(view)
            }
        }

        #[Object]
impl BankAccountGraphQlMutation {
    #[instrument(skip(self, ctx))]
    /// Issue a command on the bank account aggregate
    async fn bank_account_mutation<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        command: BankAccountCommand,
    ) -> async_graphql::Result<BankAccountView> {
        let cqrs = ctx.data::<Arc<PostgresCqrs<BankAccount>>>()?;
        let view_repo = ctx.data::<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>()?;

        match cqrs.execute(&id, command).await {
            Ok(_) => {}
            Err(err) => {
                return Err(async_graphql::Error::new(err.to_string()));
            }
        }
        let view = match view_repo.load(&id).await {
            Ok(view) => match view {
                Some(view) => view,
                None => {
                    return Err(async_graphql::Error::new("Bank account not found"));
                }
            },
            Err(err) => {
                return Err(async_graphql::Error::new(err.to_string()));
            }
        };
        Ok(view)
    }
}

    } else if #[cfg(feature = "mysql")] {
#[Object]
impl BankAccountGraphQlQuery {
    #[instrument(skip(self, ctx))]
    /// Get a bank account by its ID
    async fn bank_account_query<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
    ) -> async_graphql::Result<BankAccountView> {
        let view_repo = ctx.data::<Arc<MysqlViewRepository<BankAccountView, BankAccount>>>()?;
        let view = match view_repo.load(&id).await? {
            Some(view) => view,
            None => {
                return Err(async_graphql::Error::new("Bank account not found"));
            }
        };
        tracing::debug!("Loaded view in GraphQL response: {:?}", view);
        Ok(view)
    }
}

#[Object]
impl BankAccountGraphQlMutation {
    #[instrument(skip(self, ctx))]
    /// Issue a command on the bank account aggregate
    async fn bank_account_mutation<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        command: BankAccountCommand,
    ) -> async_graphql::Result<BankAccountView> {
        let cqrs = ctx.data::<Arc<MysqlCqrs<BankAccount>>>()?;
        let view_repo = ctx.data::<Arc<MysqlViewRepository<BankAccountView, BankAccount>>>()?;

        match cqrs.execute(&id, command).await {
            Ok(_) => {}
            Err(err) => {
                return Err(async_graphql::Error::new(err.to_string()));
            }
        }
        let view = match view_repo.load(&id).await {
            Ok(view) => match view {
                Some(view) => view,
                None => {
                    return Err(async_graphql::Error::new("Bank account not found"));
                }
            },
            Err(err) => {
                return Err(async_graphql::Error::new(err.to_string()));
            }
        };
        Ok(view)
    }
}
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}
