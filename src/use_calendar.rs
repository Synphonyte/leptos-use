use crate::core::MaybeRwSignal;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::ops::Deref;

#[cfg(feature = "chrono")]
mod impl_chrono;
#[cfg(feature = "jiff")]
mod impl_jiff;
#[cfg(feature = "time")]
mod impl_time;

#[cfg(not(any(feature = "chrono", feature = "jiff", feature = "time")))]
const _: () = {
    #[deprecated(
        note = "The feature `use_calendar` is enabled without a date backend. Enable one of the features `chrono`, `jiff` or `time` (or implement `CalendarDateType` for your own date type)."
    )]
    const NO_DATE_BACKEND: () = ();
    NO_DATE_BACKEND
};

/// Create bare-bone calendar data to use in your component.
/// See [`UseCalendarOptions`] for options and [`UseCalendarReturn`] for return values.
///
/// The calendar is generic over the date type `D` which has to implement [`CalendarDateType`].
/// Implementations are provided behind feature flags. Enable at least one of them in addition
/// to the `use_calendar` feature:
///
/// | Feature  | Types                                                                                  |
/// |----------|----------------------------------------------------------------------------------------|
/// | `chrono` | `chrono::NaiveDate`, `chrono::DateTime<Utc>`, `chrono::DateTime<Local>`, `chrono::DateTime<FixedOffset>` |
/// | `jiff`   | `jiff::civil::Date`, `jiff::Zoned`                                                     |
/// | `time`   | `time::Date`, `time::OffsetDateTime`                                                   |
///
/// For date-time types the calendar works on the calendar day in the value's time zone.
/// All comparisons (today, selected, before, after, between) are done by calendar day,
/// ignoring the time of day. The generated dates keep the time of day of the displayed date
/// unless it doesn't exist on a day because of a DST transition, in which case it is shifted.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_calendar)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use chrono::NaiveDate;
/// # use leptos_use::{use_calendar, UseCalendarReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseCalendarReturn {
///     dates,
///     weekdays,
///     previous_month,
///     today,
///     month_by_date,
///     next_month
/// } = use_calendar::<NaiveDate>();
/// #
/// # view! {
/// # }
/// # }
/// ```
///
/// Use [`use_calendar_with_options`] to change the initial date and first day of the week.
///
/// ```
/// # use leptos::prelude::*;
/// # use chrono::NaiveDate;
/// # use leptos_use::{use_calendar_with_options, UseCalendarReturn, UseCalendarOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let initial_date = RwSignal::new(
///     Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
/// );
///
/// let options = UseCalendarOptions::default()
///     .first_day_of_the_week(6)
///     .initial_date(initial_date);
///
/// let UseCalendarReturn {
///     dates,
///     weekdays,
///     previous_month,
///     today,
///     month_by_date,
///     next_month
/// } = use_calendar_with_options(options);
/// #
/// # view! {
/// # }
/// # }
/// ```
///
/// ### Using `jiff` or `time`
///
/// With the feature `jiff` or `time` enabled you can use their date types instead.
///
/// ```
/// # #[cfg(feature = "jiff")]
/// # mod jiff_demo {
/// # use leptos::prelude::*;
/// # use leptos_use::{use_calendar, UseCalendarReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseCalendarReturn { dates, .. } = use_calendar::<jiff::civil::Date>();
/// #
/// # view! {
/// # }
/// # }
/// # }
/// ```
///
/// ```
/// # #[cfg(feature = "time")]
/// # mod time_demo {
/// # use leptos::prelude::*;
/// # use leptos_use::{use_calendar, UseCalendarReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseCalendarReturn { dates, .. } = use_calendar::<time::Date>();
/// #
/// # view! {
/// # }
/// # }
/// # }
/// ```
///
/// ### Custom date types
///
/// To use your own date type, implement [`CalendarDateType`] for it. This works without enabling
/// any of the backend features. Here is an example with a new-type wrapper around a `chrono` date:
///
/// ```
/// # #[cfg(feature = "chrono")]
/// # mod custom_demo {
/// # use leptos::prelude::*;
/// use chrono::{Datelike, Local, Months, NaiveDate};
/// use leptos_use::{use_calendar, CalendarDateType, UseCalendarReturn};
///
/// #[derive(Clone, Copy, PartialEq)]
/// pub struct MyDate(NaiveDate);
///
/// impl CalendarDateType for MyDate {
///     fn today() -> Self {
///         MyDate(Local::now().date_naive())
///     }
///
///     fn ymd(&self) -> (i32, u8, u8) {
///         (self.0.year(), self.0.month() as u8, self.0.day() as u8)
///     }
///
///     fn prev_day(&self) -> Option<Self> {
///         self.0.pred_opt().map(MyDate)
///     }
///
///     fn next_day(&self) -> Option<Self> {
///         self.0.succ_opt().map(MyDate)
///     }
///
///     fn weekday_from_monday(&self) -> u8 {
///         self.0.weekday().num_days_from_monday() as u8
///     }
///
///     fn add_months(&self, months: i32) -> Option<Self> {
///         let date = if months >= 0 {
///             self.0.checked_add_months(Months::new(months as u32))
///         } else {
///             self.0.checked_sub_months(Months::new(months.unsigned_abs()))
///         };
///         date.map(MyDate)
///     }
/// }
///
/// #[component]
/// fn Demo() -> impl IntoView {
///     let UseCalendarReturn { dates, .. } = use_calendar::<MyDate>();
///     // ...
/// #     view! { }
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// Not tested yet.
// #[doc(cfg(feature = "use_calendar"))]
pub fn use_calendar<D>() -> UseCalendarReturn<
    D,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn(&D) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    D: CalendarDateType,
{
    use_calendar_with_options(UseCalendarOptions::default())
}

/// Version of [`use_calendar`] that takes a [`UseCalendarOptions`]. See [`use_calendar`] for how to use.
// #[doc(cfg(feature = "use_calendar"))]
pub fn use_calendar_with_options<D>(
    options: UseCalendarOptions<D>,
) -> UseCalendarReturn<
    D,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn(&D) + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    D: CalendarDateType,
{
    let UseCalendarOptions {
        initial_date: date,
        first_day_of_the_week,
    } = options;
    let (date, _set_date) = date.into_signal();

    let show_date = RwSignal::new(date.get_untracked().unwrap_or_else(D::today));
    Effect::new(move |_| {
        if let Some(selected_date) = date.get()
            && !show_date.with_untracked(|show_date| is_same_month(&selected_date, show_date))
        {
            show_date.set(selected_date);
        }
    });

    let dates = Memo::new(move |_| {
        show_date.with(|show_date| calendar_dates(show_date, first_day_of_the_week.get()))
    });

    let weekdays = Memo::<Vec<usize>>::new(move |_| {
        let first_day_of_the_week = first_day_of_the_week.get();
        if first_day_of_the_week < 7 {
            let first_weekdays = first_day_of_the_week..7;
            let last_weekdays = 0..first_day_of_the_week;
            first_weekdays.chain(last_weekdays).collect()
        } else {
            (0..7).collect()
        }
    });

    UseCalendarReturn {
        previous_month: move || {
            show_date.update(|date| {
                if let Some(new_date) = date.add_months(-1) {
                    *date = new_date;
                }
            });
        },
        today: move || {
            show_date.set(D::today());
        },
        month_by_date: move |new_date: &D| {
            show_date.set(new_date.clone());
        },

        next_month: move || {
            show_date.update(|date| {
                if let Some(new_date) = date.add_months(1) {
                    *date = new_date;
                }
            });
        },
        weekdays: weekdays.into(),
        dates: dates.into(),
    }
}

/// Computes all dates of the month of `show_date`, padded with the dates of the previous and
/// next month so that the result starts at `first_day_of_the_week` and ends with full weeks.
fn calendar_dates<D: CalendarDateType>(
    show_date: &D,
    first_day_of_the_week: usize,
) -> Vec<CalendarDate<D>> {
    let first_day_of_the_week = if first_day_of_the_week < 7 {
        first_day_of_the_week as u8
    } else {
        0
    };
    let days_since_first_day_of_the_week =
        |date: &D| (date.weekday_from_monday() + 7 - first_day_of_the_week) % 7;

    let mut dates = vec![];

    let mut current_date = show_date.clone();
    while let Some(date) = current_date.prev_day() {
        if !is_same_month(&date, show_date) {
            break;
        }
        dates.push(CalendarDate::Current(date.clone()));
        current_date = date;
    }
    for _ in 0..days_since_first_day_of_the_week(&current_date) {
        let Some(date) = current_date.prev_day() else {
            break;
        };
        dates.push(CalendarDate::Previous(date.clone()));
        current_date = date;
    }
    dates.reverse();

    dates.push(CalendarDate::Current(show_date.clone()));

    current_date = show_date.clone();
    while let Some(date) = current_date.next_day() {
        if !is_same_month(&date, show_date) {
            break;
        }
        dates.push(CalendarDate::Current(date.clone()));
        current_date = date;
    }
    for _ in 0..6 - days_since_first_day_of_the_week(&current_date) {
        let Some(date) = current_date.next_day() else {
            break;
        };
        dates.push(CalendarDate::Next(date.clone()));
        current_date = date;
    }

    dates
}

fn is_same_month<D: CalendarDateType>(a: &D, b: &D) -> bool {
    let (a_year, a_month, _) = a.ymd();
    let (b_year, b_month, _) = b.ymd();
    a_year == b_year && a_month == b_month
}

/// Date type that can be used with [`use_calendar`].
///
/// Implementations are provided for the date types of `chrono`, `jiff` and `time`
/// behind the respective feature flags. See [`use_calendar`] for a list.
///
/// For date-time types all methods operate on the calendar day in the value's time zone.
pub trait CalendarDateType: Clone + PartialEq + Send + Sync + 'static {
    /// Today in the local time zone.
    fn today() -> Self;

    /// Returns `(year, month, day)` with month in `1..=12` and day in `1..=31`.
    /// All date comparisons of [`CalendarDate`] are done using this.
    fn ymd(&self) -> (i32, u8, u8);

    /// The previous calendar day or `None` if out of range.
    fn prev_day(&self) -> Option<Self>;

    /// The next calendar day or `None` if out of range.
    fn next_day(&self) -> Option<Self>;

    /// The weekday as a number from 0 (Monday) to 6 (Sunday).
    fn weekday_from_monday(&self) -> u8;

    /// Adds (or subtracts if negative) `months` months. If the day doesn't exist in the
    /// resulting month it is clamped to the last day of that month.
    /// Returns `None` if out of range.
    fn add_months(&self, months: i32) -> Option<Self>;
}

/// Options for [`use_calendar_with_options`].
// #[doc(cfg(feature = "use_calendar"))]
#[derive(DefaultBuilder)]
pub struct UseCalendarOptions<D>
where
    D: CalendarDateType,
{
    /// Date being used to initialize the calendar month to be displayed.
    /// Defaults to [`CalendarDateType::today`].
    #[builder(skip)]
    pub initial_date: MaybeRwSignal<Option<D>>,
    /// First day of the week as a number from 0 to 6. Defaults to 0 (Monday).
    #[builder(into)]
    pub first_day_of_the_week: Signal<usize>,
}

impl<D> Default for UseCalendarOptions<D>
where
    D: CalendarDateType,
{
    fn default() -> Self {
        Self {
            initial_date: Some(D::today()).into(),
            first_day_of_the_week: 0.into(),
        }
    }
}

impl<D> UseCalendarOptions<D>
where
    D: CalendarDateType,
{
    /// Date being used to initialize the calendar month to be displayed.
    /// Defaults to [`CalendarDateType::today`].
    pub fn initial_date(self, value: impl Into<MaybeRwSignal<Option<D>>>) -> Self {
        Self {
            initial_date: value.into(),
            ..self
        }
    }
}

/// Return type of [`use_calendar`].
// #[doc(cfg(feature = "use_calendar"))]
pub struct UseCalendarReturn<D, PreviousMonthFn, TodayFn, MonthByDateFn, NextMonthFn>
where
    D: CalendarDateType,
    PreviousMonthFn: Fn() + Clone + Send + Sync,
    TodayFn: Fn() + Clone + Send + Sync,
    MonthByDateFn: Fn(&D) + Clone + Send + Sync,
    NextMonthFn: Fn() + Clone + Send + Sync,
{
    /// A function to go to the previous month.
    pub previous_month: PreviousMonthFn,
    /// A function to go to the current month.
    pub today: TodayFn,
    /// A function to go to the month by date.
    pub month_by_date: MonthByDateFn,
    /// A function to go to the next month.
    pub next_month: NextMonthFn,
    /// The first day of the week as a number from 0 to 6.
    pub weekdays: Signal<Vec<usize>>,
    /// A `Vec` of [`CalendarDate`]s representing the dates in the current month.
    pub dates: Signal<Vec<CalendarDate<D>>>,
}

/// Utility enum to represent a calendar date. Implements [`Deref`] to the wrapped date type.
///
/// All comparison methods compare by calendar day (see [`CalendarDateType::ymd`]).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CalendarDate<D> {
    Previous(D),
    Current(D),
    Next(D),
}

impl<D> CalendarDate<D>
where
    D: CalendarDateType,
{
    pub fn is_other_month(&self) -> bool {
        match self {
            CalendarDate::Previous(_) | CalendarDate::Next(_) => true,
            CalendarDate::Current(_) => false,
        }
    }

    pub fn is_today(&self) -> bool {
        self.ymd() == D::today().ymd()
    }

    pub fn is_selected(&self, selected_date: &D) -> bool {
        self.ymd() == selected_date.ymd()
    }

    pub fn is_before(&self, date: &D) -> bool {
        self.ymd() < date.ymd()
    }

    pub fn is_between(&self, start_date: &D, end_date: &D) -> bool {
        let date = self.ymd();
        date >= start_date.ymd() && date <= end_date.ymd()
    }

    pub fn is_between_current_month(&self, start_date: &D, end_date: &D) -> bool {
        let start_date = start_date.ymd();
        let end_date = end_date.ymd();
        match self {
            CalendarDate::Current(date) => {
                let date = date.ymd();
                date >= start_date && date <= end_date
            }
            CalendarDate::Next(date) => {
                let date = date.ymd();
                date > start_date && date <= end_date
            }
            CalendarDate::Previous(date) => {
                let date = date.ymd();
                date >= start_date && date < end_date
            }
        }
    }

    pub fn is_after(&self, date: &D) -> bool {
        self.ymd() > date.ymd()
    }

    pub fn is_first_day_of_month(&self) -> bool {
        let date = self.deref();
        if let Some(prev_date) = date.prev_day() {
            !is_same_month(date, &prev_date)
        } else {
            true
        }
    }

    pub fn is_last_day_of_month(&self) -> bool {
        let date = self.deref();
        if let Some(next_date) = date.next_day() {
            !is_same_month(date, &next_date)
        } else {
            true
        }
    }
}

impl<D> Deref for CalendarDate<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        match self {
            CalendarDate::Previous(date)
            | CalendarDate::Current(date)
            | CalendarDate::Next(date) => date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid<D: CalendarDateType>(
        show_date: D,
        first_day_of_the_week: usize,
    ) -> Vec<(char, (i32, u8, u8))> {
        calendar_dates(&show_date, first_day_of_the_week)
            .into_iter()
            .map(|date| {
                let kind = match date {
                    CalendarDate::Previous(_) => 'p',
                    CalendarDate::Current(_) => 'c',
                    CalendarDate::Next(_) => 'n',
                };
                (kind, date.ymd())
            })
            .collect()
    }

    /// Expected grid computed independently of any date library.
    fn expected_grid(
        year: i32,
        month: u8,
        first_day_of_the_week: usize,
    ) -> Vec<(char, (i32, u8, u8))> {
        fn is_leap(year: i32) -> bool {
            (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
        }
        fn days_in_month(year: i32, month: u8) -> u8 {
            match month {
                2 if is_leap(year) => 29,
                2 => 28,
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            }
        }
        // Sakamoto's algorithm
        fn weekday_from_monday(year: i32, month: u8, day: u8) -> u8 {
            let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
            let y = if month < 3 { year - 1 } else { year };
            let sunday_based =
                (y + y / 4 - y / 100 + y / 400 + t[month as usize - 1] + day as i32) % 7;
            ((sunday_based + 6) % 7) as u8
        }

        let first = first_day_of_the_week as u8;
        let (prev_year, prev_month) = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let mut result = vec![];
        let leading = (weekday_from_monday(year, month, 1) + 7 - first) % 7;
        let prev_len = days_in_month(prev_year, prev_month);
        for day in (prev_len - leading + 1)..=prev_len {
            result.push(('p', (prev_year, prev_month, day)));
        }
        let len = days_in_month(year, month);
        for day in 1..=len {
            result.push(('c', (year, month, day)));
        }
        let trailing = 6 - (weekday_from_monday(year, month, len) + 7 - first) % 7;
        for day in 1..=trailing {
            result.push(('n', (next_year, next_month, day)));
        }
        result
    }

    /// Checks grid generation, month arithmetic and comparisons for a date type.
    /// `make` builds a value of `D` from `(year, month, day)`.
    pub(super) fn check_date_type<D: CalendarDateType>(make: impl Fn(i32, u8, u8) -> D) {
        for (year, month, day) in [
            (2026, 2, 15),
            (2024, 2, 29),
            (2026, 3, 1),
            (2026, 3, 31),
            (2025, 12, 24),
            (2026, 1, 1),
            (2026, 10, 25),
        ] {
            for first_day_of_the_week in 0..7 {
                assert_eq!(
                    grid(make(year, month, day), first_day_of_the_week),
                    expected_grid(year, month, first_day_of_the_week),
                    "{year}-{month}-{day} first day {first_day_of_the_week}",
                );
            }
        }

        // invalid first day of the week falls back to Monday
        assert_eq!(grid(make(2026, 2, 15), 9), expected_grid(2026, 2, 0));

        let add_months =
            |(year, month, day), months| make(year, month, day).add_months(months).unwrap().ymd();
        assert_eq!(add_months((2026, 1, 31), 1), (2026, 2, 28));
        assert_eq!(add_months((2024, 1, 31), 1), (2024, 2, 29));
        assert_eq!(add_months((2026, 3, 31), -1), (2026, 2, 28));
        assert_eq!(add_months((2026, 1, 15), -1), (2025, 12, 15));
        assert_eq!(add_months((2025, 12, 15), 1), (2026, 1, 15));
        assert_eq!(add_months((2026, 5, 10), -17), (2024, 12, 10));

        let date = CalendarDate::Current(make(2026, 2, 15));
        assert!(date.is_selected(&make(2026, 2, 15)));
        assert!(date.is_before(&make(2026, 2, 16)));
        assert!(date.is_after(&make(2026, 2, 14)));
        assert!(date.is_between(&make(2026, 2, 15), &make(2026, 2, 15)));
        assert!(!date.is_first_day_of_month());
        assert!(CalendarDate::Current(make(2026, 2, 1)).is_first_day_of_month());
        assert!(CalendarDate::Current(make(2026, 2, 28)).is_last_day_of_month());

        assert!(CalendarDate::Current(D::today()).is_today());
    }
}
