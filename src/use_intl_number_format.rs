#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::js;
use crate::utils::js_value_from_to_string;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use std::fmt::Display;
use wasm_bindgen::{JsCast, JsValue};

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
/// # use leptos_use::{use_intl_number_format, UseIntlNumberFormatOptions};
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
/// # use leptos_use::{use_intl_number_format, UseIntlNumberFormatOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let number = 123456.789_f32;
///
/// // German uses comma as decimal separator and period for thousands
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("de-DE"),
/// );
/// let formatted = number_format.format(number); // 123.456,789
///
/// // Arabic in most Arabic speaking countries uses real Arabic digits
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("ar-EG"),
/// );
/// let formatted = number_format.format(number); // ١٢٣٤٥٦٫٧٨٩
///
/// // India uses thousands/lakh/crore separators
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("en-IN"),
/// );
/// let formatted = number_format.format(number); // 1,23,456.789
///
/// // the nu extension key requests a numbering system, e.g. Chinese decimal
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locale("zh-Hans-CN-u-nu-hanidec"),
/// );
/// let formatted = number_format.format(number); // 一二三,四五六.七八九
///
/// // when requesting a language that may not be supported, such as
/// // Balinese, include a fallback language, in this case Indonesian
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default().locales(vec!["ban".to_string(), "id".to_string()]),
/// );
/// let formatted = number_format.format(number); // 123.456,789
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
/// # use leptos_use::{NumberStyle, UnitDisplay, use_intl_number_format, UseIntlNumberFormatOptions};
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
/// let formatted = number_format.format(number); // 123.456,79 €
///
/// // the Japanese yen doesn't use a minor unit
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("ja-JP")
///         .style(NumberStyle::Currency)
///         .currency("JPY"),
/// );
/// let formatted = number_format.format(number); // ￥123,457
///
/// // limit to three significant digits
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("en-IN")
///         .maximum_significant_digits(3),
/// );
/// let formatted = number_format.format(number); // 1,23,000
///
/// // Formatting with units
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("pt-PT")
///         .style(NumberStyle::Unit)
///         .unit("kilometer-per-hour"),
/// );
/// let formatted = number_format.format(50); // 50 km/h
///
/// let number_format = use_intl_number_format(
///     UseIntlNumberFormatOptions::default()
///         .locale("en-GB")
///         .style(NumberStyle::Unit)
///         .unit("liter")
///         .unit_display(UnitDisplay::Long),
/// );
/// let formatted = number_format.format(16); // 16 litres
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

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CompactDisplay {
    #[default]
    Short,
    Long,
}

impl Display for CompactDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "short"),
            Self::Long => write!(f, "long"),
        }
    }
}

js_value_from_to_string!(CompactDisplay);

/// How to display the currency in currency formatting.
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CurrencyDisplay {
    /// use a localized currency symbol such as €.
    #[default]
    Symbol,
    /// use a narrow format symbol ("$100" rather than "US$100").
    NarrowSymbol,
    /// use the ISO currency code.
    Code,
    /// use a localized currency name such as "dollar".
    Name,
}

impl Display for CurrencyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol => write!(f, "symbol"),
            Self::NarrowSymbol => write!(f, "narrowSymbol"),
            Self::Code => write!(f, "code"),
            Self::Name => write!(f, "name"),
        }
    }
}

js_value_from_to_string!(CurrencyDisplay);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CurrencySign {
    #[default]
    Standard,
    Accounting,
}

impl Display for CurrencySign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Accounting => write!(f, "accounting"),
        }
    }
}

js_value_from_to_string!(CurrencySign);

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

/// The formatting that should be displayed for the number.
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Notation {
    /// plain number formatting.
    #[default]
    Standard,
    /// order-of-magnitude for formatted number.
    Scientific,
    /// exponent of ten when divisible by three.
    Engineering,
    /// string representing exponent
    Compact,
}

impl Display for Notation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Scientific => write!(f, "scientific"),
            Self::Engineering => write!(f, "engineering"),
            Self::Compact => write!(f, "compact"),
        }
    }
}

js_value_from_to_string!(Notation);

/// When to display the sign for the number.
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SignDisplay {
    /// sign display for negative numbers only, including negative zero.
    #[default]
    Auto,
    /// always display the sign.
    Always,
    /// sign display for positive and negative numbers, but not zero.
    ExceptZero,
    /// sign display for negative numbers only, excluding negative zero. Experimental.
    Negative,
    /// never display sign.
    Never,
}

impl Display for SignDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::ExceptZero => write!(f, "exceptZero"),
            Self::Negative => write!(f, "negative"),
            Self::Never => write!(f, "never"),
        }
    }
}

js_value_from_to_string!(SignDisplay);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NumberStyle {
    #[default]
    Decimal,
    Currency,
    Percent,
    Unit,
}

impl Display for NumberStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decimal => write!(f, "decimal"),
            Self::Currency => write!(f, "currency"),
            Self::Percent => write!(f, "percent"),
            Self::Unit => write!(f, "unit"),
        }
    }
}

js_value_from_to_string!(NumberStyle);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnitDisplay {
    /// e.g., `16 litres`
    Long,
    /// e.g., `16 l`
    #[default]
    Short,
    /// e.g., `16l`
    Narrow,
}

impl Display for UnitDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Narrow => write!(f, "narrow"),
        }
    }
}

js_value_from_to_string!(UnitDisplay);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NumberGrouping {
    /// display grouping separators even if the locale prefers otherwise.
    Always,
    /// display grouping separators based on the locale preference, which may also be dependent on the currency.
    #[default]
    Auto,
    /// do not display grouping separators.
    None,
    /// display grouping separators when there are at least 2 digits in a group.
    Min2,
}

impl Display for NumberGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Auto => write!(f, "auto"),
            Self::None => write!(f, "none"),
            Self::Min2 => write!(f, "min2"),
        }
    }
}

impl From<NumberGrouping> for JsValue {
    fn from(value: NumberGrouping) -> Self {
        match value {
            NumberGrouping::None => JsValue::from(false),
            _ => JsValue::from(&value.to_string()),
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RoundingMode {
    /// round toward +∞. Positive values round up. Negative values round "more positive".
    Ceil,
    /// round toward -∞. Positive values round down. Negative values round "more negative".
    Floor,
    /// round away from 0. The _magnitude_ of the value is always increased by rounding. Positive values round up. Negative values round "more negative".
    Expand,
    /// round toward 0. This _magnitude_ of the value is always reduced by rounding. Positive values round down. Negative values round "less negative".
    Trunc,
    /// ties toward +∞. Values above the half-increment round like `Ceil` (towards +∞), and below like `Floor` (towards -∞). On the half-increment, values round like `Ceil`.
    HalfCeil,
    /// ties toward -∞. Values above the half-increment round like `Ceil` (towards +∞), and below like `Floor` (towards -∞). On the half-increment, values round like `Floor`.
    HalfFloor,
    /// ties away from 0. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment, values round like `Expand`.
    #[default]
    HalfExpand,
    /// ties toward 0. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment, values round like `Trunc`.
    HalfTrunc,
    /// ties towards the nearest even integer. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment values round towards the nearest even digit.
    HalfEven,
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ceil => write!(f, "ceil"),
            Self::Floor => write!(f, "floor"),
            Self::Expand => write!(f, "expand"),
            Self::Trunc => write!(f, "trunc"),
            Self::HalfCeil => write!(f, "halfCeil"),
            Self::HalfFloor => write!(f, "halfFloor"),
            Self::HalfExpand => write!(f, "halfExpand"),
            Self::HalfTrunc => write!(f, "halfTrunc"),
            Self::HalfEven => write!(f, "halfEven"),
        }
    }
}

js_value_from_to_string!(RoundingMode);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RoundingPriority {
    /// the result from the significant digits property is used.
    #[default]
    Auto,
    /// the result from the property that results in more precision is used.
    MorePrecision,
    /// the result from the property that results in less precision is used.
    LessPrecision,
}

impl Display for RoundingPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::MorePrecision => write!(f, "morePrecision"),
            Self::LessPrecision => write!(f, "lessPrecision"),
        }
    }
}

js_value_from_to_string!(RoundingPriority);

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TrailingZeroDisplay {
    /// keep trailing zeros according to `minimum_fraction_digits` and `minimum_significant_digits`.
    #[default]
    Auto,
    /// remove the fraction digits _if_ they are all zero. This is the same as `Auto` if any of the fraction digits is non-zero.
    StripIfInteger,
}

impl Display for TrailingZeroDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::StripIfInteger => write!(f, "stripIfInteger"),
        }
    }
}

js_value_from_to_string!(TrailingZeroDisplay);

/// Options for [`use_intl_number_format`].
#[derive(DefaultBuilder)]
pub struct UseIntlNumberFormatOptions {
    /// A vec of strings, each with a BCP 47 language tag. Please refer to the
    /// [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#parameters)
    /// for more info.
    locales: Vec<String>,

    /// Only used when [`UseIntlNumberFormatOptions::notation`] is `Compact`. Takes either `Short` (default) or `Long`.
    compact_display: CompactDisplay,

    /// The currency to use in currency formatting.
    /// Possible values are the ISO 4217 currency codes, such as "USD" for the US dollar, "EUR" for the euro,
    /// or "CNY" for the Chinese RMB — see the [Current currency & funds code list](https://www.six-group.com/en/products-services/financial-information/data-standards.html#scrollTo=currency-codes.
    /// There is no default value; if the style is `Currency`, the currency property must be provided.
    #[builder(into)]
    currency: Option<String>,

    /// How to display the currency in currency formatting. The default is `Symbol`.
    ///
    /// - `Symbol`: use a localized currency symbol such as €.
    /// - `NarrowSymbol`: use a narrow format symbol ("$100" rather than "US$100").
    /// - `Code`: use the ISO currency code.
    /// - `Name`: use a localized currency name such as `"dollar"`.
    currency_display: CurrencyDisplay,

    /// In many locales, accounting format means to wrap the number with parentheses instead of appending a minus sign.
    /// You can enable this formatting by setting this option to `Accounting`. The default value is `Standard`.
    currency_sign: CurrencySign,

    /// The locale matching algorithm to use. Possible values are `Lookup` and `BestFit`; the default is `"BestFit"`.
    /// For information about this option, see the [Intl page](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl#locale_identification_and_negotiation).
    locale_matcher: LocaleMatcher,

    /// The formatting that should be displayed for the number. The default is `"standard"`.
    ///
    /// - `Standard`: plain number formatting.
    /// - `Scientific`: order-of-magnitude for formatted number.
    /// - `Engineering`: exponent of ten when divisible by three.
    /// - `Compact`: string representing exponent; See [`UseIntlNumberFormatOptions::compact_display`].
    notation: Notation,

    /// Numbering System. Possible values include: `"arab"`, `"arabext"`, `"bali"`, `"beng"`, `"deva"`, `"fullwide"`, `"gujr"`, `"guru"`, `"hanidec"`, `"khmr"`, `"knda"`, `"laoo"`, `"latn"`, `"limb"`, `"mlym"`, `"mong"`, `"mymr"`, `"orya"`, `"tamldec"`, `"telu"`, `"thai"`, `"tibt"`.
    #[builder(into)]
    numbering_system: Option<String>,

    /// When to display the sign for the number. The default is `Auto`.
    ///
    /// - `Auto`: sign display for negative numbers only, including negative zero.
    /// - `Always`: always display sign.
    /// - `ExceptZero`: sign display for positive and negative numbers, but not zero.
    /// - `Negative`: sign display for negative numbers only, excluding negative zero. Experimental
    /// - `Never`: never display sign.
    sign_display: SignDisplay,

    /// The formatting style to use. The default is `Decimal`.
    ///
    /// - `Decimal` for plain number formatting.
    /// - `Currency` for currency formatting.
    /// - `Percent` for percent formatting.
    /// - `Unit` for unit formatting.
    style: NumberStyle,

    /// The unit to use in `unit` formatting, Possible values are core unit identifiers,
    /// defined in [UTS #35, Part 2, Section 6](https://unicode.org/reports/tr35/tr35-general.html#Unit_Elements).
    /// A [subset](https://tc39.es/ecma402/#table-sanctioned-single-unit-identifiers) of units
    /// from the [full list](https://github.com/unicode-org/cldr/blob/main/common/validity/unit.xml)
    /// was selected for use in ECMAScript.
    /// Pairs of simple units can be concatenated with "-per-" to make a compound unit.
    /// There is no default value; if the `style` is `Unit`, the `unit` property must be provided.
    #[builder(into)]
    unit: Option<String>,

    /// The unit formatting style to use in `unit` formatting. The default is `Short`.
    ///
    /// - `Long` (e.g., `16 litres`).
    /// - `Short` (e.g., `16 l`).
    /// - `Narrow` (e.g., `16l`).
    unit_display: UnitDisplay,

    /// Experimental.
    /// Whether to use grouping separators, such as thousands separators or thousand/lakh/crore separators.
    /// The default is `Auto`.
    ///
    /// - `Always`: display grouping separators even if the locale prefers otherwise.
    /// - `Auto`: display grouping separators based on the locale preference, which may also be dependent on the currency.
    /// - `None`: do not display grouping separators.
    /// - `Min2`: display grouping separators when there are at least 2 digits in a group.
    use_grouping: NumberGrouping,

    /// Experimental.
    /// Options for rounding modes. The default is `HalfExpand`.
    ///
    /// - `Ceil`: round toward +∞. Positive values round up. Negative values round "more positive".
    /// - `Floor` round toward -∞. Positive values round down. Negative values round "more negative".
    /// - `Expand`: round away from 0. The _magnitude_ of the value is always increased by rounding. Positive values round up. Negative values round "more negative".
    /// - `Trunc`: round toward 0. This _magnitude_ of the value is always reduced by rounding. Positive values round down. Negative values round "less negative".
    /// - `HalfCeil`: ties toward +∞. Values above the half-increment round like `Ceil` (towards +∞), and below like `Floor` (towards -∞). On the half-increment, values round like `Ceil`.
    /// - `HalfFloor`: ties toward -∞. Values above the half-increment round like `Ceil` (towards +∞), and below like `Floor` (towards -∞). On the half-increment, values round like `Floor`.
    /// - `HalfExpand`: ties away from 0. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment, values round like `Expand`.
    /// - `HalfTrunc`: ties toward 0. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment, values round like `Trunc`.
    /// - `HalfEven`: ties towards the nearest even integer. Values above the half-increment round like `Expand` (away from zero), and below like `Trunc` (towards 0). On the half-increment values round towards the nearest even digit.
    ///
    /// These options reflect the [ICU user guide](https://unicode-org.github.io/icu/userguide/format_parse/numbers/rounding-modes.html), where `Expand` and `Trunc` map to ICU "UP" and "DOWN", respectively. The [rounding modes](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#rounding_modes) example demonstrates how each mode works.
    rounding_mode: RoundingMode,

    /// Experimental.
    /// Specify how rounding conflicts will be resolved if both "FractionDigits" ([`UseIntlNumberFormatOptions::minimum_fraction_digits`]/[`UseIntlNumberFormatOptions::maximum_fraction_digits`]) and "SignificantDigits" ([`UseIntlNumberFormatOptions::minimum_significant_digits`]/[`UseIntlNumberFormatOptions::maximum_significant_digits`]) are specified:
    ///
    /// - `Auto`: the result from the significant digits property is used (default).
    /// - `MorePrecision`: the result from the property that results in more precision is used.
    /// - `LessPrecision`: the result from the property that results in less precision is used.
    ///
    /// Note that for values other than `Auto` the result with more precision is calculated from the [`UseIntlNumberFormatOptions::maximum_significant_digits`] and [`UseIntlNumberFormatOptions::maximum_fraction_digits`] (minimum fractional and significant digit settings are ignored).
    rounding_priority: RoundingPriority,

    /// Experimental.
    /// Specifies the rounding-increment precision. Must be one of the following integers:
    /// `1` (default), `2`, `5`, `10`, `20`, `25`, `50`, `100`, `200`, `250`, `500`, `1000`, `2000`, `2500`, `5000`.
    ///
    /// This option controls the rounding increment to be used when formatting numbers:
    ///
    /// - It indicates the increment at which rounding should take place relative to the calculated rounding magnitude.
    /// - It cannot be mixed with significant-digits rounding or any setting of `rounding_priority` other than `Auto`.
    ///
    /// For example, if `maximum_fraction_digits` is 2 and `rounding_increment` is 5, then the number is rounded to the nearest 0.05 ("nickel rounding").
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{use_intl_number_format, UseIntlNumberFormatOptions, NumberStyle};
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let nf = use_intl_number_format(
    ///     UseIntlNumberFormatOptions::default()
    ///         .style(NumberStyle::Currency)
    ///         .currency("USD")
    ///         .maximum_fraction_digits(2)
    ///         .rounding_increment(5),    
    /// );
    ///
    /// let formatted = nf.format(11.29); // "$11.30"
    /// let formatted = nf.format(11.25); // "$11.25"
    /// let formatted = nf.format(11.22); // "$11.20"
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// If you set `minimum_fraction_digits` and `maximum_fraction_digits`, they must set them to the same value; otherwise a `RangeError` is thrown.
    rounding_increment: u16,

    /// Experimental.
    /// A string expressing the strategy for displaying trailing zeros on whole numbers. The default is `"auto"`.
    ///
    /// - `Auto`: keep trailing zeros according to `minimum_fraction_digits` and `minimum_significant_digits`.
    /// - `StripIfInteger`: remove the fraction digits _if_ they are all zero. This is the same as `Auto` if any of the fraction digits is non-zero.
    trailing_zero_display: TrailingZeroDisplay,

    /// These properties fall into two groups: `minimum_integer_digits`, `minimum_fraction_digits`,
    /// and `maximum_fraction_digits` in one group, `minimum_significant_digits` and `maximum_significant_digits`
    /// in the other. If properties from both groups are specified, conflicts in the resulting
    /// display format are resolved based on the value of the [`UseIntlNumberFormatOptions::rounding_priority`] property.
    ///
    /// The minimum number of integer digits to use. A value with a smaller number of integer digits
    /// than this number will be left-padded with zeros (to the specified length) when formatted.
    /// Possible values are from 1 to 21; the default is 1.
    minimum_integer_digits: u8,

    /// The minimum number of fraction digits to use. Possible values are from 0 to 20;
    /// the default for plain number and percent formatting is 0;
    /// the default for currency formatting is the number of minor unit digits provided by the
    /// [ISO 4217 currency code list](https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml)
    /// (2 if the list doesn't provide that information).
    #[builder(into)]
    minimum_fraction_digits: Option<u8>,

    /// The maximum number of fraction digits to use. Possible values are from 0 to 20;
    /// the default for plain number formatting is the larger of `minimum_fraction_digits` and 3;
    /// the default for currency formatting is the larger of `minimum_fraction_digits` and
    /// the number of minor unit digits provided by the
    /// [ISO 4217 currency code list](https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml)
    /// (2 if the list doesn't provide that information); the default for percent formatting is
    /// `minimum_fraction_digits`.
    #[builder(into)]
    maximum_fraction_digits: Option<u8>,

    /// The minimum number of significant digits to use. Possible values are from 1 to 21; the default is 1.
    #[builder(into)]
    minimum_significant_digits: Option<u8>,

    /// The maximum number of significant digits to use. Possible values are from 1 to 21; the default is 21.
    #[builder(into)]
    maximum_significant_digits: Option<u8>,
}

impl UseIntlNumberFormatOptions {
    /// A string with a BCP 47 language tag. Please refer to the
    /// [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#parameters)
    /// for more info.
    pub fn locale(self, locale: &str) -> Self {
        Self {
            locales: vec![locale.to_string()],
            ..self
        }
    }
}

impl Default for UseIntlNumberFormatOptions {
    fn default() -> Self {
        Self {
            locales: Default::default(),
            compact_display: Default::default(),
            currency: None,
            currency_display: Default::default(),
            currency_sign: Default::default(),
            locale_matcher: Default::default(),
            notation: Default::default(),
            numbering_system: None,
            sign_display: Default::default(),
            style: Default::default(),
            unit: None,
            unit_display: Default::default(),
            use_grouping: Default::default(),
            rounding_mode: Default::default(),
            rounding_priority: Default::default(),
            rounding_increment: 1,
            trailing_zero_display: Default::default(),
            minimum_integer_digits: 1,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
            minimum_significant_digits: None,
            maximum_significant_digits: None,
        }
    }
}

impl From<UseIntlNumberFormatOptions> for js_sys::Object {
    fn from(options: UseIntlNumberFormatOptions) -> Self {
        let obj = Self::new();

        js!(obj["compactDisplay"] = options.compact_display);

        if let Some(currency) = options.currency {
            js!(obj["currency"] = currency);
        }

        js!(obj["currencyDisplay"] = options.currency_display);
        js!(obj["currencySign"] = options.currency_sign);
        js!(obj["localeMatcher"] = options.locale_matcher);
        js!(obj["notation"] = options.notation);

        if let Some(numbering_system) = options.numbering_system {
            js!(obj["numberingSystem"] = numbering_system);
        }

        js!(obj["signDisplay"] = options.sign_display);
        js!(obj["style"] = options.style);

        if let Some(unit) = options.unit {
            js!(obj["unit"] = unit);
        }

        js!(obj["unitDisplay"] = options.unit_display);
        js!(obj["useGrouping"] = options.use_grouping);
        js!(obj["roundingMode"] = options.rounding_mode);
        js!(obj["roundingPriority"] = options.rounding_priority);
        js!(obj["roundingIncrement"] = options.rounding_increment);
        js!(obj["trailingZeroDisplay"] = options.trailing_zero_display);
        js!(obj["minimumIntegerDigits"] = options.minimum_integer_digits);

        if let Some(minimum_fraction_digits) = options.minimum_fraction_digits {
            js!(obj["minimumFractionDigits"] = minimum_fraction_digits);
        }
        if let Some(maximum_fraction_digits) = options.maximum_fraction_digits {
            js!(obj["maximumFractionDigits"] = maximum_fraction_digits);
        }

        if let Some(minimum_significant_digits) = options.minimum_significant_digits {
            js!(obj["minimumSignificantDigits"] = minimum_significant_digits);
        }
        if let Some(maximum_significant_digits) = options.maximum_significant_digits {
            js!(obj["maximumSignificantDigits"] = maximum_significant_digits);
        }

        obj
    }
}

cfg_if! { if #[cfg(feature = "ssr")] {
    /// Return type of [`use_intl_number_format`].
    pub struct UseIntlNumberFormatReturn;
} else {
    /// Return type of [`use_intl_number_format`].
    pub struct UseIntlNumberFormatReturn {
        /// The instance of [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat).
        pub js_intl_number_format: js_sys::Intl::NumberFormat,
    }
}}

impl UseIntlNumberFormatReturn {
    /// Formats a number according to the [locale and formatting options](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#parameters) of this `Intl.NumberFormat` object.
    /// See [`use_intl_number_format`] for more information.
    pub fn format<N>(&self, number: impl Into<MaybeSignal<N>>) -> Signal<String>
    where
        N: Clone + Display + 'static,
        js_sys::Number: From<N>,
    {
        let number = number.into();

        cfg_if! { if #[cfg(feature = "ssr")] {
            Signal::derive(move || {
                format!("{}", number.get())
            })
        } else {
            let number_format = self.js_intl_number_format.clone();

            Signal::derive(move || {
                if let Ok(result) = number_format
                    .format()
                    .call1(&number_format, &js_sys::Number::from(number.get()).into())
                {
                    result.as_string().unwrap_or_default()
                } else {
                    "".to_string()
                }
            })
        }}
    }

    /// Formats a range of numbers according to the locale and formatting options of this `Intl.NumberFormat` object.
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{NumberStyle, use_intl_number_format, UseIntlNumberFormatOptions};
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let nf = use_intl_number_format(
    ///     UseIntlNumberFormatOptions::default()
    ///         .locale("en-US")
    ///         .style(NumberStyle::Currency)
    ///         .currency("USD")
    ///         .maximum_fraction_digits(0),
    /// );
    ///
    /// let formatted = nf.format_range(3, 5); // "$3 – $5"
    ///
    /// // Note: the "approximately equals" symbol is added if
    /// // startRange and endRange round to the same values.
    /// let formatted = nf.format_range(2.9, 3.1); // "~$3"
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{NumberStyle, use_intl_number_format, UseIntlNumberFormatOptions};
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let nf = use_intl_number_format(
    ///     UseIntlNumberFormatOptions::default()
    ///         .locale("es-ES")
    ///         .style(NumberStyle::Currency)
    ///         .currency("EUR")
    ///         .maximum_fraction_digits(0),
    /// );
    ///
    /// let formatted = nf.format_range(3, 5); // "3-5 €"
    /// let formatted = nf.format_range(2.9, 3.1); // "~3 €"
    /// #
    /// # view! { }
    /// # }
    /// ```
    pub fn format_range<NStart, NEnd>(
        &self,
        start: impl Into<MaybeSignal<NStart>>,
        end: impl Into<MaybeSignal<NEnd>>,
    ) -> Signal<String>
    where
        NStart: Clone + Display + 'static,
        NEnd: Clone + Display + 'static,
        js_sys::Number: From<NStart>,
        js_sys::Number: From<NEnd>,
    {
        let start = start.into();
        let end = end.into();

        cfg_if! { if #[cfg(feature = "ssr")] {
            Signal::derive(move || {
                format!("{} - {}", start.get(), end.get())
            })
        } else {
            let number_format = self.js_intl_number_format.clone();

            Signal::derive(move || {
                if let Ok(function) = js!(number_format["formatRange"]) {
                    let function = function.unchecked_into::<js_sys::Function>();

                    if let Ok(result) = function.call2(
                        &number_format,
                        &js_sys::Number::from(start.get()).into(),
                        &js_sys::Number::from(end.get()).into(),
                    ) {
                        return result.as_string().unwrap_or_default();
                    }
                }

                "".to_string()
            })
        }}
    }

    // TODO : Allows locale-aware formatting of strings produced by this `Intl.NumberFormat` object.
    // pub fn format_to_parts<N>(
    //     &self,
    //     ,
    //     number: impl Into<MaybeSignal<N>>,
    // ) -> Signal<Vec<String>>
    // where
    //     N: Clone + 'static,
    //     f64: From<N>,
    // {
    //     let number = number.into();
    //     let number_format = self.js_intl_number_format.clone();
    //
    //     Signal::derive(move || {
    //         let array = number_format.format_to_parts(number.get().into());
    //
    //         array
    //             .to_vec()
    //             .iter()
    //             .map(|p| p.as_string().unwrap_or_default())
    //             .collect()
    //     })
    // }
}
