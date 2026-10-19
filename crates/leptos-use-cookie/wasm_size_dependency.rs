//! Uses the `cookie` crate like `use_cookie` does: parsing `document.cookie` into a jar and
//! building an encoded cookie with all options. Used to measure the wasm size of the crate only.

use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, CookieJar, SameSite};
use leptos::prelude::*;
use std::hint::black_box;

#[component]
fn DependencyDemo() -> impl IntoView {
    let mut jar = CookieJar::new();
    for cookie in
        Cookie::split_parse_encoded(black_box("counter=1; theme=dark").to_string()).flatten()
    {
        jar.add_original(cookie);
    }

    let mut cookie = Cookie::build(("counter", black_box("2")))
        .max_age(Duration::milliseconds(black_box(3_600_000)))
        .http_only(black_box(false))
        .secure(black_box(true))
        .domain(black_box("example.com"))
        .path(black_box("/"))
        .same_site(black_box(SameSite::Lax));
    if let Ok(expires) = OffsetDateTime::from_unix_timestamp(black_box(1_700_000_000)) {
        cookie = cookie.expires(expires);
    }
    let cookie: Cookie = cookie.into();
    jar.add(cookie.into_owned());

    let encoded = jar
        .get("counter")
        .map(|c| c.encoded().to_string())
        .unwrap_or_default();

    view! { <p>{encoded}</p> }
}
