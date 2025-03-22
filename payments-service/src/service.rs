use common::{jwt::external::ExternalClaims, models::AuthRole};
use futures_util::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::{DateTime, doc, oid::ObjectId, to_document},
    options::ReturnDocument,
};
use std::error::Error;
use validator::Validate;

use crate::model::{
    CreatePaymentRequest, Payment, PaymentCurrency, PaymentStatus, UpdatePaymentStatusByIdRequest,
};

// =============================================================================================================================

const COLLECTION_NAME: &str = "payments";

// =============================================================================================================================

pub async fn get_payments(db: &Database) -> Result<Vec<Payment>, Box<dyn Error>> {
    let collection: Collection<Payment> = db.collection(COLLECTION_NAME);
    let cursor = collection.find(doc! {}).await?;
    let payments = cursor.try_collect().await?;

    Ok(payments)
}

// =============================================================================================================================

pub async fn get_payment_by_id(
    db: &Database,
    payment_id: String,
    jwt_payload: ExternalClaims,
) -> Result<Payment, Box<dyn Error>> {
    let payment_id = ObjectId::parse_str(&payment_id)?;

    let collection: Collection<Payment> = db.collection(COLLECTION_NAME);

    let payment = match collection.find_one(doc! {"_id": payment_id}).await? {
        Some(payment) => payment,
        None => return Err("No Payment with this id exist".into()),
    };

    if jwt_payload.role != AuthRole::Admin && jwt_payload.user_id != payment.user_id {
        return Err("You must be an admin or the owner of the payment to access it.".into());
    };

    Ok(payment)
}

// =============================================================================================================================

pub async fn create_payment(
    db: &Database,
    payment_data: CreatePaymentRequest,
) -> Result<Payment, Box<dyn Error>> {
    payment_data.validate()?;

    let mut payment = Payment {
        id: None,
        amount: payment_data.amount,
        created_at: DateTime::now(),
        currency: PaymentCurrency::Eur,
        status: PaymentStatus::Pending,
        event_id: payment_data.event_id,
        user_id: payment_data.user_id,
        ticket_id: payment_data.ticket_id,
    };

    let collection: Collection<Payment> = db.collection(COLLECTION_NAME);

    let res = collection.insert_one(&payment).await?;
    payment.id = res.inserted_id.as_object_id();

    Ok(payment)
}

// =============================================================================================================================

pub async fn update_payment_status_by_id(
    db: &Database,
    payment_id: String,
    payment_status: UpdatePaymentStatusByIdRequest,
) -> Result<Payment, Box<dyn Error>> {
    let payment_id = ObjectId::parse_str(&payment_id)?;
    let collection: Collection<Payment> = db.collection(COLLECTION_NAME);

    let update_doc = to_document(&payment_status)?;

    match collection
        .find_one_and_update(doc! {"_id": payment_id}, doc! { "$set": update_doc})
        .return_document(ReturnDocument::After)
        .await?
    {
        Some(payment) => Ok(payment),
        None => Err("No payment with this id exist.".into()),
    }
}

// =============================================================================================================================

pub async fn delete_payment_by_id(
    db: &Database,
    payment_id: String,
) -> Result<Payment, Box<dyn Error>> {
    let payment_id = ObjectId::parse_str(&payment_id)?;
    let collection: Collection<Payment> = db.collection(COLLECTION_NAME);

    match collection
        .find_one_and_delete(doc! {"_id": payment_id})
        .await?
    {
        Some(payment) => Ok(payment),
        None => Err("No payment with this id exist.".into()),
    }
}

// =============================================================================================================================
