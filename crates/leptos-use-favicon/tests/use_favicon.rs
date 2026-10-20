#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_favicon::{UseFaviconOptions, use_favicon, use_favicon_with_options};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_link(rel: &str) -> web_sys::HtmlLinkElement {
    let link: web_sys::HtmlLinkElement =
        document().create_element("link").unwrap().unchecked_into();
    link.set_rel(rel);
    document().head().unwrap().append_child(&link).unwrap();
    link
}

fn href_path(link: &web_sys::HtmlLinkElement) -> String {
    link.get_attribute("href").unwrap_or_default()
}

#[wasm_bindgen_test]
async fn sets_the_href_of_icon_links() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let icon = append_link("icon");
    let shortcut = append_link("shortcut icon");
    let stylesheet = append_link("stylesheet");

    let (favicon, set_favicon) = use_favicon();
    Executor::tick().await;
    assert_eq!(favicon.get_untracked(), None);

    set_favicon.set(Some("dark.png".to_string()));
    Executor::tick().await;

    assert_eq!(href_path(&icon), "dark.png");
    assert_eq!(href_path(&shortcut), "dark.png");
    assert_eq!(href_path(&stylesheet), "", "only icon links change");
}

#[wasm_bindgen_test]
async fn prefixes_the_base_url_and_follows_a_signal() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let link = append_link("apple-touch-icon");
    let (icon, set_icon) = signal(Some("a.png".to_string()));

    let _ = use_favicon_with_options(
        UseFaviconOptions::default()
            .new_icon(icon)
            .base_url("/static/")
            .rel("apple-touch-icon"),
    );
    Executor::tick().await;

    set_icon.set(Some("b.png".to_string()));
    Executor::tick().await;

    assert_eq!(href_path(&link), "/static/b.png");
}
