use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

///Represents a go link, a mapping from source -> sink with some metadata
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Redirect {
    pub source: String,
    pub sink: String,
    ///number of times this link has been used
    pub usages: i32,

    ///Last time this link was used. Serialized as seconds since epoch
    #[serde(with = "ts_seconds_option")]
    pub last_used: Option<DateTime<Utc>>,

    ///When this link was created. Serialized as seconds since epoch
    #[serde(with = "ts_seconds_option")]
    pub created: Option<DateTime<Utc>>,

    pub author: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GoPair {
    pub source: String,
    pub sink: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GoPairAuthor {
    pub source: String,
    pub sink: String,
    pub author: String,
}
