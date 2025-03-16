use std::str::FromStr;

use futures_util::TryStreamExt;
use mongodb::{
    Collection, Cursor, Database,
    bson::{doc, oid::ObjectId, to_document},
};
use validator::Validate;

use crate::model::{CreateUserRequest, UpdateUserRequest, User};

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

pub async fn get_user_by_id(db: &Database, id: String) -> Result<User, Box<dyn std::error::Error>> {
    let id = ObjectId::from_str(&id).expect("The id is not a valid mongodb id");
    let collection: Collection<User> = db.collection(COLLECTION_NAME);

    match collection.find_one(doc! { "_id": id }).await? {
        Some(user) => Ok(user),
        None => Err("No user found with the given id".into()),
    }
}

// =============================================================================================================================

pub async fn create_user(
    db: &Database,
    payload: CreateUserRequest,
) -> Result<User, Box<dyn std::error::Error>> {
    payload.validate()?;
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

pub async fn update_user_by_id(
    db: &Database,
    id: String,
    user: UpdateUserRequest,
) -> Result<User, Box<dyn std::error::Error>> {
    let id = ObjectId::from_str(&id).expect("The id is not a valid mongodb id");
    let collection: Collection<User> = db.collection(COLLECTION_NAME);
    let update_doc = to_document(&user)?;

    match collection
        .find_one_and_update(doc! { "_id": id }, doc! { "$set": update_doc })
        .await?
    {
        Some(user) => Ok(user),
        None => Err("Failed to update the current user.".into()),
    }
}

// =============================================================================================================================

pub async fn delete_user(db: &Database, id: String) -> Result<User, Box<dyn std::error::Error>> {
    let id = ObjectId::from_str(&id).expect("The id is not a valid mongodb id");
    let collection: Collection<User> = db.collection(COLLECTION_NAME);
    match collection.find_one_and_delete(doc! { "_id": id }).await? {
        Some(user) => Ok(user),
        None => Err("No user found with the given id".into()),
    }
}

// =============================================================================================================================
