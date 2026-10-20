#![cfg(target_arch = "wasm32")]

use chrono::{TimeZone, Utc};
use leptos::prelude::*;
use leptos_use_intl_datetime_format::{
    DateTimeStyle, MonthFormat, UseIntlDateTimeFormatOptions, use_intl_datetime_format,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn formats_dates_for_the_locale() {
    let owner = Owner::new();
    owner.set();

    let (date, set_date) = signal(Utc.with_ymd_and_hms(2020, 12, 20, 3, 23, 16).unwrap());

    let en = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("en-US")
            .time_zone("UTC"),
    );
    let de = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("de-DE")
            .time_zone("UTC"),
    );

    let en_formatted = en.format(date);
    assert_eq!(en_formatted.get_untracked(), "12/20/2020");
    assert_eq!(de.format(date).get_untracked(), "20.12.2020");

    set_date.set(Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 0).unwrap());
    assert_eq!(en_formatted.get_untracked(), "1/2/2021");
}

#[wasm_bindgen_test]
fn formats_with_styles_and_components() {
    let owner = Owner::new();
    owner.set();

    let date = Utc.with_ymd_and_hms(2020, 12, 20, 3, 23, 16).unwrap();

    let long = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("en-US")
            .time_zone("UTC")
            .date_style(DateTimeStyle::Long),
    );
    assert_eq!(long.format(date).get_untracked(), "December 20, 2020");

    let month = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("en-US")
            .time_zone("UTC")
            .month(MonthFormat::Short),
    );
    assert_eq!(month.format(date).get_untracked(), "Dec");
}

#[wasm_bindgen_test]
fn formats_ranges() {
    let owner = Owner::new();
    owner.set();

    let format = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("en-US")
            .time_zone("UTC"),
    );
    let start = Utc.with_ymd_and_hms(2020, 12, 20, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2020, 12, 24, 0, 0, 0).unwrap();

    assert_eq!(
        format.format_range(start, end).get_untracked(),
        "12/20/2020 – 12/24/2020"
    );
}
