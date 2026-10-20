#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_color_mode::{
    ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// The target option takes a selector, so every element gets an id.
fn append_div(id: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_id(id);
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn local() -> web_sys::Storage {
    window().local_storage().unwrap().unwrap()
}

fn system_mode() -> ColorMode {
    if window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
    {
        ColorMode::Dark
    } else {
        ColorMode::Light
    }
}

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn applies_the_mode_to_the_html_element_by_default() {
    let _owner = setup().await;

    let html = document().document_element().unwrap();
    let UseColorModeReturn { set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().storage_enabled(false));
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(20).await;
    assert!(html.class_list().contains("dark"));

    set_mode.set(ColorMode::Light);
    TimeoutFuture::new(20).await;
    assert!(html.class_list().contains("light"));
    assert!(!html.class_list().contains("dark"));

    html.class_list().remove_1("light").unwrap();
}

#[wasm_bindgen_test]
async fn applies_the_mode_as_a_class() {
    let _owner = setup().await;

    let target = append_div("color-mode-class");
    let UseColorModeReturn {
        mode,
        set_mode,
        store,
        system,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-class")
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(system.get_untracked(), system_mode());
    assert_eq!(
        mode.get_untracked(),
        system_mode(),
        "auto resolves to the system mode"
    );

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(20).await;
    assert_eq!(mode.get_untracked(), ColorMode::Dark);
    assert!(target.class_list().contains("dark"));
    assert!(!target.class_list().contains("light"));

    set_mode.set(ColorMode::Light);
    TimeoutFuture::new(20).await;
    assert!(target.class_list().contains("light"));
    assert!(!target.class_list().contains("dark"));
}

#[wasm_bindgen_test]
async fn applies_the_mode_to_a_custom_attribute() {
    let _owner = setup().await;

    let target = append_div("color-mode-attribute");
    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-attribute")
            .attribute("data-theme")
            .custom_modes(vec!["sepia".to_string()])
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Custom("sepia".to_string()));
    TimeoutFuture::new(20).await;

    assert_eq!(target.get_attribute("data-theme").as_deref(), Some("sepia"));
}

#[wasm_bindgen_test]
async fn emit_auto_reports_auto_instead_of_the_resolved_mode() {
    let _owner = setup().await;

    append_div("color-mode-emit-auto");
    let UseColorModeReturn { mode, state, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-emit-auto")
            .emit_auto(true)
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    assert_eq!(mode.get_untracked(), ColorMode::Auto);
    assert_eq!(state.get_untracked(), system_mode());
}

#[wasm_bindgen_test]
async fn persists_the_mode_in_local_storage() {
    let _owner = setup().await;
    local().remove_item("color-mode-test").unwrap();
    append_div("color-mode-persist-a");
    append_div("color-mode-persist-b");

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-persist-a")
            .storage_key("color-mode-test"),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(50).await;
    assert_eq!(
        local().get_item("color-mode-test").unwrap().as_deref(),
        Some("dark")
    );

    // a new instance starts with the persisted mode
    let UseColorModeReturn { store, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-persist-b")
            .storage_key("color-mode-test"),
    );
    TimeoutFuture::new(20).await;
    assert_eq!(store.get_untracked(), ColorMode::Dark);

    local().remove_item("color-mode-test").unwrap();
}

fn html_document() -> web_sys::HtmlDocument {
    document().unchecked_into()
}

fn cookie(name: &str) -> Option<String> {
    html_document()
        .cookie()
        .unwrap()
        .split("; ")
        .find_map(|c| c.strip_prefix(&format!("{name}=")).map(str::to_string))
}

fn set_cookie(name: &str, value: &str) {
    html_document()
        .set_cookie(&format!("{name}={value}; path=/"))
        .unwrap();
}

fn remove_cookie(name: &str) {
    html_document()
        .set_cookie(&format!("{name}=; Max-Age=0; path=/"))
        .unwrap();
}

/// Records every string assigned to `document.cookie`, as the attributes (like `Max-Age`) can't
/// be read back.
fn record_cookie_writes() {
    js_sys::eval(
        r#"
        if (!window.__cookieWrites) {
            const desc = Object.getOwnPropertyDescriptor(Document.prototype, "cookie");
            Object.defineProperty(document, "cookie", {
                configurable: true,
                get() { return desc.get.call(document); },
                set(value) { window.__cookieWrites.push(value); desc.set.call(document, value); },
            });
        }
        window.__cookieWrites = [];
        "#,
    )
    .unwrap();
}

fn cookie_writes(name: &str) -> Vec<String> {
    js_sys::eval("window.__cookieWrites")
        .unwrap()
        .unchecked_into::<js_sys::Array>()
        .iter()
        .filter_map(|w| w.as_string())
        .filter(|w| w.starts_with(&format!("{name}=")))
        .collect()
}

/// Replaces the `(prefers-color-scheme: dark)` media query with one that `set_system_dark` controls.
fn fake_system_mode(dark: bool) {
    js_sys::eval(&format!(
        r#"
        window.__realMatchMedia = window.__realMatchMedia || window.matchMedia;
        const mql = new EventTarget();
        Object.setPrototypeOf(mql, MediaQueryList.prototype);
        Object.defineProperty(mql, "matches", {{ value: {dark}, writable: true }});
        Object.defineProperty(mql, "media", {{ value: "(prefers-color-scheme: dark)" }});
        window.__fakeDarkQuery = mql;
        window.matchMedia = (query) => query === "(prefers-color-scheme: dark)"
            ? mql
            : window.__realMatchMedia.call(window, query);
        "#
    ))
    .unwrap();
}

fn set_system_dark(dark: bool) {
    js_sys::eval(&format!(
        r#"
        window.__fakeDarkQuery.matches = {dark};
        window.__fakeDarkQuery.dispatchEvent(new Event("change"));
        "#
    ))
    .unwrap();
}

fn restore_system_mode() {
    js_sys::eval("if (window.__realMatchMedia) window.matchMedia = window.__realMatchMedia;")
        .unwrap();
}

#[wasm_bindgen_test]
async fn cookie_stores_auto_with_the_system_mode() {
    let _owner = setup().await;
    fake_system_mode(true);
    remove_cookie("color-mode-cookie-auto");
    append_div("color-mode-cookie-auto");

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-auto")
            .cookie_name("color-mode-cookie-auto")
            .cookie_enabled(true),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Light);
    TimeoutFuture::new(20).await;
    assert_eq!(
        cookie("color-mode-cookie-auto").as_deref(),
        Some("light"),
        "explicit modes are stored as they are"
    );

    set_mode.set(ColorMode::Auto);
    TimeoutFuture::new(20).await;
    assert_eq!(
        cookie("color-mode-cookie-auto").as_deref(),
        Some("auto-dark"),
        "auto is stored with the system mode it resolves to"
    );

    restore_system_mode();
    remove_cookie("color-mode-cookie-auto");
}

#[wasm_bindgen_test]
async fn cookie_follows_a_system_switch_while_auto() {
    let _owner = setup().await;
    fake_system_mode(false);
    set_cookie("color-mode-cookie-switch", "auto-light");
    append_div("color-mode-cookie-switch");

    let UseColorModeReturn { mode, store, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-switch")
            .cookie_name("color-mode-cookie-switch")
            .cookie_enabled(true),
    );
    TimeoutFuture::new(20).await;
    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(mode.get_untracked(), ColorMode::Light);

    set_system_dark(true);
    TimeoutFuture::new(20).await;
    assert_eq!(mode.get_untracked(), ColorMode::Dark);
    assert_eq!(
        cookie("color-mode-cookie-switch").as_deref(),
        Some("auto-dark")
    );

    restore_system_mode();
    remove_cookie("color-mode-cookie-switch");
}

#[wasm_bindgen_test]
async fn starts_with_the_mode_from_the_cookie() {
    let _owner = setup().await;
    set_cookie("color-mode-cookie-initial", "dark");
    set_cookie("color-mode-cookie-initial-auto", "auto-dark");
    append_div("color-mode-cookie-initial");

    let UseColorModeReturn { store, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-initial")
            .cookie_name("color-mode-cookie-initial")
            .cookie_enabled(true),
    );
    // no waiting: the value has to be there before the first effect runs to hydrate correctly
    assert_eq!(store.get_untracked(), ColorMode::Dark);

    let UseColorModeReturn { store, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-initial")
            .cookie_name("color-mode-cookie-initial-auto")
            .cookie_enabled(true),
    );
    assert_eq!(store.get_untracked(), ColorMode::Auto);

    remove_cookie("color-mode-cookie-initial");
    remove_cookie("color-mode-cookie-initial-auto");
}

#[wasm_bindgen_test]
async fn cookie_replaces_local_storage() {
    let _owner = setup().await;
    local().remove_item("color-mode-cookie-only").unwrap();
    remove_cookie("color-mode-cookie-only");
    append_div("color-mode-cookie-only");

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-only")
            .storage_key("color-mode-cookie-only")
            .cookie_name("color-mode-cookie-only")
            .cookie_enabled(true),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(50).await;
    assert_eq!(cookie("color-mode-cookie-only").as_deref(), Some("dark"));
    assert_eq!(
        local().get_item("color-mode-cookie-only").unwrap(),
        None,
        "storage_enabled is ignored when the cookie is enabled"
    );

    remove_cookie("color-mode-cookie-only");
}

#[wasm_bindgen_test]
async fn cookie_max_age_defaults_to_a_year() {
    let _owner = setup().await;
    record_cookie_writes();
    append_div("color-mode-cookie-max-age");

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-max-age")
            .cookie_name("color-mode-cookie-year")
            .cookie_enabled(true),
    );
    TimeoutFuture::new(20).await;
    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(20).await;

    let writes = cookie_writes("color-mode-cookie-year");
    assert!(
        writes
            .last()
            .is_some_and(|w| w.contains("Max-Age=31536000")),
        "expected a one year Max-Age in {writes:?}"
    );

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-cookie-max-age")
            .cookie_name("color-mode-cookie-session")
            .cookie_enabled(true)
            .cookie_max_age::<i64>(None),
    );
    TimeoutFuture::new(20).await;
    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(20).await;

    let writes = cookie_writes("color-mode-cookie-session");
    assert!(
        writes.last().is_some_and(|w| !w.contains("Max-Age")),
        "expected a session cookie in {writes:?}"
    );

    remove_cookie("color-mode-cookie-year");
    remove_cookie("color-mode-cookie-session");
}

#[wasm_bindgen_test]
fn parses_and_displays_modes() {
    assert_eq!(ColorMode::from("dark"), ColorMode::Dark);
    assert_eq!(ColorMode::from("light"), ColorMode::Light);
    assert_eq!(ColorMode::from(""), ColorMode::Auto);
    assert_eq!(
        ColorMode::from("sepia"),
        ColorMode::Custom("sepia".to_string())
    );
    assert_eq!(ColorMode::Dark.to_string(), "dark");
}
