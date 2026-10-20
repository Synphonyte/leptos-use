#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::{UseIntlNumberFormatOptions, UseIntlNumberFormatReturn};
use cfg_if::cfg_if;
use wasm_bindgen::JsValue;

/// Reactive [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_intl_number_format)
///
/// ## Usage
///
/// In basic use without specifying a locale, a formatted string in the default locale and with default options is returned.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_number_format::{use_intl_number_format, UseIntlNumberFormatOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (number, set_number) = signal(3500);
///
/// let number_format = use_intl_number_format(UseIntlNumberFormatOptions::default());
///
/// let formatted = number_format.format::<u16>(number); // "3,500" if in US English locale
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Using locales
///
/// This example shows some of the variations in localized number formats. In order to get the format
/// of the language used in the user interface of your application, make sure to specify that language
/// (and possibly some fallback languages) using the `locales` argument:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_intl_number_format::{use_intl_number_format, UseIntlNumberFormatOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let number = 123456.789_f32;
///
/// // German uses comma as decimal separator and period for thousands
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("de-DE"),
/// );
/// let formatted = number_format.format::<f32>(number); // 123.456,789
///
/// // Arabic in most Arabic speaking countries uses real Arabic digits
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("ar-EG"),
/// );
/// let formatted = number_format.format::<f32>(number); // ١٢٣٤٥٦٫٧٨٩
///
/// // India uses thousands/lakh/crore separators
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("en-IN"),
/// );
/// let formatted = number_format.format::<f32>(number); // 1,23,456.789
///
/// // the nu extension key requests a numbering system, e.g. Chinese decimal
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("zh-Hans-CN-u-nu-hanidec"),
/// );
/// let formatted = number_format.format::<f32>(number); // 一二三,四五六.七八九
///
/// // when requesting a language that may not be supported, such as
/// // Balinese, include a fallback language, in this case Indonesian
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locales(vec!["ban".to_string(), "id".to_string()]),
/// );
/// let formatted = number_format.format::<f32>(number); // 123.456,789
///
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
/// # use leptos_use_intl_number_format::{NumberStyle, UnitDisplay, use_intl_number_format, UseIntlNumberFormatOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let number = 123456.789_f64;
///
/// // request a currency format
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("de-DE")
///         .style(NumberStyle::Currency)
///         .currency("EUR"),
/// );
/// let formatted = number_format.format::<f64>(number); // 123.456,79 €
///
/// // the Japanese yen doesn't use a minor unit
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("ja-JP")
///         .style(NumberStyle::Currency)
///         .currency("JPY"),
/// );
/// let formatted = number_format.format::<f64>(number); // ￥123,457
///
/// // limit to three significant digits
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("en-IN")
///         .maximum_significant_digits(3),
/// );
/// let formatted = number_format.format::<f64>(number); // 1,23,000
///
/// // Formatting with units
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("pt-PT")
///         .style(NumberStyle::Unit)
///         .unit("kilometer-per-hour"),
/// );
/// let formatted = number_format.format::<i32>(50); // 50 km/h
///
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("en-GB")
///         .style(NumberStyle::Unit)
///         .unit("liter")
///         .unit_display(UnitDisplay::Long),
/// );
/// let formatted = number_format.format::<i32>(16); // 16 litres
/// #
/// # view! { }
/// # }
/// ```
///
/// For an exhaustive list of options see [`UseIntlNumberFormatOptions`](https://docs.rs/leptos_use/latest/leptos_use/struct.UseIntlNumberFormatOptions.html).
///
/// ## Formatting ranges
///
/// Apart from the `format` method, the `format_range` method can be used to format a range of numbers.
/// Please see [`UseIntlNumberFormatReturn::format_range`](https://docs.rs/leptos_use/latest/leptos_use/struct.UseIntlNumberFormatReturn.html#method.format_range)
/// for details.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// Since `Intl.NumberFormat` is a JavaScript API it is not available on the server. That's why
/// it falls back to a simple call to `format!()` on the server.
pub fn use_intl_number_format(options: UseIntlNumberFormatOptions) -> UseIntlNumberFormatReturn {
    cfg_if! { if #[cfg(feature = "ssr")] {
        UseIntlNumberFormatReturn
    } else {
        let number_format = js_sys::Intl::NumberFormat::new(
            &js_sys::Array::from_iter(options.locales.iter().map(JsValue::from)),
            &js_sys::Object::from(options),
        );

        UseIntlNumberFormatReturn {
            js_intl_number_format: number_format,
        }
    }}
}
