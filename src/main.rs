mod db;

use dotenvy::dotenv;
use db::init_db;

mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = init_db()
        .await
        .expect("Failed to initialize database");
}
