use common::{jwt::internal::encode_internal_jwt, utils::api_response::ApiResponse};
use futures_util::TryStreamExt;
use mongodb::{
    Collection, Cursor, Database,
    bson::{DateTime, doc, oid::ObjectId, to_bson},
};

use crate::model::{Backup, BackupService, BackupStatus, CreateBackup, GetLastBackupByServiceName};
use std::error::Error;

// =============================================================================================================================

const COLLECTION_NAME: &str = "backups";

// =============================================================================================================================

pub async fn get_last_backup_by_service_name(
    db: &Database,
    backup: GetLastBackupByServiceName,
) -> Result<Backup, Box<dyn Error>> {
    let service_name = to_bson(&backup.service_name)?;
    let collection: Collection<Backup> = db.collection(COLLECTION_NAME);

    match collection
        .find_one(doc! {"service_name": service_name})
        .sort(doc! { "date": -1 })
        .await?
    {
        Some(backup) => Ok(backup),
        None => Err("No backup has been achieved for this service".into()),
    }
}

// =============================================================================================================================

pub async fn get_backup_by_id(db: &Database, backup_id: String) -> Result<Backup, Box<dyn Error>> {
    let backup_id = ObjectId::parse_str(backup_id)?;
    let collection: Collection<Backup> = db.collection(COLLECTION_NAME);

    match collection.find_one(doc! { "_id": backup_id }).await? {
        Some(backup) => Ok(backup),
        None => Err("No backup with this id was found.".into()),
    }
}

// =============================================================================================================================

pub async fn create_backup(db: &Database, backup: CreateBackup) -> Result<Backup, Box<dyn Error>> {
    let mut backup = Backup {
        id: None,
        created_at: DateTime::now(),
        service_name: backup.service_name,
        status: BackupStatus::Pending,
        data: None,
        finished_at: None,
    };

    let collection: Collection<Backup> = db.collection(COLLECTION_NAME);

    let res = collection.insert_one(&backup).await?;
    backup.id = res.inserted_id.as_object_id();

    Ok(backup)
}

// =============================================================================================================================

pub async fn delete_backup_by_id(
    db: &Database,
    backup_id: String,
) -> Result<Backup, Box<dyn Error>> {
    let backup_id = ObjectId::parse_str(backup_id)?;
    let collection: Collection<Backup> = db.collection(COLLECTION_NAME);

    match collection
        .find_one_and_delete(doc! {"_id": backup_id})
        .await?
    {
        Some(backup) => Ok(backup),
        None => Err("No backup with this id was found.".into()),
    }
}

// =============================================================================================================================

pub async fn process_pending_backups(db: &Database) -> Result<(), Box<dyn Error + Send + Sync>> {
    let collection: Collection<Backup> = db.collection("backups");
    let filter = doc! { "status": "Pending" };

    let mut cursor: Cursor<Backup> = collection.find(filter).await?;

    while let Some(backup) = cursor.try_next().await? {
        let update_in_progress = doc! { "$set": { "status": "InProgress" } };
        collection
            .update_one(doc! { "_id": &backup.id }, update_in_progress)
            .await?;

        let datas_to_save = match get_specific_service_datas(backup.service_name).await {
            Ok(data) => data,
            Err(e) => {
                let update_failed = doc! { "$set": { "status": "Failed" } };
                collection
                    .update_one(doc! { "_id": &backup.id }, update_failed)
                    .await?;
                eprintln!(
                    "Error retrieving data for backup {:?}: {}",
                    backup.id.as_ref().map(|oid| oid.to_hex()),
                    e
                );
                continue;
            }
        };

        let datas_to_save = to_bson(&datas_to_save)?;

        let update_data = doc! { "$set": { "data": datas_to_save } };
        match collection
            .update_one(doc! { "_id": &backup.id }, update_data)
            .await
        {
            Ok(_) => {
                let update_completed = doc! { "$set": { "status": "Completed", "finished_at": DateTime::now() } };
                collection
                    .update_one(doc! { "_id": &backup.id }, update_completed)
                    .await?;
            }
            Err(e) => {
                let update_failed = doc! { "$set": { "status": "Failed", "finished_at": DateTime::now() } };
                collection
                    .update_one(doc! { "_id": &backup.id }, update_failed)
                    .await?;
                eprintln!(
                    "Error updating backup data for backup {:?}: {}",
                    backup.id.as_ref().map(|oid| oid.to_hex()),
                    e
                );
            }
        }
    }

    Ok(())
}

// =============================================================================================================================

async fn get_specific_service_datas(
    service_name: BackupService,
) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let internal_token = encode_internal_jwt()?;

    let url = match service_name {
        BackupService::Auth => "http://auth-service:8080/api/auth",
        BackupService::Events => "http://events-service:8080/api/events",
        BackupService::Notifications => "http://notifications-service:8080/api/notifications",
        BackupService::Payments => "http://payments-service:8080/api/payments",
        BackupService::Tickets => "http://tickets-service:8080/api/tickets",
        BackupService::Users => "http://users-service:8080/api/users",
    };

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", internal_token))
        .send()
        .await?
        .json::<ApiResponse<serde_json::Value>>()
        .await?;

    match res {
        ApiResponse::Success { data, .. } => Ok(data.unwrap()),
        ApiResponse::Error { error, .. } => Err(error.into()),
    }
}

// =============================================================================================================================

pub async fn trigger_backups_for_all_services(
    db: &Database,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let collection: Collection<Backup> = db.collection(COLLECTION_NAME);

    let services: Vec<BackupService> = vec![
        BackupService::Auth,
        BackupService::Events,
        BackupService::Notifications,
        BackupService::Payments,
        BackupService::Tickets,
        BackupService::Users,
    ];

    let now = DateTime::now();

    for service in services {
        let backup = Backup {
            id: None,
            created_at: now,
            finished_at: None,
            service_name: service,
            status: BackupStatus::Pending,
            data: None,
        };

        collection.insert_one(backup).await?;
    }

    Ok(())
}

// =============================================================================================================================
