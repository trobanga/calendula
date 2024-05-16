use getset::{Getters, Setters};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use leptos::*;

use crate::error_template::ErrorTemplate;

mod date_or_datetime;
use date_or_datetime::DateOrDateTime;

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
    id: i32,
    title: String,
    start: DateOrDateTime,
    end: Option<DateOrDateTime>,
}

/// Renders the home page of your application.
#[component]
pub fn Calendar() -> impl IntoView {
    let (done, set_done) = create_signal(0);
    view! {
        <NewEntry set_done=set_done/>
        <CalendarEntries done=done set_done=set_done/>
    }
}

#[component]
pub fn CalendarEntries(done: ReadSignal<usize>, set_done: WriteSignal<usize>) -> impl IntoView {
    let entries_resource = create_resource(done, move |_| calendar());
    view! {
        <div class="mx-auto flex flex-col p-8">
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                    {move || {
                        entries_resource.get()
                               .map(move |entries| match entries {
                                   Err(e) => {
                                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                   }
                                   Ok(mut entries) => {
                                       sort_by_datetime(&mut entries);
                                       entries
                                           .into_iter()
                                              .map(move |entry| view!{<CalendarEntry entry=entry set_done=set_done/>})
                                              .collect_view()
                                   }
                    }).unwrap_or_default()
                    }}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

fn sort_by_datetime(entries: &mut Vec<CalendarEntry>) {
    entries.sort_by(|a, b| a.start().partial_cmp(b.start()).unwrap());
}

#[component]
pub fn CalendarEntry(entry: CalendarEntry, set_done: WriteSignal<usize>) -> impl IntoView {
    let entry_id = entry.id();
    view! {
        <div class="p-6 mx-auto px-8 w-1/2 min-w-fit space-x-4 rounded-xl shadow-lg bg-slate-100 relative my-5">
            <p class="text-amber-600">
                {entry.start().to_string()}
            </p>
            <br/>
            <p>{entry.title()}</p>
            <DeleteEntryButton id=*entry_id set_done=set_done/>
        </div>
    }
}

#[component]
pub fn DeleteEntryButton(id: i32, set_done: WriteSignal<usize>) -> impl IntoView {
    view! {
        <button
            class="absolute top-0 right-0 p-2"
            on:click=move |_| {
            let id = id;
            spawn_local(async move {
                if delete_calendar_entry(id).await.is_err() {logging::warn!("Cannot delete");};
                set_done.update(|x| *x += 1);
            });}>
            Delete
        </button>
    }
}

#[server(DeleteCalendarEntry, "/api")]
async fn delete_calendar_entry(id: i32) -> Result<(), ServerFnError> {
    use crate::calendar::ssr::*;

    logging::log!("Delete entry {id}");

    let mut conn = db().await?;
    match sqlx::query("delete from calendar_entries where id = ?")
        .bind(id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn NewEntry(set_done: WriteSignal<usize>) -> impl IntoView {
    use leptos::html::Input;
    let (value, set_value) = create_signal(false);

    let start_date_ref = create_node_ref::<Input>();
    let start_datetime_ref = create_node_ref::<Input>();
    let title_ref = create_node_ref::<Input>();
    let all_day_long_ref = create_node_ref::<Input>();

    view! {
        <div class="p-6 mx-auto px-8 w-1/2 min-w-fit space-x-4 rounded-xl shadow-lg flex bg-slate-100">
            <form class="space-y-4 mx-auto"
                on:submit=move |ev| {
                    ev.prevent_default(); // don't reload the page...
                        logging::log!("Submit");
                    let title = title_ref.get().expect("title to exist");
                    let all_day_long_checked = all_day_long_ref.get().expect("There should be a checkbox").checked();
                    let start = if all_day_long_checked {
                        let start = start_date_ref.get().expect("start to exist");
                        logging::log!("{}", start.value());
                        let start =  start.value().parse::<NaiveDate>().expect("this to be correct");
                        DateOrDateTime::Date(start)
                    } else {
                        let start = start_datetime_ref.get().expect("start to exist");
                        logging::log!("{}", start.value());
                        let start = NaiveDateTime::parse_from_str(&start.value(), "%Y-%m-%dT%H:%M").expect("this to be correct");
                        DateOrDateTime::DateTime(start)
                    };

                    spawn_local(async move {
                        add_calendar_entry(title.value(), start, None).await.expect("this to work");
                        title.set_value("");
                        set_done.update(|x| *x += 1);
                    });
                }
            >
                <label class="text-xl font-medium mx-auto">
                "Neuer Termin"
                </label>
                    <div>
                        <input class="" type="text" name="title" node_ref=title_ref/>
                    </div>
                    <div class="space-x-4">
                        {
                            let all_day_long_checked = if let Some(b) = all_day_long_ref.get_untracked() {b.checked()} else {false};
                            move || {
                                match value() {
                                    true => {
                                        let now = Local::now().date_naive().to_string();
                                        view! {
                                            <input type="date" name="start" value=&now min=&now node_ref=start_date_ref/>
                                        }.into_any()
                                    },
                                    false => {
                                        let now = Local::now().format("%Y-%m-%dT%H:%M").to_string();
                                        logging::log!("dt: {}", now);
                                        view! {
                                            <input type="datetime-local" name="start" value=&now min=&now node_ref=start_datetime_ref/>
                                        }.into_any()
                                    }
                                }
                            }
                        }
                        <label><input type="checkbox" name="with_time" node_ref=all_day_long_ref
                            on:input=move |_| set_value(all_day_long_ref
                                                        .get()
                                                        .map(|c| c.checked())
                                                        .unwrap_or_default())
                        />
                            All day long
                        </label>
                    </div>
                    <br/>
                    <div class="mx-auto flex">
                        <button class="mx-auto ring-2 p-2 rounded-xl" type="submit">"Hinzufügen"</button>
                    </div>
            </form>
        </div>
    }
}

#[server(AddCalendarEntry, "/api")]
async fn add_calendar_entry(
    title: String,
    start: DateOrDateTime,
    end: Option<DateOrDateTime>,
) -> Result<(), ServerFnError> {
    use crate::calendar::ssr::*;
    let mut conn = db().await?;

    // let end_datetime = start_time.and_then(|t| {
    //     start_date
    //         .and_time(t)
    //         .checked_add_signed(Duration::try_hours(1).unwrap())
    // });

    // let end_date = end_date.unwrap_or(end_datetime.map(|dt| dt.date()).unwrap_or(start_date));
    // let end_time: Option<NaiveTime> = end_time.or(end_datetime.map(|dt| dt.time()));
    let end: Option<DateOrDateTime> = None;

    logging::log!("start: {}", start);

    match sqlx::query("INSERT INTO calendar_entries (title, start, end) VALUES ($1, $2, $3)")
        .bind(title)
        .bind(start)
        .bind(end)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GetCalendarEntries, "/api")]
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
