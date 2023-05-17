use chrono::serde::ts_seconds_option;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, PgPool,
};

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
}

/// Gets the redirect URL (sink) for a given source
pub async fn get_sink(source: &str, conn: &PgPool) -> Result<Option<String>, sqlx::Error> {
    sqlx::query!("SELECT sink FROM redirects WHERE source = $1", source)
        .fetch_optional(conn)
        .await
        .map(|s| s.map(|s| s.sink))
}

/// Gets the n most recently used redirects.
pub async fn get_recent(conn: &PgPool, n: i64) -> Result<Vec<Redirect>, sqlx::Error> {
    sqlx::query_as!(Redirect,"SELECT source, sink, usages, last_used, created FROM redirects ORDER BY last_used desc NULLS LAST LIMIT $1", n)
        .fetch_all(conn).await
}

/// Gets all redirects. Allocates a [`Vec`] for results so may cause a large allocation.
pub async fn get_all(conn: &PgPool) -> Result<Vec<Redirect>, sqlx::Error> {
    sqlx::query_as!(Redirect, "SELECT source, sink, usages, last_used, created FROM redirects ORDER BY last_used desc NULLS LAST")
        .fetch_all(conn).await
}

///Increments the count and updates the date for the given go link
pub async fn bump_count(source: &str, conn: &PgPool) -> Result<(), sqlx::Error> {
    //update usage info
    sqlx::query!(
        "UPDATE redirects SET usages = usages + 1, last_used=NOW() WHERE source=$1",
        source
    )
    .execute(conn)
    .await
    .map(|_| ())
}
