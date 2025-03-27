use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{DateTime, oid::ObjectId};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::Error as SerError;
use serde::{self, Deserialize, Serializer};
use serde_json::json;
use std::fmt;
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
            formatter.write_str("a BSON DateTime, an ISO8601 string, or a map with $date")
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime, E>
        where
            E: de::Error,
        {
            let chrono_dt = chrono::DateTime::parse_from_rfc3339(value).map_err(E::custom)?;
            let ts = chrono_dt.with_timezone(&chrono::Utc).timestamp_millis();
            Ok(DateTime::from_millis(ts))
        }

        fn visit_i64<E>(self, value: i64) -> Result<DateTime, E>
        where
            E: de::Error,
        {
            Ok(DateTime::from_millis(value))
        }

        fn visit_map<M>(self, mut map: M) -> Result<DateTime, M::Error>
        where
            M: MapAccess<'de>,
        {
            let key: Option<String> = map.next_key()?;
            if let Some(key) = key {
                if key == "$date" {
                    let value: serde_json::Value = map.next_value()?;
                    if let Some(s) = value.as_str() {
                        let chrono_dt =
                            chrono::DateTime::parse_from_rfc3339(s).map_err(de::Error::custom)?;
                        let ts = chrono_dt.with_timezone(&chrono::Utc).timestamp_millis();
                        return Ok(DateTime::from_millis(ts));
                    } else if let Some(n) = value.as_i64() {
                        return Ok(DateTime::from_millis(n));
                    } else if let Some(obj) = value.as_object() {
                        if let Some(number_long) = obj.get("$numberLong") {
                            if let Some(s) = number_long.as_str() {
                                let n = s.parse::<i64>().map_err(de::Error::custom)?;
                                return Ok(DateTime::from_millis(n));
                            } else if let Some(n) = number_long.as_i64() {
                                return Ok(DateTime::from_millis(n));
                            }
                        }
                        return Err(de::Error::custom("unexpected type for nested $date value"));
                    } else {
                        return Err(de::Error::custom("unexpected type for $date value"));
                    }
                } else {
                    return Err(de::Error::custom("expected key \"$date\""));
                }
            }
            Err(de::Error::custom("expected a map with \"$date\""))
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

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let dt = deserialize_datetime_from_any(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(dt))
        }
    }

    deserializer.deserialize_option(OptionDateTimeVisitor)
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

pub fn serialize_datetime_as_rfc3339_string<S>(
    date: &DateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.try_to_rfc3339_string().map_err(SerError::custom)?;
    serializer.serialize_str(&s)
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
