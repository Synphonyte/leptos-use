use cookie::Cookie;

pub struct UseCookie {
    pub cookie: Option<Cookie<'static>>,
}

pub fn use_cookie(cookie_name: &str) -> UseCookie {
    let cookies;
    #[cfg(feature = "ssr")]
    {
        use http::HeaderValue;
        use leptos::expect_context;

        let headers;
        #[cfg(feature = "actix")]
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
        
        let js_value: wasm_bindgen::JsValue = leptos::window().document().unwrap().into();
        let document: web_sys::HtmlDocument = js_value.unchecked_into();
        cookies = document.cookie().unwrap_or_default();
    }

    let cookie = Cookie::split_parse_encoded(cookies)
        .into_iter()
        .filter_map(|cookie| cookie.ok())
        .find(|cookie| cookie.name() == cookie_name)
        .map(|cookie| cookie.into_owned());

    UseCookie { cookie }
}
