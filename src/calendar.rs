use chrono::{NaiveDate, NaiveTime};
use getset::{Getters, Setters};
use leptonic::components::date_selector::DateSelector;
use time::OffsetDateTime;

use leptos_router::MultiActionForm;
use serde::{Deserialize, Serialize};

use leptos::*;

use crate::error_template::ErrorTemplate;

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

/// Renders the home page of your application.
#[component]
pub fn Calendar() -> impl IntoView {
    let add_calendar_entry = create_server_multi_action::<AddCalendarEntry>();
    let entries = create_resource(
        move || add_calendar_entry.version().get(),
        move |_| calendar(),
    );

    // let mut current_date = None;
    view! {
        <div>
            <DateSelector value=OffsetDateTime::now_utc() on_change=move |v| {
                // tracing::info!(v);
            }/>

            <MultiActionForm action=add_calendar_entry>
                <label>
                    "Add a CalendarEntry"
                    <input type="text" name="title"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                    {move || {
                        entries.get()
                               .map(move |entries| match entries {
                                   Err(e) => {
                                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                   }
                                   Ok(entries) => {
                                       entries.into_iter()
                                              .map(move |entry|
                                                   view! {
                                                       <p class="text-amber-600">{entry.title()}</p>

                                       }).collect_view()
                                   }
                    }).unwrap_or_default()
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[server(AddCalendarEntry, "/api")]
async fn add_calendar_entry(
    title: String,
    // start_date: NaiveDate,
    // start_time: Option<NaiveTime>,
    // end_date: Option<NaiveDate>,
    // end_time: Option<NaiveTime>,
) -> Result<(), ServerFnError> {
    use crate::calendar::ssr::*;
    let mut conn = db().await?;

    let start_date = chrono::Local::now().date_naive();
    let start_time: Option<NaiveTime> = None;
    let end_date: Option<NaiveDate> = None;
    let end_time: Option<NaiveTime> = None;
    logging::log!("{}", start_date);

    match sqlx::query("INSERT INTO calendar_entries (title, start_date, start_time, end_date, end_time) VALUES ($1, $2, $3, $4, $5)")
        .bind(title).bind(start_date).bind(start_time).bind(end_date).bind(end_time)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server]
async fn calendar() -> Result<Vec<CalendarEntry>, ServerFnError> {
    use crate::calendar::ssr::db;
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut calendar_entries = Vec::new();
    let mut rows =
        sqlx::query_as::<_, CalendarEntry>("SELECT * FROM calendar_entries").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        calendar_entries.push(row);
    }

    // Lines below show how to set status code and headers on the response
    // let resp = expect_context::<ResponseOptions>();
    // resp.set_status(StatusCode::IM_A_TEAPOT);
    // resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

    Ok(calendar_entries)
}
