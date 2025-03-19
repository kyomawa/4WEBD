use bson::{DateTime, doc, oid::ObjectId, to_document};
use common::models::AuthRole;
use futures_util::TryStreamExt;
use mongodb::{Collection, Cursor, Database, options::ReturnDocument};
use validator::Validate;

use crate::model::{CreateEventRequest, Event, UpdateEventRequest};

// =============================================================================================================================

const COLLECTION_NAME: &str = "events";

// =============================================================================================================================

pub async fn get_events(db: &Database) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let collection: Collection<Event> = db.collection(COLLECTION_NAME);

    let cursor: Cursor<Event> = collection.find(doc! {}).await?;
    let events: Vec<Event> = cursor.try_collect().await?;

    Ok(events)
}

// =============================================================================================================================

pub async fn get_event_by_id(
    db: &Database,
    id: String,
) -> Result<Event, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Event> = db.collection(COLLECTION_NAME);

    match collection.find_one(doc! { "_id": id }).await? {
        Some(event) => Ok(event),
        None => Err("No event was found with this id".into()),
    }
}

// =============================================================================================================================

pub async fn create_event(
    db: &Database,
    event: CreateEventRequest,
    user_id: String,
) -> Result<Event, Box<dyn std::error::Error>> {
    event.validate()?;

    let creator_id = ObjectId::parse_str(&user_id)?;

    let mut event = Event {
        id: None,
        title: event.title,
        description: event.description,
        capacity: event.capacity,
        remaining_seats: event.remaining_seats,
        creator_id,
        created_at: DateTime::now(),
        date: event.date,
    };

    let collection: Collection<Event> = db.collection(COLLECTION_NAME);

    let res = collection.insert_one(&event).await?;
    event.id = res.inserted_id.as_object_id();

    Ok(event)
}

// =============================================================================================================================

pub async fn update_event_by_id(
    db: &Database,
    event: UpdateEventRequest,
    id: String,
) -> Result<Event, Box<dyn std::error::Error>> {
    event.validate()?;

    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Event> = db.collection(COLLECTION_NAME);
    let update_doc = to_document(&event)?;

    match collection
        .find_one_and_update(doc! { "_id": id}, doc! { "$set": update_doc})
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(event) => Ok(event),
        None => Err("No event was found with this id".into()),
    }
}

// =============================================================================================================================

pub async fn delete_event_by_id(
    db: &Database,
    creator_id: String,
    role: AuthRole,
    id: String,
) -> Result<Event, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let creator_id = ObjectId::parse_str(&creator_id)?;
    let collection: Collection<Event> = db.collection(COLLECTION_NAME);
    let filter = match role {
        AuthRole::Admin => {
            doc! { "_id" : id }
        }
        _ => doc! { "_id": id, "creator_id": creator_id},
    };

    match collection.find_one_and_delete(filter).await? {
        Some(event) => Ok(event),
        None => Err("No event was found with this id or the user who's trying to delete the event is not the creator".into()),
    }
}

// =============================================================================================================================
