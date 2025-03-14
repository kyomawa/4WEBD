use mongodb::bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "_id")]
    id: ObjectId,
    password: String,
    role: AuthRole,
    user_id: ObjectId,
}

#[derive(Serialize, Deserialize)]
enum AuthRole {
    User,
    EventCreator,
    Operator,
    Admin,
}
