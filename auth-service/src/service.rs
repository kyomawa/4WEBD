use bcrypt::{DEFAULT_COST, hash, verify};
use common::{
    jwt::{external::encode_external_jwt, internal::encode_internal_jwt},
    models::{AuthRole, TriggerNotificationRequest},
    utils::{
        api_response::{ApiResponse, ObjectIdWrapper},
        utils::trigger_notification,
    },
};
use futures::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;
use std::{thread, time::Duration};
use validator::Validate;

use crate::model::{
    Auth, CreateAuthRequest, CreateUserInternalResponse, LoginRequest, LoginResponse,
};

// =============================================================================================================================

const COLLECTION_NAME: &str = "auth";

// =============================================================================================================================

pub async fn get_auths(db: &Database) -> Result<Vec<Auth>, Box<dyn std::error::Error>> {
    let collection: Collection<Auth> = db.collection(COLLECTION_NAME);
    let cursor = collection.find(doc! {}).await?;
    let auths = cursor.try_collect().await?;

    Ok(auths)
}

// =============================================================================================================================

pub async fn delete_auth_by_user_id(
    db: &Database,
    user_id: String,
) -> Result<Auth, Box<dyn std::error::Error>> {
    let user_id = ObjectId::parse_str(&user_id)?;
    let collection: Collection<Auth> = db.collection(COLLECTION_NAME);

    match collection
        .find_one_and_delete(doc! {"user_id": user_id})
        .await?
    {
        Some(auth) => Ok(auth),
        None => Err("No auth found with the given user_id".into()),
    }
}

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
    let res: ApiResponse<CreateUserInternalResponse> = client
        .post("http://users-service:8080/api/users")
        .header("Authorization", format!("Bearer {}", internal_token))
        .json(&user)
        .send()
        .await?
        .json::<ApiResponse<CreateUserInternalResponse>>()
        .await?;

    let user = match res {
        ApiResponse::Success {
            data: Some(user), ..
        } => user,
        ApiResponse::Error { error, .. } => return Err(error.into()),
        other => return Err(format!("Unexpected response from User Service: {:?}", other).into()),
    };

    let id = ObjectId::parse_str(&user.id)?;

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

    let notification_data = TriggerNotificationRequest {
        message: String::from("Welcome to 4WEBD."),
        user_id: credential.user_id.clone(),
    };

    if let Err(e) = trigger_notification(notification_data).await {
        return Err(e);
    }

    Ok(credential)
}

// =============================================================================================================================

pub async fn login(
    db: &Database,
    payload: LoginRequest,
) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let email = json!({ "email": &payload.email });
    let client = reqwest::Client::new();

    let internal_token = encode_internal_jwt()?;
    let res: ApiResponse<ObjectIdWrapper> = client
        .get("http://users-service:8080/api/users/id-by-email")
        .header("Authorization", format!("Bearer {}", internal_token))
        .json(&email)
        .send()
        .await?
        .json::<ApiResponse<ObjectIdWrapper>>()
        .await?;

    let user_id: ObjectId = match res {
        ApiResponse::Success {
            data: Some(wrapper),
            ..
        } => wrapper.id,
        ApiResponse::Error { error, .. } => return Err(error.into()),
        other => return Err(format!("Unexpected response from User Service: {:?}", other).into()),
    };

    let collection: Collection<Auth> = db.collection(COLLECTION_NAME);

    let credentials = match collection.find_one(doc! { "user_id": user_id  }).await? {
        Some(credentials) => credentials,
        None => return Err("No user with this id exist".into()),
    };

    if let Err(_) | Ok(false) = verify(&payload.password, &credentials.password) {
        thread::sleep(Duration::from_millis(300));
        return Err("Invalid email or password".into());
    }

    let token = encode_external_jwt(credentials.user_id.to_hex(), credentials.role)?;

    Ok(LoginResponse { token })
}

// =============================================================================================================================
