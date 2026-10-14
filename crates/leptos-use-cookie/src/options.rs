use crate::get_header;
use codee::CodecError;
use cookie::{Cookie, SameSite};
use default_struct_builder::DefaultBuilder;
use leptos::logging::error;
use leptos::prelude::*;
use std::sync::Arc;

/// Options for [`use_cookie_with_options`](crate::use_cookie_with_options).
#[derive(DefaultBuilder)]
pub struct UseCookieOptions<T, E, D> {
    /// [`Max-Age` of the cookie](https://tools.ietf.org/html/rfc6265#section-5.2.2) in milliseconds. The returned signal will turn to `None` after the max age is reached.
    /// Default: `None`
    ///
    /// > The [cookie storage model specification](https://tools.ietf.org/html/rfc6265#section-5.3) states
    /// > that if both `expires` and `max_age` is set, then `max_age` takes precedence,
    /// > but not all clients may obey this, so if both are set, they should point to the same date and time!
    ///
    /// > If neither of `expires` and `max_age` is set, the cookie will be session-only and removed when the user closes their browser.
    #[builder(into)]
    pub(crate) max_age: Option<i64>,

    /// [Expiration date-time of the cookie](https://tools.ietf.org/html/rfc6265#section-5.2.1) as UNIX timestamp in seconds.
    /// The signal will turn to `None` after the expiration date-time is reached.
    /// Default: `None`
    ///
    /// > The [cookie storage model specification](https://tools.ietf.org/html/rfc6265#section-5.3) states
    /// > that if both `expires` and `max_age` is set, then `max_age` takes precedence,
    /// > but not all clients may obey this, so if both are set, they should point to the same date and time!
    ///
    /// > If neither of `expires` and `max_age` is set, the cookie will be session-only and removed when the user closes their browser.
    #[builder(into)]
    pub(crate) expires: Option<i64>,

    /// Specifies the [`HttpOnly` cookie attribute](https://tools.ietf.org/html/rfc6265#section-5.2.6).
    /// When `true`, the `HttpOnly` attribute is set; otherwise it is not.
    /// By default, the `HttpOnly` attribute is not set.
    ///
    /// > Be careful when setting this to `true`, as compliant clients will not allow client-side JavaScript to see the cookie in `document.cookie`.
    pub(crate) http_only: bool,

    /// Specifies the value for the [`Secure` cookie attribute](https://tools.ietf.org/html/rfc6265#section-5.2.5).
    /// When `true`, the `Secure` attribute is set; otherwise it is not.
    /// By default, the `Secure` attribute is not set.
    ///
    /// > Be careful when setting this to `true`, as compliant clients will not send the cookie back to the
    /// > server in the future if the browser does not have an HTTPS connection. This can lead to hydration errors.
    pub(crate) secure: bool,

    /// Specifies the value for the [`Domain` cookie attribute](https://tools.ietf.org/html/rfc6265#section-5.2.3).
    /// By default, no domain is set, and most clients will consider applying the cookie only to the current domain.
    #[builder(into)]
    pub(crate) domain: Option<String>,

    /// Specifies the value for the [`Path` cookie attribute](https://tools.ietf.org/html/rfc6265#section-5.2.4).
    /// By default, the path is considered the ["default path"](https://tools.ietf.org/html/rfc6265#section-5.1.4).
    #[builder(into)]
    pub(crate) path: Option<String>,

    /// Specifies the value for the [`SameSite` cookie attribute](https://tools.ietf.org/html/draft-ietf-httpbis-rfc6265bis-03#section-4.1.2.7).
    ///
    /// - `'Some(SameSite::Lax)'` will set the `SameSite` attribute to `Lax` for lax same-site enforcement.
    /// - `'Some(SameSite::None)'` will set the `SameSite` attribute to `None` for an explicit cross-site cookie.
    /// - `'Some(SameSite::Strict)'` will set the `SameSite` attribute to `Strict` for strict same-site enforcement.
    /// - `None` will not set the `SameSite` attribute (default).
    ///
    /// More information about the different enforcement levels can be found in [the specification](https://tools.ietf.org/html/draft-ietf-httpbis-rfc6265bis-03#section-4.1.2.7).
    #[builder(into)]
    pub(crate) same_site: Option<SameSite>,

    /// The default cookie value in case the cookie is not set.
    /// Defaults to `None`.
    pub(crate) default_value: Option<T>,

    /// If `true` the returned `WriteSignal` will not affect the actual cookie.
    /// Default: `false`
    pub(crate) readonly: bool,

    /// Getter function to return the string value of the cookie header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default implementation provided.
    pub(crate) ssr_cookies_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,

    /// Function to add a set cookie header to the response on the server.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default implementation provided.
    pub(crate) ssr_set_cookie: Arc<dyn Fn(&Cookie) + Send + Sync>,

    /// Callback for encoding/decoding errors. Defaults to logging the error to the console.
    pub(crate) on_error: Arc<dyn Fn(CodecError<E, D>) + Send + Sync>,
}

impl<T, E, D> Default for UseCookieOptions<T, E, D> {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            max_age: None,
            expires: None,
            http_only: false,
            default_value: None,
            readonly: false,
            secure: false,
            domain: None,
            path: None,
            same_site: None,
            ssr_cookies_header_getter: Arc::new(move || {
                get_header!(COOKIE, use_cookie, ssr_cookies_header_getter)
            }),
            ssr_set_cookie: Arc::new(|cookie: &Cookie| {
                #[cfg(feature = "ssr")]
                {
                    server_set_cookie(cookie);
                }

                let _ = cookie;
            }),
            on_error: Arc::new(|_| {
                error!("cookie (de-/)serialization error");
            }),
        }
    }
}

#[cfg(feature = "ssr")]
fn server_set_cookie(cookie: &Cookie) {
    #[cfg(feature = "actix")]
    use leptos_actix::ResponseOptions;
    #[cfg(feature = "axum")]
    use leptos_axum::ResponseOptions;

    #[cfg(feature = "actix")]
    const SET_COOKIE: http0_2::HeaderName = http0_2::header::SET_COOKIE;
    #[cfg(feature = "axum")]
    const SET_COOKIE: http1::HeaderName = http1::header::SET_COOKIE;

    #[cfg(feature = "actix")]
    type HeaderValue = http0_2::HeaderValue;
    #[cfg(feature = "axum")]
    type HeaderValue = http1::HeaderValue;

    #[cfg(all(not(feature = "axum"), not(feature = "actix")))]
    {
        use leptos::logging::warn;
        let _ = cookie;
        warn!(
            "If you're using use_cookie without the feature `axum` or `actix` enabled, you should provide the option `ssr_set_cookie`"
        );
    }

    #[cfg(any(feature = "axum", feature = "actix"))]
    {
        if let Some(response_options) = use_context::<ResponseOptions>()
            && let Ok(header_value) = HeaderValue::from_str(&cookie.encoded().to_string())
        {
            response_options.append_header(SET_COOKIE, header_value);
        }
    }
}
