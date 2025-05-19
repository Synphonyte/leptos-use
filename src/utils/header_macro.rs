#![allow(unused_macros, unused_imports)]

macro_rules! get_header {
    (
        $header_name:expr,
        $function_name:ident,
        $option_name:ident
        $(,)?
    ) => {
        if cfg!(feature = "ssr") {
            #[cfg(all(
                not(feature = "axum"),
                not(feature = "actix")
            ))]
            {
                leptos::logging::warn!(
                    "If you're using `{}` with SSR but without any of the features `axum`, `actix` enabled, you have to provide the option `{}`",
                    stringify!($function_name),
                    stringify!($option_name)
                );
                return None;
            }

            #[cfg(feature = "actix")]
            #[allow(unused_imports)]
            use http0_2::{HeaderName, header::*};
            #[cfg(feature = "axum")]
            #[allow(unused_imports)]
            use http1::{HeaderName, header::*};

            #[cfg(any(feature = "axum", feature = "actix"))]
            {
                let header_name = $header_name;
                crate::utils::header(header_name)
            }
        } else {
            None
        }
    };
}

pub(crate) use get_header;
