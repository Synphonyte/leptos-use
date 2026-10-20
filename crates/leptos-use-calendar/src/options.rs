use crate::{CalendarDateType, MaybeRwSignal};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;

/// Options for [`use_calendar_with_options`](crate::use_calendar_with_options).
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
