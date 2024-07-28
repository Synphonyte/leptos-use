macro_rules! get_header {
    (
        $header_name:ident,
        $function_name:ident,
        $option_name:ident
        $(,)?
    ) => {
        if cfg!(feature = "ssr") {
            #[cfg(all(
                not(feature = "axum"),
                not(feature = "actix"),
                not(feature = "spin")
            ))]
            {
                leptos::logging::warn!(
                    "If you're using `{}` with SSR but without any of the features `axum`, `actix` or `spin` enabled, you have to provide the option `{}`",
                    stringify!($function_name),
                    stringify!($option_name)
                );
                return None;
            }

            #[cfg(feature = "actix")]
            const $header_name: http0_2::HeaderName = http0_2::header::$header_name;
            #[cfg(any(feature = "axum", feature = "spin"))]
            const $header_name: http1::HeaderName = http1::header::$header_name;

            #[cfg(any(feature = "axum", feature = "actix", feature = "spin"))]
            crate::utils::header($header_name)
        } else {
            None
        }
    };
}

pub(crate) use get_header;
