use bson::{DateTime, doc, oid::ObjectId, to_document};
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{Collection, Cursor, Database};
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
        .await?
    {
        Some(event) => Ok(event),
        None => Err("No event was found with this id".into()),
    }
}

// =============================================================================================================================

pub async fn delete_event_by_id(
    db: &Database,
    id: String,
) -> Result<Event, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Event> = db.collection(COLLECTION_NAME);

    match collection.find_one_and_delete(doc! { "_id": id}).await? {
        Some(event) => Ok(event),
        None => Err("No event was found with this id".into()),
    }
}

// =============================================================================================================================
