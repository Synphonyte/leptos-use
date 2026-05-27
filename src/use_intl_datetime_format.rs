#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::{js, sendwrap_fn, utils::js_value_from_to_string};
use cfg_if::cfg_if;
use chrono::{DateTime, TimeZone};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::fmt::Display;
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
/// # use leptos_use::{use_intl_datetime_format, UseIntlDateTimeFormatOptions};
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
/// # use leptos_use::{use_intl_datetime_format, UseIntlDateTimeFormatOptions};
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
/// # use leptos_use::{
/// #     use_intl_datetime_format, UseIntlDateTimeFormatOptions, WeekdayFormat, EraFormat,
/// #     YearFormat, MonthFormat, DayFormat, TimeNumericFormat, TimeZoneNameFormat,
/// # };
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
/// # use leptos_use::{use_intl_datetime_format, UseIntlDateTimeFormatOptions, DateTimeStyle};
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

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FormatMatcher {
    Basic,
    #[default]
    BestFit,
}

impl Display for FormatMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::BestFit => write!(f, "best fit"),
        }
    }
}

js_value_from_to_string!(FormatMatcher);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocaleMatcher {
    #[default]
    BestFit,
    Lookup,
}

impl Display for LocaleMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BestFit => write!(f, "best fit"),
            Self::Lookup => write!(f, "lookup"),
        }
    }
}

js_value_from_to_string!(LocaleMatcher);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum HourCycle {
    /// hour system using 0–11; corresponds to midnight starting at 0:00 AM.
    H11,
    /// hour system using 1–12; corresponds to midnight starting at 12:00 AM.
    H12,
    /// hour system using 0–23; corresponds to midnight starting at 0:00.
    H23,
    /// hour system using 1–24; corresponds to midnight starting at 24:00.
    H24,
}

impl Display for HourCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::H11 => write!(f, "h11"),
            Self::H12 => write!(f, "h12"),
            Self::H23 => write!(f, "h23"),
            Self::H24 => write!(f, "h24"),
        }
    }
}

js_value_from_to_string!(HourCycle);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DateTimeStyle {
    Full,
    Long,
    Medium,
    Short,
}

impl Display for DateTimeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::Long => write!(f, "long"),
            Self::Medium => write!(f, "medium"),
            Self::Short => write!(f, "short"),
        }
    }
}

js_value_from_to_string!(DateTimeStyle);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum WeekdayFormat {
    /// e.g., `Thursday`
    Long,
    /// e.g., `Thu`
    Short,
    /// e.g., `T`. Note that two weekdays may have the same narrow style for some locales (e.g. `Tuesday`'s narrow style is also `T`).
    Narrow,
}

impl Display for WeekdayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Narrow => write!(f, "narrow"),
        }
    }
}

js_value_from_to_string!(WeekdayFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EraFormat {
    /// e.g., `Anno Domini`
    Long,
    /// e.g., `AD`
    Short,
    /// e.g., `A`
    Narrow,
}

impl Display for EraFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Narrow => write!(f, "narrow"),
        }
    }
}

js_value_from_to_string!(EraFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum YearFormat {
    /// e.g., `2012`
    Numeric,
    /// e.g., `12`
    TwoDigit,
}

impl Display for YearFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric => write!(f, "numeric"),
            Self::TwoDigit => write!(f, "2-digit"),
        }
    }
}

js_value_from_to_string!(YearFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MonthFormat {
    /// e.g., `2`
    Numeric,
    /// e.g., `02`
    TwoDigit,
    /// e.g., `March`
    Long,
    /// e.g., `Mar`
    Short,
    /// e.g., `M`
    Narrow,
}

impl Display for MonthFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric => write!(f, "numeric"),
            Self::TwoDigit => write!(f, "2-digit"),
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Narrow => write!(f, "narrow"),
        }
    }
}

js_value_from_to_string!(MonthFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DayFormat {
    /// e.g., `1`
    Numeric,
    /// e.g., `01`
    TwoDigit,
}

impl Display for DayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric => write!(f, "numeric"),
            Self::TwoDigit => write!(f, "2-digit"),
        }
    }
}

js_value_from_to_string!(DayFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TimeNumericFormat {
    /// e.g., `1`
    Numeric,
    /// e.g., `01`
    TwoDigit,
}

impl Display for TimeNumericFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric => write!(f, "numeric"),
            Self::TwoDigit => write!(f, "2-digit"),
        }
    }
}

js_value_from_to_string!(TimeNumericFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DayPeriodFormat {
    Long,
    Short,
    Narrow,
}

impl Display for DayPeriodFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Narrow => write!(f, "narrow"),
        }
    }
}

js_value_from_to_string!(DayPeriodFormat);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TimeZoneNameFormat {
    /// Long localized form (e.g., `Pacific Standard Time`, `Nordamerikanische Westküsten-Normalzeit`)
    Long,
    /// Short localized form (e.g.: `PST`, `GMT-8`)
    Short,
    /// Short localized GMT format (e.g., `GMT-8`)
    ShortOffset,
    /// Long localized GMT format (e.g., `GMT-0800`)
    LongOffset,
    /// Short generic non-location format (e.g.: `PT`, `Los Angeles Zeit`).
    ShortGeneric,
    /// Long generic non-location format (e.g.: `Pacific Time`, `Nordamerikanische Westküstenzeit`)
    LongGeneric,
}

impl Display for TimeZoneNameFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::ShortOffset => write!(f, "shortOffset"),
            Self::LongOffset => write!(f, "longOffset"),
            Self::ShortGeneric => write!(f, "shortGeneric"),
            Self::LongGeneric => write!(f, "longGeneric"),
        }
    }
}

js_value_from_to_string!(TimeZoneNameFormat);

#[derive(DefaultBuilder, Default)]
pub struct UseIntlDateTimeFormatOptions {
    /// A vec of strings, each with a BCP 47 language tag. Please refer to the
    /// [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#parameters)
    /// for more info.
    locales: Vec<String>,

    /// The locale matching algorithm to use. Possible values are `Lookup` and `BestFit`; the default is `BestFit`.
    /// For information about this option, see the [Intl page](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl#locale_identification_and_negotiation).
    locale_matcher: LocaleMatcher,

    /// The calendar to use, such as `"chinese"`, `"gregory"`, `"persian"` etc. For a list of the
    /// supported calendar types, see [the MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getCalendars#supported_calendar_types).
    #[builder(into)]
    calendar: Option<String>,

    /// The way day periods should be expressed. This option only has an effect if a 12-hour clock
    /// (`hour_cycle` `H11` or `H12`, or `hour12` is `true`) is used. Note that the day period may
    /// be displayed even if the `hour` is not set to be displayed.
    #[builder(into)]
    day_period: Option<DayPeriodFormat>,

    /// The numbering system to use. Possible values include: `"arab"`, `"arabext"`, `"bali"`, `"beng"`, `"deva"`, `"fullwide"`, `"gujr"`, `"guru"`, `"hanidec"`, `"khmr"`, `"knda"`, `"laoo"`, `"latn"`, `"limb"`, `"mlym"`, `"mong"`, `"mymr"`, `"orya"`, `"tamldec"`, `"telu"`, `"thai"`, `"tibt"`.
    #[builder(into)]
    numbering_system: Option<String>,

    /// Whether to use 12-hour time (as opposed to 24-hour time). The default is locale dependent.
    /// This option overrides the `hc` language tag and/or the `hour_cycle` option in case both are present.
    #[builder(into)]
    hour12: Option<bool>,

    /// The hour cycle to use. This option overrides the `hc` language tag, if both are present, and
    /// the `hour12` option takes precedence in case both options have been specified.
    ///
    /// - `H11`: hour system using 0–11; corresponds to midnight starting at 0:00 AM.
    /// - `H12`: hour system using 1–12; corresponds to midnight starting at 12:00 AM.
    /// - `H23`: hour system using 0–23; corresponds to midnight starting at 0:00.
    /// - `H24`: hour system using 1–24; corresponds to midnight starting at 24:00.
    #[builder(into)]
    hour_cycle: Option<HourCycle>,

    /// The time zone to use. The only value implementations must recognize is `"UTC"`; the default
    /// is the runtime's default time zone. Implementations may also recognize the time zone names
    /// of the [IANA time zone database](https://www.iana.org/time-zones), such as `"Asia/Shanghai"`,
    /// `"Asia/Kolkata"`, `"America/New_York"`.
    #[builder(into)]
    time_zone: Option<String>,

    /// The format matching algorithm to use. Possible values are `Basic` and `BestFit`; the default
    /// is `BestFit`. See the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#formatmatcher)
    /// for details. Only has an effect when using the individual date-time component options
    /// (and not [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`]).
    format_matcher: FormatMatcher,

    /// The representation of the weekday.
    ///
    /// - `Long`: e.g., `Thursday`
    /// - `Short`: e.g., `Thu`
    /// - `Narrow`: e.g., `T`. Two weekdays may have the same narrow style for some locales (e.g. `Tuesday`'s narrow style is also `T`).
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    weekday: Option<WeekdayFormat>,

    /// The representation of the era.
    ///
    /// - `Long`: e.g., `Anno Domini`
    /// - `Short`: e.g., `AD`
    /// - `Narrow`: e.g., `A`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    era: Option<EraFormat>,

    /// The representation of the year.
    ///
    /// - `Numeric`: e.g., `2012`
    /// - `TwoDigit`: e.g., `12`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    year: Option<YearFormat>,

    /// The representation of the month.
    ///
    /// - `Numeric`: e.g., `3`
    /// - `TwoDigit`: e.g., `03`
    /// - `Long`: e.g., `March`
    /// - `Short`: e.g., `Mar`
    /// - `Narrow`: e.g., `M`. Two months may have the same narrow style for some locales (e.g. `May`'s narrow style is also `M`).
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    month: Option<MonthFormat>,

    /// The representation of the day.
    ///
    /// - `Numeric`: e.g., `1`
    /// - `TwoDigit`: e.g., `01`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    day: Option<DayFormat>,

    /// The representation of the hour.
    ///
    /// - `Numeric`: e.g., `1`
    /// - `TwoDigit`: e.g., `01`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    hour: Option<TimeNumericFormat>,

    /// The representation of the minute.
    ///
    /// - `Numeric`: e.g., `1`
    /// - `TwoDigit`: e.g., `01`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    minute: Option<TimeNumericFormat>,

    /// The representation of the second.
    ///
    /// - `Numeric`: e.g., `1`
    /// - `TwoDigit`: e.g., `01`
    ///
    /// Cannot be combined with [`UseIntlDateTimeFormatOptions::date_style`] / [`UseIntlDateTimeFormatOptions::time_style`].
    #[builder(into)]
    second: Option<TimeNumericFormat>,

    /// The number of digits used to represent fractions of a second (any additional digits are
    /// truncated). Possible values are from `1` to `3`.
    #[builder(into)]
    fractional_second_digits: Option<u8>,

    /// The localized representation of the time zone name.
    ///
    /// - `Long`: long localized form (e.g., `Pacific Standard Time`).
    /// - `Short`: short localized form (e.g.: `PST`).
    /// - `ShortOffset`: short localized GMT format (e.g., `GMT-8`).
    /// - `LongOffset`: long localized GMT format (e.g., `GMT-0800`).
    /// - `ShortGeneric`: short generic non-location format (e.g.: `PT`).
    /// - `LongGeneric`: long generic non-location format (e.g.: `Pacific Time`).
    #[builder(into)]
    time_zone_name: Option<TimeZoneNameFormat>,

    /// The date formatting length. Cannot be combined with the individual date-time component
    /// options (`weekday`, `era`, `year`, `month`, `day`, `hour`, `minute`, `second`,
    /// `fractional_second_digits`, `time_zone_name`, `day_period`).
    ///
    /// Possible values are `Full`, `Long`, `Medium` and `Short`.
    #[builder(into)]
    date_style: Option<DateTimeStyle>,

    /// The time formatting length. Cannot be combined with the individual date-time component
    /// options (`weekday`, `era`, `year`, `month`, `day`, `hour`, `minute`, `second`,
    /// `fractional_second_digits`, `time_zone_name`, `day_period`).
    ///
    /// Possible values are `Full`, `Long`, `Medium` and `Short`.
    #[builder(into)]
    time_style: Option<DateTimeStyle>,
}

impl UseIntlDateTimeFormatOptions {
    /// A string with a BCP 47 language tag. Please refer to the
    /// [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#parameters)
    /// for more info.
    pub fn locale(self, locale: &str) -> Self {
        Self {
            locales: vec![locale.to_string()],
            ..self
        }
    }
}

impl From<UseIntlDateTimeFormatOptions> for js_sys::Object {
    fn from(options: UseIntlDateTimeFormatOptions) -> Self {
        let object = Self::new();

        _ = js!(object["localeMatcher"] = options.locale_matcher);
        _ = js!(object["formatMatcher"] = options.format_matcher);

        if let Some(calendar) = options.calendar {
            _ = js!(object["calendar"] = calendar);
        }
        if let Some(day_period) = options.day_period {
            _ = js!(object["dayPeriod"] = day_period);
        }
        if let Some(numbering_system) = options.numbering_system {
            _ = js!(object["numberingSystem"] = numbering_system);
        }
        if let Some(hour12) = options.hour12 {
            _ = js!(object["hour12"] = hour12);
        }
        if let Some(hour_cycle) = options.hour_cycle {
            _ = js!(object["hourCycle"] = hour_cycle);
        }
        if let Some(time_zone) = options.time_zone {
            _ = js!(object["timeZone"] = time_zone);
        }
        if let Some(weekday) = options.weekday {
            _ = js!(object["weekday"] = weekday);
        }
        if let Some(era) = options.era {
            _ = js!(object["era"] = era);
        }
        if let Some(year) = options.year {
            _ = js!(object["year"] = year);
        }
        if let Some(month) = options.month {
            _ = js!(object["month"] = month);
        }
        if let Some(day) = options.day {
            _ = js!(object["day"] = day);
        }
        if let Some(hour) = options.hour {
            _ = js!(object["hour"] = hour);
        }
        if let Some(minute) = options.minute {
            _ = js!(object["minute"] = minute);
        }
        if let Some(second) = options.second {
            _ = js!(object["second"] = second);
        }
        if let Some(fractional_second_digits) = options.fractional_second_digits {
            _ = js!(object["fractionalSecondDigits"] = fractional_second_digits);
        }
        if let Some(time_zone_name) = options.time_zone_name {
            _ = js!(object["timeZoneName"] = time_zone_name);
        }
        if let Some(date_style) = options.date_style {
            _ = js!(object["dateStyle"] = date_style);
        }
        if let Some(time_style) = options.time_style {
            _ = js!(object["timeStyle"] = time_style);
        }

        object
    }
}

cfg_if! { if #[cfg(feature = "ssr")] {
    /// Return type of [`use_intl_datetime_format`].
    pub struct UseIntlDateTimeFormatReturn;
} else {
    /// Return type of [`use_intl_datetime_format`].
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
    /// See [`use_intl_datetime_format`] for more information.
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
    /// # use leptos_use::{use_intl_datetime_format, UseIntlDateTimeFormatOptions, MonthFormat, DayFormat, YearFormat};
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
