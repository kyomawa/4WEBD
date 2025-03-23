use mongodb::{
    Collection, Database,
    bson::{DateTime, doc, oid::ObjectId, to_bson},
};

use crate::model::{Backup, BackupStatus, CreateBackup, GetLastBackupByServiceName};
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
