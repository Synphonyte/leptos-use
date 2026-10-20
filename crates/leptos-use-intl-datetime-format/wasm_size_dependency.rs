//! Uses `chrono` like `use_intl_datetime_format` does: converting a `DateTime` to a timestamp.
//! Used to measure the wasm size of the crate only.

use chrono::{TimeZone, Utc};
use leptos::prelude::*;
use std::hint::black_box;

#[component]
fn DependencyDemo() -> impl IntoView {
    let date = Utc
        .timestamp_millis_opt(black_box(1_700_000_000_000))
        .unwrap();

    view! { <p>{date.timestamp_millis() as f64}</p> }
}
