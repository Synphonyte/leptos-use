use cookie::Cookie;

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
///    if let Some(cookie) = use_cookie("auth") {
///        view! {
///            <div>
///                format!("'auth' cookie set to `{}`", cookie.value())
///            </div>
///        }.into_view()
///    } else {
///        view! {
///            <div>
///                "No 'auth' cookie set"
///            </div>
///        }.into_view()
///    }
/// # }
/// ```
pub fn use_cookie(cookie_name: &str) -> Option<Cookie<'static>> {
    let cookies;
    #[cfg(feature = "ssr")]
    {
        use http::HeaderValue;
        use leptos::expect_context;

        let headers;
        #[cfg(not(feature = "axum"))]
        {
            headers = expect_context::<actix_web::HttpRequest>().headers().clone();
        }
        #[cfg(feature = "axum")]
        {
            headers = expect_context::<leptos_axum::RequestParts>().headers;
        }
        cookies = headers
            .get(http::header::COOKIE)
            .cloned()
            .unwrap_or_else(|| HeaderValue::from_static(""))
            .to_str()
            .unwrap_or_default()
            .to_owned();
    }
    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        let js_value: wasm_bindgen::JsValue = leptos::document().into();
        let document: web_sys::HtmlDocument = js_value.unchecked_into();
        cookies = document.cookie().unwrap_or_default();
    }

    Cookie::split_parse_encoded(cookies)
        .filter_map(|cookie| cookie.ok())
        .find(|cookie| cookie.name() == cookie_name)
        .map(|cookie| cookie.into_owned())
}
