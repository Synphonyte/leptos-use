#![cfg(all(target_arch = "wasm32", feature = "chrono"))]

use any_spawner::Executor;
use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_use_calendar::{
    CalendarDate, UseCalendarOptions, UseCalendarReturn, use_calendar_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn ymd(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn counts(dates: &[CalendarDate<NaiveDate>]) -> (usize, usize, usize) {
    let previous = dates
        .iter()
        .filter(|d| matches!(d, CalendarDate::Previous(_)))
        .count();
    let current = dates
        .iter()
        .filter(|d| matches!(d, CalendarDate::Current(_)))
        .count();
    let next = dates
        .iter()
        .filter(|d| matches!(d, CalendarDate::Next(_)))
        .count();
    (previous, current, next)
}

#[wasm_bindgen_test]
async fn lays_out_whole_weeks_around_the_month() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    // October 2026 starts on a Thursday and ends on a Saturday.
    let UseCalendarReturn {
        dates, weekdays, ..
    } = use_calendar_with_options(
        UseCalendarOptions::default().initial_date(Some(ymd(2026, 10, 13))),
    );

    let dates = dates.get_untracked();
    assert_eq!(dates.len() % 7, 0, "whole weeks");
    assert_eq!(
        counts(&dates),
        (3, 31, 1),
        "weeks start on Monday by default"
    );
    assert_eq!(*dates[0], ymd(2026, 9, 28));
    assert_eq!(*dates[3], ymd(2026, 10, 1));
    assert_eq!(*dates[dates.len() - 1], ymd(2026, 11, 1));
    assert_eq!(weekdays.get_untracked(), vec![0, 1, 2, 3, 4, 5, 6]);
}

#[wasm_bindgen_test]
async fn starts_weeks_on_the_configured_day() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseCalendarReturn {
        dates, weekdays, ..
    } = use_calendar_with_options(
        UseCalendarOptions::default()
            .initial_date(Some(ymd(2026, 10, 13)))
            .first_day_of_the_week(6),
    );

    let dates = dates.get_untracked();
    assert_eq!(counts(&dates), (4, 31, 0));
    assert_eq!(*dates[0], ymd(2026, 9, 27), "a Sunday");
    assert_eq!(weekdays.get_untracked(), vec![6, 0, 1, 2, 3, 4, 5]);
}

#[wasm_bindgen_test]
async fn navigates_between_months() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseCalendarReturn {
        dates,
        previous_month,
        next_month,
        month_by_date,
        ..
    } = use_calendar_with_options(
        UseCalendarOptions::default().initial_date(Some(ymd(2026, 1, 31))),
    );

    let first_current = |dates: Vec<CalendarDate<NaiveDate>>| {
        dates
            .into_iter()
            .find(|d| matches!(d, CalendarDate::Current(_)))
            .map(|d| *d)
            .unwrap()
    };

    next_month();
    assert_eq!(first_current(dates.get_untracked()), ymd(2026, 2, 1));
    assert_eq!(counts(&dates.get_untracked()).1, 28);

    previous_month();
    previous_month();
    assert_eq!(first_current(dates.get_untracked()), ymd(2025, 12, 1));

    month_by_date(&ymd(2024, 2, 10));
    assert_eq!(counts(&dates.get_untracked()).1, 29, "leap year");
}

#[wasm_bindgen_test]
fn calendar_dates_answer_questions_about_themselves() {
    let first = CalendarDate::Current(ymd(2026, 10, 1));
    let last = CalendarDate::Current(ymd(2026, 10, 31));
    let other = CalendarDate::Previous(ymd(2026, 9, 30));

    assert!(first.is_first_day_of_month());
    assert!(last.is_last_day_of_month());
    assert!(other.is_other_month());
    assert!(!first.is_other_month());
    assert!(first.is_selected(&ymd(2026, 10, 1)));
    assert!(first.is_before(&ymd(2026, 10, 2)));
    assert!(last.is_after(&ymd(2026, 10, 2)));
    assert!(
        CalendarDate::Current(ymd(2026, 10, 15)).is_between(&ymd(2026, 10, 1), &ymd(2026, 10, 31))
    );
}
