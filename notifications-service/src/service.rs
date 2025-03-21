use common::{jwt::internal::encode_internal_jwt, utils::api_response::ApiResponse};
use futures_util::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::{DateTime, doc, oid::ObjectId, to_bson},
    options::ReturnDocument,
};
use validator::Validate;

use crate::{
    email::{SendMail, send_mail},
    model::{
        CreateNotification, GetUserInternalResponse, Notification, NotificationStatus,
        NotificationType, UpdateNotificationStatus,
    },
};

// =============================================================================================================================

const COLLECTION_NAME: &str = "notifications";

// =============================================================================================================================

pub async fn get_notifications(
    db: &Database,
) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);
    let cursor = collection.find(doc! {}).await?;

    let notifications = cursor.try_collect().await?;

    Ok(notifications)
}

// =============================================================================================================================

pub async fn get_notification_by_id(
    db: &Database,
    id: String,
) -> Result<Notification, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;

    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);
    match collection.find_one(doc! { "_id": id}).await? {
        Some(notification) => Ok(notification),
        None => Err("No notification was found with this id".into()),
    }
}

// =============================================================================================================================

pub async fn create_notification(
    db: &Database,
    notification: CreateNotification,
) -> Result<Notification, Box<dyn std::error::Error>> {
    notification.validate()?;

    let mut notification = Notification {
        id: None,
        message: notification.message,
        notif_type: NotificationType::Email,
        status: NotificationStatus::Pending,
        created_at: DateTime::now(),
        user_id: notification.user_id,
    };

    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);

    let res = collection.insert_one(&notification).await?;
    notification.id = res.inserted_id.as_object_id();

    Ok(notification)
}

// =============================================================================================================================

pub async fn update_notification_status_by_id(
    db: &Database,
    id: String,
    notification: UpdateNotificationStatus,
) -> Result<Notification, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);
    let update_doc = doc! {
      "$set": {
        "status": to_bson(&notification.status)?
      }
    };

    match collection
        .find_one_and_update(doc! { "_id": id}, update_doc)
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(notification) => Ok(notification),
        None => Err("No notification with this id was found".into()),
    }
}

// =============================================================================================================================

pub async fn delete_notification_by_id(
    db: &Database,
    id: String,
) -> Result<Notification, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);

    match collection.find_one_and_delete(doc! { "_id": id}).await? {
        Some(notification) => Ok(notification),
        None => Err("No notification with this id was found".into()),
    }
}

// =============================================================================================================================

pub async fn check_notifications_and_try_to_send_mail(
    db: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Notification> = db.collection(COLLECTION_NAME);
    let status_bson = to_bson(&NotificationStatus::Pending).unwrap();
    let cursor = collection.find(doc! { "status": status_bson }).await?;
    let pending_notifications: Vec<Notification> = cursor.try_collect().await?;

    let client = reqwest::Client::new();
    let internal_token = encode_internal_jwt()?;

    for notification in pending_notifications {
        let api_response = client
            .get(format!(
                "http://users-service:8080/users/{}",
                notification.user_id
            ))
            .header("Authorization", format!("Bearer {}", internal_token))
            .send()
            .await?
            .json::<ApiResponse<GetUserInternalResponse>>()
            .await?;

        match api_response {
            ApiResponse::Success {
                data: Some(user), ..
            } => {
                let mail_to_send = SendMail {
                    to: user.email.clone(),
                    subject: "The 4WEBD Team".to_string(),
                    body: notification.message.clone(),
                };

                match send_mail(mail_to_send).await {
                    Ok(_) => {
                        let update_doc = doc! {
                            "$set": { "status": to_bson(&NotificationStatus::Sent).unwrap() }
                        };
                        collection
                            .update_one(doc! { "_id": notification.id.clone() }, update_doc)
                            .await?;
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de l'envoi à {}: {:?}", user.email, e);
                        let update_doc = doc! {
                            "$set": { "status": to_bson(&NotificationStatus::Failed).unwrap() }
                        };
                        collection
                            .update_one(doc! { "_id": notification.id.clone() }, update_doc)
                            .await?;
                    }
                }
            }
            ApiResponse::Success {
                data: None,
                message,
                ..
            } => {
                eprintln!(
                    "Réponse réussie mais aucune donnée utilisateur pour {}: {}",
                    notification.user_id, message
                );
                let update_doc = doc! {
                    "$set": { "status": to_bson(&NotificationStatus::Failed).unwrap() }
                };
                collection
                    .update_one(doc! { "_id": notification.id.clone() }, update_doc)
                    .await?;
            }
            ApiResponse::Error { message, error, .. } => {
                eprintln!(
                    "Erreur lors de la récupération de l'utilisateur {}: {} - {}",
                    notification.user_id, message, error
                );
                let update_doc = doc! {
                    "$set": { "status": to_bson(&NotificationStatus::Failed).unwrap() }
                };
                collection
                    .update_one(doc! { "_id": notification.id.clone() }, update_doc)
                    .await?;
            }
        }
    }

    Ok(())
}

// =============================================================================================================================
