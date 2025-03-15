use futures_util::TryStreamExt;
use mongodb::{ bson::doc, Collection, Cursor, Database };

use crate::model::User;

// =============================================================================================================================

const COLLECTION_NAME: &str = "users";

// =============================================================================================================================

pub async fn get_users(db: &Database) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let collection: Collection<User> = db.collection(COLLECTION_NAME);
    let cursor: Cursor<User> = collection.find(doc! {}).await?;
    let users: Vec<User> = cursor.try_collect().await?;

    Ok(users)
}

// =============================================================================================================================

pub async fn create_user(db: &Database, payload: User) -> Result<User, Box<dyn std::error::Error>> {
    let collection: Collection<User> = db.collection(COLLECTION_NAME);
    let mut user = User {
        id: None,
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
    };

    let res = collection.insert_one(&user).await?;

    user.id = res.inserted_id.as_object_id();

    Ok(user)
}

// =============================================================================================================================
