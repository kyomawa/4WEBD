use mongodb::{ Client, options::ClientOptions, Database };
use std::error::Error;

// =============================================================================================================================

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    let client_options = ClientOptions::parse(
        std::env
            ::var("DATABASE_URL_USERS_SERVICE")
            .expect(
                "❌ No env variable called DATABASE_URL_USERS_SERVICE was found in the .env file"
            )
    ).await?;

    let client = Client::with_options(client_options)?;

    Ok(client.database("users-service"))
}

// =============================================================================================================================
