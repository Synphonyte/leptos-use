#[cfg(feature = "actix")]
use http0_2::HeaderName;
#[cfg(feature = "axum")]
use http1::HeaderName;
use leptos::prelude::*;

/// Get the value of the header with the given name.
///
/// This function is only meant to be used on the server.
/// So it is only defined when the feature `"ssr"` is enabled together with one of the
/// features `"axum"` or `"actix"`.
///
/// ## Example
///
/// ```ignore
/// # use leptos_use::utils::header;
/// #
/// let content_len = header(http::header::CONTENT_LENGTH);
/// ```
pub fn header<N>(name: N) -> Option<String>
where
    N: Into<HeaderName>,
{
    let name = name.into();

    #[cfg(all(feature = "actix", feature = "axum"))]
    compile_error!("You can only enable one of features \"actix\" and \"axum\" at the same time");

    #[cfg(feature = "actix")]
    type HeaderValue = http0_2::HeaderValue;
    #[cfg(feature = "axum")]
    type HeaderValue = http1::HeaderValue;

    #[cfg(any(feature = "axum", feature = "actix"))]
    let headers;
    #[cfg(feature = "actix")]
    {
        headers =
            use_context::<leptos_actix::Request>().map(|req| req.into_inner().headers().clone());
    }
    #[cfg(feature = "axum")]
    {
        headers = use_context::<http1::request::Parts>().map(|parts| parts.headers);
    }

    #[cfg(any(feature = "axum", feature = "actix"))]
    {
        headers.map(|headers| {
            headers
                .get(name)
                .cloned()
                .unwrap_or_else(|| HeaderValue::from_static(""))
                .to_str()
                .unwrap_or_default()
                .to_owned()
        })
    }
}
