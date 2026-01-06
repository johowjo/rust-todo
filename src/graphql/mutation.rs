use crate::Db;
use crate::entities::todo::Model as Todo;
use async_graphql::{Context, Object, Result};
pub struct Mutation;

#[Object]
impl Mutation {
    async fn new_todo<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        title: String,
        content: String,
    ) -> Result<Todo> {
        let db = ctx.data::<Db>()?;
        let result = sqlx::query!(
            "INSERT INTO todo(title, content, done) VALUES (?, ?, ?)",
            title,
            content,
            0
        )
        .execute(&db.mysql_pool)
        .await?;

        Ok(Todo {
            title,
            content,
            id: result.last_insert_id() as i32,
            done: 0,
        })
    }
}
