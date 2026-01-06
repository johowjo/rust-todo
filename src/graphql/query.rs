use crate::Db;
use crate::entities::todo::Model as Todo;
use async_graphql::{Context, Object, Result};
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "hello!".to_string()
    }

    async fn get_todos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Todo>> {
        let db = ctx.data::<Db>()?;
        Ok(sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(&db.mysql_pool)
            .await?)
    }
}
