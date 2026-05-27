use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{
    use_intl_datetime_format, DateTimeStyle, DayFormat, MonthFormat, TimeNumericFormat,
    TimeZoneNameFormat, UseIntlDateTimeFormatOptions, WeekdayFormat, YearFormat,
};

fn parse_datetime_local(value: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M"))
        .ok()
        .map(|naive_date_time| Utc.from_utc_datetime(&naive_date_time))
}

#[component]
fn Demo() -> impl IntoView {
    let (date, set_date) = signal(Utc.with_ymd_and_hms(2020, 12, 20, 19, 30, 00).unwrap());

    let input_value = move || date.get().format("%Y-%m-%dT%H:%M:%S").to_string();

    let english_formatter = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("en-US")
            .weekday(WeekdayFormat::Long)
            .year(YearFormat::Numeric)
            .month(MonthFormat::Long)
            .day(DayFormat::Numeric)
            .hour(TimeNumericFormat::Numeric)
            .minute(TimeNumericFormat::TwoDigit)
            .second(TimeNumericFormat::TwoDigit)
            .time_zone("UTC")
            .time_zone_name(TimeZoneNameFormat::Short),
    );
    let formatted_in_english = english_formatter.format(date);

    let german_formatter = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("de-DE")
            .date_style(DateTimeStyle::Full)
            .time_style(DateTimeStyle::Medium)
            .time_zone("UTC"),
    );
    let formatted_in_german = german_formatter.format(date);

    let japanese_formatter = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("ja-JP")
            .date_style(DateTimeStyle::Long)
            .time_style(DateTimeStyle::Short)
            .time_zone("UTC"),
    );
    let formatted_in_japanese = japanese_formatter.format(date);

    let arabic_formatter = use_intl_datetime_format(
        UseIntlDateTimeFormatOptions::default()
            .locale("ar-EG")
            .date_style(DateTimeStyle::Full)
            .time_zone("UTC"),
    );
    let formatted_in_arabic = arabic_formatter.format(date);

    view! {
        <input
            class="block"
            prop:value=input_value
            on:input=move |event| {
                if let Some(parsed_date) = parse_datetime_local(&event_target_value(&event)) {
                    set_date.set(parsed_date);
                }
            }
            type="datetime-local"
            step="1"
        />
        <p>"English (en-US): " {formatted_in_english}</p>
        <p>"German (de-DE): " {formatted_in_german}</p>
        <p>"Japanese (ja-JP): " {formatted_in_japanese}</p>
        <p>"Arabic (ar-EG): " {formatted_in_arabic}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
