use chrono::{NaiveDate, NaiveTime};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:calendula.db").await?)
    }
}

#[derive(Debug, Getters, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[getset(get = "pub", set = "pub")]
pub struct CalendarEntry {
    title: String,
    start_date: NaiveDate,
    start_time: Option<NaiveTime>,
    end_date: Option<NaiveDate>,
    end_time: Option<NaiveTime>,
}
