use mongodb::{ bson::oid::ObjectId, Collection, Database };

use crate::model::{ CreateUserRequest, User };

// =============================================================================================================================

pub async fn create_user(
    db: &Database,
    payload: CreateUserRequest
) -> Result<User, Box<dyn std::error::Error>> {
    let collection: Collection<User> = db.collection("users");
    let user = User {
        id: ObjectId::new(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
    };

    collection.insert_one(&user).await?;

    Ok(user)
}

// =============================================================================================================================
