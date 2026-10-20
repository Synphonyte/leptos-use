#[cfg(not(feature = "ssr"))]
use crate::sendwrap_fn;
use cfg_if::cfg_if;
use chrono::{DateTime, TimeZone};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::fmt::Display;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsValue;

cfg_if! { if #[cfg(feature = "ssr")] {
    /// Return type of [`use_intl_datetime_format`].
    pub struct UseIntlDateTimeFormatReturn;
} else {
    /// Return type of [`use_intl_datetime_format`](crate::use_intl_datetime_format).
    pub struct UseIntlDateTimeFormatReturn {
        /// The instance of [`Intl.DateTimeFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat).
        pub js_intl_datetime_format: js_sys::Intl::DateTimeFormat,
    }
}}

#[cfg(not(feature = "ssr"))]
fn datetime_to_js_date<Tz>(date: &DateTime<Tz>) -> js_sys::Date
where
    Tz: TimeZone,
{
    js_sys::Date::new(&JsValue::from_f64(date.timestamp_millis() as f64))
}

impl UseIntlDateTimeFormatReturn {
    /// Formats a date according to the [locale and formatting options](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#parameters) of this `Intl.DateTimeFormat` object.
    /// See [`use_intl_datetime_format`](crate::use_intl_datetime_format) for more information.
    /// In the browser this uses `SendWrapper` internally so the returned signal can only be used on
    /// the same thread where this method was called.
    pub fn format<Tz>(&self, date: impl Into<Signal<DateTime<Tz>>>) -> Signal<String>
    where
        Tz: TimeZone,
        DateTime<Tz>: Clone + Display + Send + Sync + 'static,
    {
        let date = date.into();

        cfg_if! { if #[cfg(feature = "ssr")] {
            Signal::derive(move || {
                format!("{}", date.get())
            })
        } else {
            let datetime_format = self.js_intl_datetime_format.clone();

            Signal::derive(sendwrap_fn!(move || {
                if let Ok(result) = datetime_format
                    .format()
                    .call1(&datetime_format, &datetime_to_js_date(&date.get()).into())
                {
                    result.as_string().unwrap_or_default()
                } else {
                    "".to_string()
                }
            }))
        }}
    }

    /// Formats a range of dates according to the locale and formatting options of this
    /// `Intl.DateTimeFormat` object.
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use_intl_datetime_format::{use_intl_datetime_format, UseIntlDateTimeFormatOptions, MonthFormat, DayFormat, YearFormat};
    /// # use chrono::{TimeZone, Utc};
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let start = Utc.with_ymd_and_hms(2007, 1, 10, 10, 0, 0).unwrap();
    /// let end = Utc.with_ymd_and_hms(2008, 1, 10, 11, 0, 0).unwrap();
    ///
    /// let date_time_format = use_intl_datetime_format(
    ///     UseIntlDateTimeFormatOptions::default()
    ///         .locale("en-US")
    ///         .year(YearFormat::Numeric)
    ///         .month(MonthFormat::Short)
    ///         .day(DayFormat::Numeric),
    /// );
    ///
    /// let formatted = date_time_format.format_range(start, end); // "Jan 10, 2007 – Jan 10, 2008"
    /// #
    /// # view! { }
    /// # }
    /// ```
    pub fn format_range<TzStart, TzEnd>(
        &self,
        start: impl Into<Signal<DateTime<TzStart>>>,
        end: impl Into<Signal<DateTime<TzEnd>>>,
    ) -> Signal<String>
    where
        TzStart: TimeZone,
        TzEnd: TimeZone,
        DateTime<TzStart>: Clone + Display + Send + Sync + 'static,
        DateTime<TzEnd>: Clone + Display + Send + Sync + 'static,
    {
        let start = start.into();
        let end = end.into();

        cfg_if! { if #[cfg(feature = "ssr")] {
            Signal::derive(move || {
                format!("{} - {}", start.get(), end.get())
            })
        } else {
            let datetime_format = self.js_intl_datetime_format.clone();

            Signal::derive(sendwrap_fn!(move || {
                datetime_format
                    .format_range(
                        &datetime_to_js_date(&start.get()),
                        &datetime_to_js_date(&end.get()),
                    )
                    .map(|s| s.as_string().unwrap_or_default())
                    .unwrap_or_default()
            }))
        }}
    }
}
