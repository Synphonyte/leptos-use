#[cfg(not(feature = "ssr"))]
use crate::{js, sendwrap_fn};
use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::fmt::Display;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

cfg_if! { if #[cfg(feature = "ssr")] {
    /// Return type of [`use_intl_number_format`].
    pub struct UseIntlNumberFormatReturn;
} else {
    /// Return type of [`use_intl_number_format`](crate::use_intl_number_format).
    pub struct UseIntlNumberFormatReturn {
        /// The instance of [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat).
        pub js_intl_number_format: js_sys::Intl::NumberFormat,
    }
}}

impl UseIntlNumberFormatReturn {
    /// Formats a number according to the [locale and formatting options](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#parameters) of this `Intl.NumberFormat` object.
    /// See [`use_intl_number_format`](crate::use_intl_number_format) for more information.
    /// In the browser this uses `SendWrapper` internally so the returned signal can only be used on
    /// the same thread where this method was called.
    pub fn format<N>(&self, number: impl Into<Signal<N>>) -> Signal<String>
    where
        N: Clone + Display + Send + Sync + 'static,
        js_sys::Number: From<N>,
    {
        let number = number.into();

        cfg_if! { if #[cfg(feature = "ssr")] {
            Signal::derive(move || {
                format!("{}", number.get())
            })
        } else {
            let number_format = self.js_intl_number_format.clone();

            Signal::derive(sendwrap_fn!(move || {
                if let Ok(result) = number_format
                    .format()
                    .call1(&number_format, &js_sys::Number::from(number.get()).into())
                {
                    result.as_string().unwrap_or_default()
                } else {
                    "".to_string()
                }
            }))
        }}
    }

    /// Formats a range of numbers according to the locale and formatting options of this `Intl.NumberFormat` object.
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use_intl_number_format::{NumberStyle, use_intl_number_format, UseIntlNumberFormatOptions};
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
    /// let formatted = nf.format_range::<i32, i32>(3, 5); // "$3 – $5"
    ///
    /// // Note: the "approximately equals" symbol is added if
    /// // startRange and endRange round to the same values.
    /// let formatted = nf.format_range::<f64, f64>(2.9, 3.1); // "~$3"
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use_intl_number_format::{NumberStyle, use_intl_number_format, UseIntlNumberFormatOptions};
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
    /// let formatted = nf.format_range::<i32, i32>(3, 5); // "3-5 €"
    /// let formatted = nf.format_range::<f64, f64>(2.9, 3.1); // "~3 €"
    /// #
    /// # view! { }
    /// # }
    /// ```
    pub fn format_range<NStart, NEnd>(
        &self,
        start: impl Into<Signal<NStart>>,
        end: impl Into<Signal<NEnd>>,
    ) -> Signal<String>
    where
        NStart: Clone + Display + Send + Sync + 'static,
        NEnd: Clone + Display + Send + Sync + 'static,
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

            Signal::derive(
                sendwrap_fn!(move || {
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
            )
        }}
    }

    // TODO : Allows locale-aware formatting of strings produced by this `Intl.NumberFormat` object.
    // pub fn format_to_parts<N>(
    //     &self,
    //     ,
    //     number: impl Into<Signal<N>>,
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
