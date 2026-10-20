//! Uses `unic-langid` like `use_locale` does: parsing and matching language identifiers. Used to
//! measure the wasm size of the crate only.

use leptos::prelude::*;
use std::hint::black_box;
use unic_langid::LanguageIdentifier;

#[component]
fn DependencyDemo() -> impl IntoView {
    let supported: Vec<LanguageIdentifier> = black_box(["en", "de-CH"])
        .iter()
        .filter_map(|l| l.parse().ok())
        .collect();

    let locale = black_box("de-CH")
        .parse::<LanguageIdentifier>()
        .ok()
        .and_then(|client| {
            supported
                .iter()
                .find(|s| client.matches(*s, true, true))
                .cloned()
        })
        .unwrap_or_default();

    view! { <p>{locale.to_string()}</p> }
}
