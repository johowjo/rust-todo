mod db;
mod entities;
mod graphql;
mod routes;
use async_graphql::{EmptySubscription, Schema};
use db::Db;
use dotenv::dotenv;
use graphql::{mutation::Mutation, query::Query};
use routes::build_routes;
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port: String = var("PORT").expect("PORT is not set");
    let db = Db::init().await;
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();

    let app = build_routes(schema).await;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("GraphQL available at http://localhost:{port}/graphql");

    axum::serve(listener, app).await.unwrap();
}
