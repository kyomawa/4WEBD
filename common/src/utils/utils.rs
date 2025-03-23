use chrono::Utc;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{self, Deserializer, Visitor};
use serde::{self, Deserialize, Serializer};
use serde_json::json;
use std::fmt;
use std::time::{Duration, UNIX_EPOCH};
use validator::ValidationError;

use crate::jwt::internal::encode_internal_jwt;
use crate::models::{TriggerNotificationRequest, TriggerNotificationResponse};

use super::api_response::ApiResponse::{self, Error, Success};

// =============================================================================================================================

pub static LETTERS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s'-]+$").unwrap());

// =============================================================================================================================

pub fn serialize_option_object_id_as_hex_string<S>(
    id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serialize_object_id_as_hex_string(oid, serializer),
        None => serializer.serialize_none(),
    }
}

// =============================================================================================================================

pub fn trim_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_lowercase())
}

// =============================================================================================================================

pub fn trim<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

// =============================================================================================================================

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    ObjectId::parse_str(&s).map_err(de::Error::custom)
}

// =============================================================================================================================

pub fn deserialize_datetime_from_any<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = DateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a BSON DateTime or an ISO8601 string")
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime, E>
        where
            E: de::Error,
        {
            let chrono_dt = chrono::DateTime::parse_from_rfc3339(value).map_err(E::custom)?;
            let ts = chrono_dt.with_timezone(&Utc).timestamp_millis();
            let system_time = UNIX_EPOCH + Duration::from_millis(ts as u64);
            Ok(DateTime::from_system_time(system_time))
        }

        fn visit_i64<E>(self, value: i64) -> Result<DateTime, E>
        where
            E: de::Error,
        {
            Ok(DateTime::from_millis(value))
        }
    }

    deserializer.deserialize_any(DateTimeVisitor)
}

// =============================================================================================================================

pub fn deserialize_option_datetime_from_any<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionDateTimeVisitor;

    impl<'de> de::Visitor<'de> for OptionDateTimeVisitor {
        type Value = Option<DateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional BSON DateTime, an ISO8601 string, or null")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_datetime_from_any(deserializer).map(Some)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let chrono_dt = chrono::DateTime::parse_from_rfc3339(value).map_err(E::custom)?;
            let ts = chrono_dt.with_timezone(&Utc).timestamp_millis();
            let system_time = UNIX_EPOCH + Duration::from_millis(ts as u64);
            Ok(Some(DateTime::from_system_time(system_time)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(DateTime::from_millis(value)))
        }
    }

    deserializer.deserialize_any(OptionDateTimeVisitor)
}

// =============================================================================================================================

pub fn serialize_option_datetime_as_rfc3339_string<S>(
    date: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => {
            let s = dt.try_to_rfc3339_string().unwrap();
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

// =============================================================================================================================

pub fn validate_date_not_in_past(date: &DateTime) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    let event_date_chrono = date.to_chrono();

    if event_date_chrono < now {
        let mut err = ValidationError::new("date_in_past");
        err.message = Some("The date cannot be in the past.".into());
        return Err(err);
    }

    Ok(())
}

// =============================================================================================================================

pub async fn trigger_notification(
    notification_data: TriggerNotificationRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let notification_data = json!(notification_data);

    let internal_token = encode_internal_jwt()?;
    let client = reqwest::Client::new();
    let res = client
        .post("http://notifications-service:8080/api/notifications")
        .header("Authorization", format!("Bearer {}", internal_token))
        .json(&notification_data)
        .send()
        .await?
        .json::<ApiResponse<TriggerNotificationResponse>>()
        .await?;

    match res {
        Success { .. } => Ok(()),
        Error { error, .. } => Err(error.into()),
    }
}

// =============================================================================================================================
