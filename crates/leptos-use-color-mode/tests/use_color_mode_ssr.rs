#![cfg(all(feature = "ssr", feature = "axum", not(target_arch = "wasm32")))]

//! Server-side rendering with the color mode stored in a cookie.
//!
//! Most browsers never send the `Sec-CH-Prefers-Color-Scheme` header, so for `Auto` the server
//! renders the system mode that the client stored with it in the cookie (`auto-dark` /
//! `auto-light`).

use leptos::prelude::*;
use leptos_use_color_mode::{
    ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options,
};

const COOKIE_NAME: &str = "leptos-use-color-scheme";

/// Renders `use_color_mode` with cookies enabled for a request with the given headers. The
/// returned owner has to be kept alive while the signals are read.
fn render(headers: &[(&str, &str)]) -> (Owner, UseColorModeReturn) {
    let owner = Owner::new();
    owner.set();

    let mut request = http1::Request::builder();
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let (parts, _) = request.body(()).unwrap().into_parts();
    provide_context(parts);

    let color_mode =
        use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    (owner, color_mode)
}

fn cookie(value: &str) -> String {
    format!("{COOKIE_NAME}={value}")
}

#[test]
fn auto_renders_the_system_mode_stored_in_the_cookie() {
    let (
        _owner,
        UseColorModeReturn {
            mode,
            store,
            system,
            ..
        },
    ) = render(&[("cookie", &cookie("auto-dark"))]);

    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(system.get_untracked(), ColorMode::Dark);
    assert_eq!(mode.get_untracked(), ColorMode::Dark);

    let (_owner, UseColorModeReturn { mode, store, .. }) =
        render(&[("cookie", &cookie("auto-light"))]);

    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(mode.get_untracked(), ColorMode::Light);
}

#[test]
fn explicit_modes_are_rendered_from_the_cookie() {
    let (_owner, UseColorModeReturn { mode, store, .. }) = render(&[("cookie", &cookie("dark"))]);

    assert_eq!(store.get_untracked(), ColorMode::Dark);
    assert_eq!(mode.get_untracked(), ColorMode::Dark);
}

#[test]
fn the_color_scheme_header_wins_over_the_cookie() {
    let (_owner, UseColorModeReturn { mode, system, .. }) = render(&[
        ("cookie", &cookie("auto-light")),
        ("sec-ch-prefers-color-scheme", "dark"),
    ]);

    assert_eq!(system.get_untracked(), ColorMode::Dark);
    assert_eq!(mode.get_untracked(), ColorMode::Dark);
}

#[test]
fn without_cookie_and_header_auto_renders_light() {
    let (_owner, UseColorModeReturn { mode, store, .. }) = render(&[]);

    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(mode.get_untracked(), ColorMode::Light);
}
