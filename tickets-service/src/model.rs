use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub enum TicketStatus {
    Active,
    Refunded,
    Cancelled,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Option<ObjectId>,
    pub purchase_date: DateTime,
    pub seat_number: u16,
    pub status: TicketStatus,
    pub price: u16,
    pub event_id: ObjectId,
    pub user_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTicketRequest {
    pub purchase_date: DateTime,
    pub seat_number: u16,
    pub status: TicketStatus,
    pub price: u16,
    pub event_id: ObjectId,
}

// =============================================================================================================================
