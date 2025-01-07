use crate::core::MaybeRwSignal;
use chrono::*;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::ops::Deref;

/// Create bare-bone calendar data to use in your component.
/// See [`UseCalendarOptions`] for options and [`UseCalendarReturn`] for return values.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/leptos-0.6/examples/use_calendar)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_calendar, UseCalendarReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseCalendarReturn {
///     dates,
///     weekdays,
///     previous_month,
///     today,
///     next_month
/// } = use_calendar();
/// #
/// # view! {
/// # }
/// # }
/// ```
///
/// Use [`use_calendar_with_options`] to change the initial date and first day of the week.
///
/// ```
/// # use leptos::*;
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
///     next_month
/// } = use_calendar_with_options(options);
/// #
/// # view! {
/// # }
/// # }
/// ```
///
///
/// ## Server-Side Rendering
///
/// Not tested yet.
// #[doc(cfg(feature = "use_calendar"))]
pub fn use_calendar() -> UseCalendarReturn<impl Fn() + Clone, impl Fn() + Clone, impl Fn() + Clone>
{
    use_calendar_with_options(UseCalendarOptions::default())
}

/// Version of [`use_calendar`] that takes a [`UseCalendarOptions`]. See [`use_calendar`] for how to use.
// #[doc(cfg(feature = "use_calendar"))]
pub fn use_calendar_with_options(
    options: UseCalendarOptions,
) -> UseCalendarReturn<impl Fn() + Clone, impl Fn() + Clone, impl Fn() + Clone> {
    let UseCalendarOptions {
        initial_date: date,
        first_day_of_the_week,
    } = options;
    let (date, _set_date) = date.into_signal();
    let first_day_of_the_week = first_day_of_the_week.into_signal();

    let show_date = RwSignal::new(date.get_untracked().unwrap_or(Local::now().date_naive()));
    Effect::new(move |_| {
        if let Some(selected_date) = date.get() {
            let show_date_data = show_date.get_untracked();
            if selected_date.year() != show_date_data.year()
                || selected_date.month() != show_date_data.month()
            {
                show_date.set(selected_date);
            }
        }
    });

    let dates = Memo::new(move |_| {
        let show_date = show_date.get();
        let show_date_month = show_date.month();
        let mut dates = vec![];

        let mut current_date = show_date;
        let mut current_weekday_number = None::<u32>;
        loop {
            let date = current_date - Days::new(1);
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(
                        current_date.weekday().days_since(
                            Weekday::try_from(first_day_of_the_week.get() as u8)
                                .unwrap_or(Weekday::Mon),
                        ),
                    );
                }
                let weekday_number = current_weekday_number.unwrap();
                if weekday_number == 0 {
                    break;
                }
                current_weekday_number = Some(weekday_number - 1);

                dates.push(CalendarDate::Previous(date));
            } else {
                dates.push(CalendarDate::Current(date));
            }
            current_date = date;
        }
        dates.reverse();
        dates.push(CalendarDate::Current(show_date));
        current_date = show_date;
        current_weekday_number = None;
        loop {
            let date = current_date + Days::new(1);
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(
                        current_date.weekday().days_since(
                            Weekday::try_from(first_day_of_the_week.get() as u8)
                                .unwrap_or(Weekday::Mon),
                        ),
                    );
                }
                let weekday_number = current_weekday_number.unwrap();
                if weekday_number == 6 {
                    break;
                }
                current_weekday_number = Some(weekday_number + 1);
                dates.push(CalendarDate::Next(date));
            } else {
                dates.push(CalendarDate::Current(date));
            }
            current_date = date;
        }
        dates
    });

    let weekdays = Memo::<Vec<usize>>::new(move |_| {
        if Weekday::try_from(first_day_of_the_week.get() as u8).is_ok() {
            let first_weekdays = first_day_of_the_week.get()..7;
            let last_weekdays = 0..first_day_of_the_week.get();
            first_weekdays.chain(last_weekdays).collect()
        } else {
            (0..7).collect()
        }
    });

    UseCalendarReturn {
        previous_month: move || {
            show_date.update(|date| {
                *date = *date - Months::new(1);
            });
        },
        today: move || {
            show_date.set(Local::now().date_naive());
        },
        next_month: move || {
            show_date.update(|date| {
                *date = *date + Months::new(1);
            });
        },
        weekdays: weekdays.into(),
        dates: dates.into(),
    }
}

/// Options for [`use_calendar_with_options`].
// #[doc(cfg(feature = "use_calendar"))]
#[derive(DefaultBuilder)]
pub struct UseCalendarOptions {
    /// Date being used to initialize the calendar month to be displayed.
    /// Optional [`chrono::NaiveDate`](https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html). Defaults to [`chrono::Local::now()`](https://docs.rs/chrono/latest/chrono/struct.Local.html#method.now).
    #[builder(into)]
    pub initial_date: MaybeRwSignal<Option<NaiveDate>>,
    /// First day of the week as a number from 0 to 6. Defaults to 0 (Monday).
    #[builder(into)]
    pub first_day_of_the_week: MaybeSignal<usize>,
}

impl Default for UseCalendarOptions {
    fn default() -> Self {
        Self {
            initial_date: Some(Local::now().date_naive()).into(),
            first_day_of_the_week: 0.into(),
        }
    }
}

/// Return type of [`use_calendar`].
// #[doc(cfg(feature = "use_calendar"))]
pub struct UseCalendarReturn<PreviousMonthFn, TodayFn, NextMonthFn>
where
    PreviousMonthFn: Fn() + Clone,
    TodayFn: Fn() + Clone,
    NextMonthFn: Fn() + Clone,
{
    /// A function to go to the previous month.
    pub previous_month: PreviousMonthFn,
    /// A function to go to the current month.
    pub today: TodayFn,
    /// A function to go to the next month.
    pub next_month: NextMonthFn,
    /// The first day of the week as a number from 0 to 6.
    pub weekdays: Signal<Vec<usize>>,
    /// A `Vec` of [`CalendarDate`]s representing the dates in the current month.
    pub dates: Signal<Vec<CalendarDate>>,
}

/// Utility enum to represent a calendar date. Implements [`Deref`] to [`chrono::NaiveDate`](https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html).
#[derive(Clone, Copy, PartialEq)]
pub enum CalendarDate {
    Previous(NaiveDate),
    Current(NaiveDate),
    Next(NaiveDate),
}

impl CalendarDate {
    pub fn is_other_month(&self) -> bool {
        match self {
            CalendarDate::Previous(_) | CalendarDate::Next(_) => true,
            CalendarDate::Current(_) => false,
        }
    }
    pub fn is_today(&self) -> bool {
        let date = self.deref();
        let now_date = Local::now().date_naive();
        &now_date == date
    }

    pub fn is_selected(&self, selected_date: &NaiveDate) -> bool {
        self.deref() == selected_date
    }

    pub fn is_before(&self, date: &NaiveDate) -> bool {
        self.deref() < date
    }

    pub fn is_between(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> bool {
        let date = self.deref();
        date >= start_date && date <= end_date
    }

    pub fn is_between_current_month(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> bool {
        match self {
            CalendarDate::Current(date) => date >= start_date && date <= end_date,
            CalendarDate::Next(date) => date > start_date && date <= end_date,
            CalendarDate::Previous(date) => date >= start_date && date < end_date,
        }
    }

    pub fn is_after(&self, date: &NaiveDate) -> bool {
        self.deref() > date
    }

    pub fn is_first_day_of_month(&self) -> bool {
        let date = self.deref();
        if let Some(prev_date) = date.pred_opt() {
            date.month() != prev_date.month()
        } else {
            true
        }
    }

    pub fn is_last_day_of_month(&self) -> bool {
        let date = self.deref();
        if let Some(next_date) = date.succ_opt() {
            date.month() != next_date.month()
        } else {
            true
        }
    }
}

impl Deref for CalendarDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        match self {
            CalendarDate::Previous(date)
            | CalendarDate::Current(date)
            | CalendarDate::Next(date) => date,
        }
    }
}
