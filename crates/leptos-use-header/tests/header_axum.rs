//! Runs natively: `cargo test -p leptos-use-header --features ssr,axum`
#![cfg(all(not(target_arch = "wasm32"), feature = "ssr", feature = "axum"))]

use http1::Request;
use http1::header::ACCEPT_LANGUAGE;
use leptos::prelude::*;
use leptos_use_header::{get_header, header};

fn provide_request(accept_language: Option<&str>) -> Owner {
    let owner = Owner::new();
    owner.set();

    let mut request = Request::builder().uri("/");
    if let Some(value) = accept_language {
        request = request.header(ACCEPT_LANGUAGE, value);
    }
    let (parts, _) = request.body(()).unwrap().into_parts();
    provide_context(parts);

    owner
}

fn accept_language() -> Option<String> {
    get_header!(ACCEPT_LANGUAGE, accept_language, header_getter)
}

#[test]
fn reads_a_header_of_the_current_request() {
    let _owner = provide_request(Some("de-AT,en;q=0.5"));

    assert_eq!(header(ACCEPT_LANGUAGE), Some("de-AT,en;q=0.5".to_string()));
    assert_eq!(accept_language(), Some("de-AT,en;q=0.5".to_string()));
}

#[test]
fn is_none_for_a_missing_header() {
    let _owner = provide_request(None);

    assert_eq!(header(ACCEPT_LANGUAGE), None);
}

#[test]
fn is_none_outside_of_a_request() {
    let owner = Owner::new();
    owner.set();

    assert_eq!(header(ACCEPT_LANGUAGE), None);
}
