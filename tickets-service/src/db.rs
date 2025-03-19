use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;

// =============================================================================================================================

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    let client_optins = ClientOptions::parse(std::env::var("DATABASE_URL_TICKETS_SERVICE").expect(
        "❌ No env variable called DATABASE_URL_TICKETS_SERVICE was found in the .env file",
    ))
    .await?;

    let client = Client::with_options(client_optins)?;

    Ok(client.database("tickets-service"))
}

// =============================================================================================================================
