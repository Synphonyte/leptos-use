use crate::{CalendarDateType, use_calendar::is_same_month};
use leptos::prelude::*;
use std::ops::Deref;

/// Return type of [`use_calendar`](crate::use_calendar).
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
