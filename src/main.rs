mod db;
mod graphql;
mod routes;
use db::Db;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database = Db::init().await;
}
