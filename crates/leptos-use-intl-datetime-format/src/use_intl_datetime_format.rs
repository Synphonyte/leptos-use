#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::{UseIntlDateTimeFormatOptions, UseIntlDateTimeFormatReturn};
use cfg_if::cfg_if;
use wasm_bindgen::JsValue;

/// Reactive [`Intl.DateTimeFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_intl_datetime_format)
///
/// ## Usage
///
/// Dates are passed in as [`chrono::DateTime`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)
/// (just like in [`fn@crate::use_calendar`]).
///
/// In basic use without specifying a locale, a formatted string in the default locale and with default
/// options is returned.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_datetime_format::{use_intl_datetime_format, UseIntlDateTimeFormatOptions};
/// # use chrono::{TimeZone, Utc};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (date, set_date) = signal(Utc.with_ymd_and_hms(2020, 12, 20, 3, 23, 16).unwrap());
///
/// let date_time_format = use_intl_datetime_format(UseIntlDateTimeFormatOptions::default());
///
/// let formatted = date_time_format.format(date); // "12/20/2020" if in US English locale
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Using locales
///
/// In order to get the format of the language used in the user interface of your application, make
/// sure to specify that language (and possibly some fallback languages) using the `locales` argument:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_datetime_format::{use_intl_datetime_format, UseIntlDateTimeFormatOptions};
/// # use chrono::{TimeZone, Utc};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let date = Utc.with_ymd_and_hms(2012, 12, 20, 3, 0, 0).unwrap();
///
/// // British English uses day-month-year order and 24-hour time without AM/PM
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default().locale("en-GB"),
/// );
/// let formatted = date_time_format.format(date); // 20/12/2012
///
/// // Korean uses year-month-day order
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default().locale("ko-KR"),
/// );
/// let formatted = date_time_format.format(date); // 2012. 12. 20.
///
/// // Arabic in most Arabic speaking countries uses real Arabic digits
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default().locale("ar-EG"),
/// );
/// let formatted = date_time_format.format(date); // ٢٠‏/١٢‏/٢٠١٢
///
/// // when requesting a language that may not be supported, such as
/// // Balinese, include a fallback language, in this case Indonesian
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default()
///         .locales(vec!["ban".to_string(), "id".to_string()]),
/// );
/// let formatted = date_time_format.format(date); // 20/12/2012
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Using options
///
/// The results can be customized in multiple ways.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_datetime_format::{use_intl_datetime_format, UseIntlDateTimeFormatOptions, WeekdayFormat, EraFormat, YearFormat, MonthFormat, DayFormat, TimeNumericFormat, TimeZoneNameFormat};
/// # use chrono::{TimeZone, Utc};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let date = Utc.with_ymd_and_hms(2012, 12, 20, 3, 0, 0).unwrap();
///
/// // request a weekday along with a long date
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default()
///         .locale("en-US")
///         .weekday(WeekdayFormat::Long)
///         .year(YearFormat::Numeric)
///         .month(MonthFormat::Long)
///         .day(DayFormat::Numeric),
/// );
/// let formatted = date_time_format.format(date); // "Thursday, December 20, 2012"
///
/// // an application may want to use UTC and make that visible
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default()
///         .locale("en-US")
///         .hour(TimeNumericFormat::Numeric)
///         .minute(TimeNumericFormat::TwoDigit)
///         .second(TimeNumericFormat::TwoDigit)
///         .time_zone("UTC")
///         .time_zone_name(TimeZoneNameFormat::Short),
/// );
/// let formatted = date_time_format.format(date); // "3:00:00 AM UTC"
/// #
/// # view! { }
/// # }
/// ```
///
/// Instead of specifying the individual date and time components you can use the
/// [`UseIntlDateTimeFormatOptions::date_style`] and [`UseIntlDateTimeFormatOptions::time_style`]
/// shortcuts.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_datetime_format::{use_intl_datetime_format, UseIntlDateTimeFormatOptions, DateTimeStyle};
/// # use chrono::{TimeZone, Utc};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let date = Utc.with_ymd_and_hms(2012, 12, 20, 3, 0, 0).unwrap();
///
/// let date_time_format = use_intl_datetime_format(
///     UseIntlDateTimeFormatOptions::default()
///         .locale("en-GB")
///         .date_style(DateTimeStyle::Full)
///         .time_style(DateTimeStyle::Long)
///         .time_zone("UTC"),
/// );
/// let formatted = date_time_format.format(date); // "Thursday 20 December 2012 at 03:00:00 UTC"
/// #
/// # view! { }
/// # }
/// ```
///
/// > Note that `date_style` and `time_style` cannot be mixed with the individual date-time component
/// > options (like `weekday`, `year`, `hour`, ...). Doing so makes `Intl.DateTimeFormat` throw a
/// > `RangeError` which results in an empty string being returned.
///
/// For an exhaustive list of options see [`UseIntlDateTimeFormatOptions`](https://docs.rs/leptos_use/latest/leptos_use/struct.UseIntlDateTimeFormatOptions.html).
///
/// ## Formatting ranges
///
/// Apart from the `format` method, the `format_range` method can be used to format a range of dates.
/// Please see [`UseIntlDateTimeFormatReturn::format_range`](https://docs.rs/leptos_use/latest/leptos_use/struct.UseIntlDateTimeFormatReturn.html#method.format_range)
/// for details.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// Since `Intl.DateTimeFormat` is a JavaScript API it is not available on the server. That's why
/// it falls back to a simple call to `format!()` on the server, which is **not** locale-aware.
pub fn use_intl_datetime_format(
    options: UseIntlDateTimeFormatOptions,
) -> UseIntlDateTimeFormatReturn {
    cfg_if! { if #[cfg(feature = "ssr")] {
        UseIntlDateTimeFormatReturn
    } else {
        let datetime_format = js_sys::Intl::DateTimeFormat::new(
            &js_sys::Array::from_iter(options.locales.iter().map(JsValue::from)),
            &js_sys::Object::from(options),
        );

        UseIntlDateTimeFormatReturn {
            js_intl_datetime_format: datetime_format,
        }
    }}
}
