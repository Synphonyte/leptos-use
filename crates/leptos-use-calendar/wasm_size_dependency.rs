//! Uses `chrono` (the default date backend) like `use_calendar` does. Used to measure the wasm
//! size of the crate only.

use chrono::{Datelike, Local, Months, NaiveDate, TimeZone};
use leptos::prelude::*;
use std::hint::black_box;

#[component]
fn DependencyDemo() -> impl IntoView {
    let today = Local::now().date_naive();
    let date = black_box(today)
        .checked_add_months(Months::new(black_box(1)))
        .and_then(|d| d.checked_sub_months(Months::new(black_box(2))))
        .and_then(|d| d.succ_opt())
        .and_then(|d| d.pred_opt())
        .unwrap_or(today);

    let local = Local
        .from_local_datetime(&date.and_hms_opt(12, 0, 0).unwrap_or_default())
        .earliest()
        .map(|d| d.date_naive())
        .unwrap_or(NaiveDate::MIN);

    view! {
        <p>
            {date.year()} {date.month()} {date.day()} {date.weekday().num_days_from_monday()}
            {local.day()}
        </p>
    }
}
