use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub async fn init_db() -> mongodb::error::Result<Database> {
    let uri = std::env::var("MONGODB_URL").expect("Missing MONGODB_URL environment variable");
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("movie_api"))
}