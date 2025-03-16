use bcrypt::{DEFAULT_COST, hash, verify};
use common::{
    jwt::{external::encode_external_jwt, internal::encode_internal_jwt},
    models::{AuthRole, User},
    utils::api_response::ApiResponse,
};
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;
use validator::Validate;

use crate::model::{Auth, CreateAuthRequest, LoginRequest};

// =============================================================================================================================

const COLLECTION_NAME: &str = "auth";

// =============================================================================================================================

pub async fn register(
    db: &Database,
    payload: CreateAuthRequest,
) -> Result<Auth, Box<dyn std::error::Error>> {
    payload.validate()?;

    let client = reqwest::Client::new();
    let user = json!({
        "email": &payload.email,
        "first_name": &payload.first_name,
        "last_name": &payload.last_name,
    });

    let internal_token = encode_internal_jwt()?;
    let res: ApiResponse<User> = client
        .post("http://users-service:8080/users")
        .header("Authorization", format!("Bearer {}", internal_token))
        .json(&user)
        .send()
        .await?
        .json::<ApiResponse<User>>()
        .await?;

    let user = match res {
        ApiResponse::Success {
            data: Some(user), ..
        } => user,
        ApiResponse::Error { error, .. } => return Err(error.into()),
        other => return Err(format!("Unexpected response from User Service: {:?}", other).into()),
    };

    let id = match user.id {
        Some(id) => id,
        None => return Err("Missing user id".into()),
    };

    let collection: Collection<Auth> = db.collection(COLLECTION_NAME);
    let hashed_password = hash(&payload.password, DEFAULT_COST)?;

    let mut credential = Auth {
        id: None,
        password: hashed_password,
        role: AuthRole::User,
        user_id: id,
    };

    let result = collection.insert_one(&credential).await?;

    credential.id = result.inserted_id.as_object_id();

    Ok(credential)
}

// =============================================================================================================================

pub async fn login(
    db: &Database,
    user_id: ObjectId,
    payload: LoginRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let collection: Collection<Auth> = db.collection(COLLECTION_NAME);

    let credentials = match collection.find_one(doc! { "user_id": user_id  }).await? {
        Some(credentials) => credentials,
        None => return Err("No user with this id exist".into()),
    };

    match verify(&payload.password, &credentials.password) {
        Ok(true) => (),
        Ok(false) => return Err("Invalid password".into()),
        Err(e) => return Err(e.into()),
    };

    let token = encode_external_jwt(credentials.user_id.to_hex(), credentials.role)?;

    Ok(token)
}

// =============================================================================================================================
