use common::jwt::internal::encode_internal_jwt;
use common::{models::AuthRole, utils::api_response::ApiResponse};
use futures_util::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{Document, doc, to_bson};
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database, bson::DateTime};
use serde_json::json;
use validator::Validate;

use crate::model::{
    CreateTicketRequest, GetEventInternalResponse, Ticket, TicketStatus, UpdateTicketRequest,
};

// =============================================================================================================================

const COLLECTION_NAME: &str = "tickets";

// =============================================================================================================================

pub async fn get_tickets(
    db: &Database,
    user_id: String,
    role: AuthRole,
) -> Result<Vec<Ticket>, Box<dyn std::error::Error>> {
    let user_id: ObjectId = ObjectId::parse_str(&user_id)?;

    let filter: Document = match role {
        AuthRole::Admin | AuthRole::Operator => doc! {},
        _ => doc! { "user_id": user_id },
    };

    let collection: Collection<Ticket> = db.collection(COLLECTION_NAME);
    let cursor = collection.find(filter).await?;

    let tickets = cursor.try_collect().await?;

    Ok(tickets)
}

// =============================================================================================================================

pub async fn get_ticket_by_id(
    db: &Database,
    ticket_id: String,
    role: AuthRole,
    user_id: String,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    let ticket_id = ObjectId::parse_str(&ticket_id)?;
    let user_id = ObjectId::parse_str(&user_id)?;

    let filter: Document = match role {
        AuthRole::Admin | AuthRole::Operator => doc! { "_id": ticket_id },
        _ => doc! { "_id": ticket_id, "user_id" : user_id },
    };

    let collection: Collection<Ticket> = db.collection(COLLECTION_NAME);
    match collection.find_one(filter).await? {
        Some(ticket) => Ok(ticket),
        None => Err("No ticket was found with this id".into()),
    }
}

// =============================================================================================================================

pub async fn create_ticket(
    db: &Database,
    ticket_data: CreateTicketRequest,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    ticket_data.validate()?;

    let res: ApiResponse<GetEventInternalResponse> = reqwest::get(format!(
        "http://events-service:8080/events/{}",
        &ticket_data.event_id
    ))
    .await?
    .json::<ApiResponse<GetEventInternalResponse>>()
    .await?;

    let event: GetEventInternalResponse = if let ApiResponse::Success {
        data: Some(event), ..
    } = res
    {
        event
    } else {
        return Err("No Event found with this id".into());
    };

    if event.remaining_seats < 1 {
        return Err("No more seats are avalaible for this event.".into());
    }

    if event.date < DateTime::now() {
        return Err("This event is not avalaible.".into());
    }

    if ticket_data.seat_number > event.capacity {
        return Err("This seat doesn't exist.".into());
    }

    let mut ticket = Ticket {
        id: None,
        price: event.price,
        seat_number: ticket_data.seat_number,
        status: TicketStatus::Active,
        purchase_date: DateTime::now(),
        event_id: ticket_data.event_id,
        user_id: ticket_data.user_id,
    };

    let collection: Collection<Ticket> = db.collection(COLLECTION_NAME);

    let res = collection.insert_one(&ticket).await?;
    ticket.id = res.inserted_id.as_object_id();

    update_event_remaining_seats_by_id_request(ticket.event_id, -1).await?;

    Ok(ticket)
}

// =============================================================================================================================

pub async fn update_ticket_by_id(
    db: &Database,
    ticket_data: UpdateTicketRequest,
    role: AuthRole,
    ticket_id: String,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    ticket_data.validate()?;
    let ticket_id = ObjectId::parse_str(&ticket_id)?;

    let mut update_fields = doc! {
      "seat_number": ticket_data.seat_number,
    };

    if let AuthRole::Admin = role {
        if let Some(status) = ticket_data.status {
            let status_bson = to_bson(&status)?;
            update_fields.insert("status", status_bson);
        }
    }

    let update_doc = doc! { "$set": update_fields };
    let collection: Collection<Ticket> = db.collection(COLLECTION_NAME);

    match collection
        .find_one_and_update(doc! { "_id": ticket_id }, update_doc)
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(ticket) => Ok(ticket),
        None => Err("No ticket with this id was found.".into()),
    }
}

// =============================================================================================================================

pub async fn cancel_ticket_by_id(
    db: &Database,
    ticket_id: String,
    role: AuthRole,
    user_id: String,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    let ticket_id = ObjectId::parse_str(&ticket_id)?;
    let user_id = ObjectId::parse_str(&user_id)?;

    let filter = match role {
        AuthRole::Admin => doc! { "_id": ticket_id },
        _ => doc! { "_id": ticket_id, "user_id": user_id },
    };

    let update_doc = doc! {
        "$set": {
            "status": to_bson(&TicketStatus::Cancelled)?
        }
    };

    let collection: Collection<Ticket> = db.collection("tickets");

    match collection
        .find_one_and_update(filter, update_doc)
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(ticket) => {
            update_event_remaining_seats_by_id_request(ticket.event_id, 1).await?;
            Ok(ticket)
        }
        None => Err("No ticket found with provided id or unauthorized.".into()),
    }
}

// =============================================================================================================================

pub async fn refund_ticket_by_id(
    db: &Database,
    ticket_id: String,
    role: AuthRole,
    user_id: String,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    let ticket_id = ObjectId::parse_str(&ticket_id)?;
    let user_id = ObjectId::parse_str(&user_id)?;

    let filter = match role {
        AuthRole::Admin => doc! { "_id": ticket_id },
        _ => doc! { "_id": ticket_id, "user_id": user_id },
    };

    let update_doc = doc! {
        "$set": {
            "status": to_bson(&TicketStatus::Refunded)?
        }
    };

    let collection: Collection<Ticket> = db.collection("tickets");

    match collection
        .find_one_and_update(filter, update_doc)
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(ticket) => {
            update_event_remaining_seats_by_id_request(ticket.event_id, 1).await?;
            Ok(ticket)
        }
        None => Err("No ticket found with provided id or unauthorized.".into()),
    }
}

// =============================================================================================================================

pub async fn delete_ticket_by_id(
    db: &Database,
    id: String,
) -> Result<Ticket, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(&id)?;
    let collection: Collection<Ticket> = db.collection(COLLECTION_NAME);

    match collection.find_one_and_delete(doc! { "_id": id}).await? {
        Some(ticket) => {
            update_event_remaining_seats_by_id_request(ticket.event_id, 1).await?;
            Ok(ticket)
        }
        None => Err("No ticket with this id was found".into()),
    }
}

// =============================================================================================================================

async fn update_event_remaining_seats_by_id_request(
    event_id: ObjectId,
    delta: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let internal_token = encode_internal_jwt()?;
    let payload = json!({ "delta": delta });

    client
        .patch(format!(
            "http://events-service:8080/events/{}/update-seats",
            event_id.to_hex()
        ))
        .header("Authorization", format!("Bearer {}", internal_token))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

// =============================================================================================================================
