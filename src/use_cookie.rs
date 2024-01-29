use cookie::Cookie;
use default_struct_builder::DefaultBuilder;
use std::rc::Rc;

/// Get a cookie by name, for both SSR and CSR
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_cookie)
///
/// ## Usage
///
/// This provides you with the cookie that has been set. For more details on how to use the cookie provided, refer: https://docs.rs/cookie/0.18/cookie/struct.Cookie.html
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_cookie;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// if let Some(cookie) = use_cookie("auth") {
///     view! {
///         <div>
///             format!("'auth' cookie set to `{}`", cookie.value())
///         </div>
///     }.into_view()
/// } else {
///     view! {
///         <div>
///             "No 'auth' cookie set"
///         </div>
///     }.into_view()
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// This works equally well on the server or the client.
/// On the server this function gets the cookie from the HTTP request header.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum nor Actix, or the default implementation is not to your liking,
/// you can provide your own way of reading the cookie header value.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_cookie_with_options, UseCookieOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_cookie_with_options("auth", UseCookieOptions::default().ssr_cookies_header_getter(|| {
///     #[cfg(feature = "ssr")]
///     {
///         "Somehow get the value of the cookie header as a string".to_owned()
///     }
/// }));
/// # view! {}
/// # }
/// ```
pub fn use_cookie(cookie_name: &str) -> Option<Cookie<'static>> {
    use_cookie_with_options(cookie_name, UseCookieOptions::default())
}

/// Version of [`use_cookie`] that takes [`UseCookieOptions`].
pub fn use_cookie_with_options(
    cookie_name: &str,
    options: UseCookieOptions,
) -> Option<Cookie<'static>> {
    let UseCookieOptions {
        ssr_cookies_header_getter,
    } = options;

    let cookie = read_cookies_string(ssr_cookies_header_getter);

    Cookie::split_parse_encoded(cookies)
        .filter_map(|cookie| cookie.ok())
        .find(|cookie| cookie.name() == cookie_name)
        .map(|cookie| cookie.into_owned())
}

/// Options for [`use_cookie_with_options`].
#[derive(Clone, DefaultBuilder)]
pub struct UseCookieOptions {
    /// Getter function to return the string value of the cookie header.
    /// When you use one of the features "axum" or "actix" there's a valid default implementation provided.
    ssr_cookies_header_getter: Box<dyn Fn() -> String>,
}

impl Default for UseCookieOptions {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            ssr_cookies_header_getter: Box::new(move || {
                #[cfg(feature = "ssr")]
                {
                    #[cfg(any(feature = "axum", feature = "actix"))]
                    use leptos::expect_context;

                    #[cfg(all(feature = "actix", feature = "axum"))]
                    compile_error!("You cannot enable only one of features \"actix\" and \"axum\" at the same time");

                    #[cfg(feature = "actix")]
                    const COOKIE: http0_2::HeaderName = http0_2::header::COOKIE;
                    #[cfg(feature = "axum")]
                    const COOKIE: http1::HeaderName = http1::header::COOKIE;

                    #[cfg(feature = "actix")]
                    type HeaderValue = http0_2::HeaderValue;
                    #[cfg(feature = "axum")]
                    type HeaderValue = http1::HeaderValue;

                    #[cfg(any(feature = "axum", feature = "actix"))]
                    let headers;
                    #[cfg(feature = "actix")]
                    {
                        headers = expect_context::<actix_web::HttpRequest>().headers().clone();
                    }
                    #[cfg(feature = "axum")]
                    {
                        headers = expect_context::<http1::request::Parts>().headers;
                    }

                    #[cfg(all(not(feature = "axum"), not(feature = "actix")))]
                    {
                        leptos::logging::warn!("If you're using use_cookie without the feature `axum` or `actix` enabled, you should provide the option `ssr_cookies_header_getter`");
                        "".to_owned()
                    }

                    #[cfg(any(feature = "axum", feature = "actix"))]
                    headers
                        .get(COOKIE)
                        .cloned()
                        .unwrap_or_else(|| HeaderValue::from_static(""))
                        .to_str()
                        .unwrap_or_default()
                        .to_owned()
                }
                #[cfg(not(feature = "ssr"))]
                "".to_owned()
            }),
        }
    }
}

fn read_cookies_string(ssr_cookies_header_getter: Box<dyn Fn() -> String>) -> String {
    #[cfg(feature = "ssr")]
    ssr_cookies_header_getter();

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        let js_value: wasm_bindgen::JsValue = leptos::document().into();
        let document: web_sys::HtmlDocument = js_value.unchecked_into();
        document.cookie().unwrap_or_default()
    }
}
