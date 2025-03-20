use mongodb::{Client, Database, options::ClientOptions};

// =============================================================================================================================

pub async fn init_db() -> Result<Database, Box<dyn std::error::Error>> {
    let client_options =
        ClientOptions::parse(std::env::var("DATABASE_URL_NOTIFICATIONS_SERVICE").expect(
            "❌ No env variable called DATABASE_URL_NOTIFICATIONS_SERVICE was found in the .env file",
        ))
        .await?;

    let client = Client::with_options(client_options)?;

    Ok(client.database("notifications-service"))
}

// =============================================================================================================================
