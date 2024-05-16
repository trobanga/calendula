use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::Sqlite> for DateOrDateTime {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Encode<'_, sqlx::Sqlite> for DateOrDateTime {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<sqlx::Sqlite>>::encode(self.to_string(), buf)
    }

    fn encode(
        self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<sqlx::Sqlite>>::encode(self.to_string(), buf)
    }
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for DateOrDateTime {
    fn decode(value: sqlx::sqlite::SqliteValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let date_or_datetime = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        let date = NaiveDate::parse_from_str(&date_or_datetime, "%d.%m.%Y");
        if let Ok(date) = date {
            Ok(DateOrDateTime::Date(date))
        } else {
            let datetime = NaiveDateTime::parse_from_str(&date_or_datetime, "%d.%m.%Y - %H:%M");
            if let Ok(dt) = datetime {
                Ok(DateOrDateTime::DateTime(dt))
            } else {
                leptos::logging::log!("Cannot decode date(time).");
                todo!()
            }
        }
    }
}

impl PartialOrd for DateOrDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DateOrDateTime::Date(d), DateOrDateTime::Date(o)) => Some(d.cmp(o)),
            (DateOrDateTime::DateTime(d), DateOrDateTime::DateTime(o)) => Some(d.cmp(o)),
            (DateOrDateTime::Date(d), DateOrDateTime::DateTime(o)) => Some(d.cmp(&o.date())),
            (DateOrDateTime::DateTime(d), DateOrDateTime::Date(o)) => Some(d.date().cmp(o)),
        }
    }
}

impl fmt::Display for DateOrDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateOrDateTime::Date(d) => write!(f, "{}", d.format("%d.%m.%Y")),
            DateOrDateTime::DateTime(dt) => write!(f, "{}", dt.format("%d.%m.%Y - %H:%M")),
        }
    }
}
