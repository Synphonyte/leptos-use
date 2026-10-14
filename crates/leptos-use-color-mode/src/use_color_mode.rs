use crate::{
    ColorMode, IntoElementMaybeSignal, MaybeRwSignal, StorageType, SyncSignalOptions,
    UseColorModeOptions, UseColorModeReturn, UseCookieOptions, UsePreferredDarkOptions,
    UseStorageOptions, sync_signal_with_options, url, use_cookie_with_options,
    use_preferred_dark_with_options, use_storage_with_options,
};
use cfg_if::cfg_if;
use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::str::FromStr;
#[cfg(not(feature = "ssr"))]
use std::sync::Arc;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::ElementMaybeSignal;
    use wasm_bindgen::JsCast;
}}

/// Reactive color mode (dark / light / customs) with auto data persistence.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_color_mode)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_color_mode::{use_color_mode, UseColorModeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseColorModeReturn {
///     mode, // Signal<ColorMode::dark | ColorMode::light>
///     set_mode,
///     ..
/// } = use_color_mode();
/// #
/// # view! { }
/// # }
/// ```
///
/// By default, it will match with users' browser preference using [`fn@crate::use_preferred_dark`] (a.k.a. `ColorMode::Auto`).
/// When reading the signal, it will by default return the current color mode (`ColorMode::Dark`, `ColorMode::Light` or
/// your custom modes `ColorMode::Custom("some-custom")`). The `ColorMode::Auto` variant can
/// be included in the returned modes by enabling the `emit_auto` option and using [`use_color_mode_with_options`].
/// When writing to the signal (`set_mode`), it will trigger DOM updates and persist the color mode to local
/// storage (or your custom storage). You can pass `ColorMode::Auto` to set back to auto mode.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_color_mode::{ColorMode, use_color_mode, UseColorModeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
/// #
/// mode.get(); // ColorMode::Dark or ColorMode::Light
///
/// set_mode.set(ColorMode::Dark); // change to dark mode and persist
///
/// set_mode.set(ColorMode::Auto); // change to auto mode
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Options
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_color_mode::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
///     UseColorModeOptions::default()
///         .attribute("theme") // instead of writing to `class`
///         .custom_modes(vec![
///             // custom colors in addition to light/dark
///             "dim".to_string(),
///             "cafe".to_string(),
///         ]),
/// ); // Signal<ColorMode::Dark | ColorMode::Light | ColorMode::Custom("dim") | ColorMode::Custom("cafe")>
/// #
/// # view! { }
/// # }
/// ```
///
/// ### Cookie
///
/// To persist color mode in a cookie, use `use_cookie_with_options` and specify `.cookie_enabled(true)`.
///
/// > Note: To work with SSR you have to add the `axum` or `actix` feature as described in [`fn@crate::use_cookie`].
///
/// ```rust
/// # use leptos::prelude::*;
/// # use leptos_meta::*;
/// # use leptos_use_color_mode::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
///     UseColorModeOptions::default()
///         .cookie_enabled(true),
/// );
///
/// // This adds the color mode class to the `<html>` element even with SSR
/// view! {
///     <Html {..} class=move || mode.get().to_string()/>
/// }
/// # }
/// ```
///
/// For a working example please check out the [ssr example](https://github.com/Synphonyte/leptos-use/blob/main/examples/ssr/src/app.rs).
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this will try to read the
/// [`Sec-CH-Prefers-Color-Scheme` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
/// to determine the color mode. If the header is not present it will return `ColorMode::Light`.
/// Please have a look at the linked documentation above for that header to see browser support
/// as well as potential server requirements.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum nor Actix or the default implementation is not to your liking,
/// you can provide your own way of reading the color scheme header value using the option
/// [`crate::UseColorModeOptions::ssr_color_header_getter`].
///
/// ### Cookie
///
/// If `cookie_enabled` is set to `true`, a cookie will be used and if present this value will be used
/// on the server as well as on the client. Please note that you have to add the `axum` or `actix`
/// feature as described in [`fn@crate::use_cookie`].
///
/// ## See also
///
/// * [`fn@crate::use_preferred_dark`]
/// * [`fn@crate::use_storage`]
/// * [`fn@crate::use_cookie`]
pub fn use_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(UseColorModeOptions::default())
}

/// Version of [`use_color_mode`] that takes a `UseColorModeOptions`. See [`use_color_mode`] for how to use.
pub fn use_color_mode_with_options<El, M>(options: UseColorModeOptions<El, M>) -> UseColorModeReturn
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    #[allow(unused_variables)]
    let UseColorModeOptions {
        target,
        attribute,
        initial_value,
        initial_value_from_url_param,
        initial_value_from_url_param_to_storage,
        on_changed,
        storage_signal,
        custom_modes,
        storage_key,
        storage,
        storage_enabled,
        cookie_name,
        cookie_enabled,
        emit_auto,
        transition_enabled,
        listen_to_storage_changes,
        ssr_color_header_getter,
        _marker,
    } = options;

    let preferred_dark = use_preferred_dark_with_options(UsePreferredDarkOptions {
        ssr_color_header_getter,
    });

    let system = Signal::derive(move || {
        if preferred_dark.get() {
            ColorMode::Dark
        } else {
            ColorMode::Light
        }
    });

    let mut initial_value_from_url = None;
    if let Some(param) = initial_value_from_url_param.as_ref()
        && let Some(value) = url::params::get(param)
    {
        initial_value_from_url = ColorMode::from_str(&value).map(MaybeRwSignal::Static).ok()
    }

    let (store, set_store) = get_store_signal(
        initial_value_from_url.clone().unwrap_or(initial_value),
        storage_signal,
        &storage_key,
        storage_enabled,
        storage,
        listen_to_storage_changes,
    );

    let (cookie, set_cookie) = get_cookie_signal(&cookie_name, cookie_enabled);

    if cookie_enabled {
        let _ = sync_signal_with_options(
            (cookie, set_cookie),
            (store, set_store),
            SyncSignalOptions::with_assigns(
                move |store: &mut ColorMode, cookie: &Option<ColorMode>| {
                    if let Some(cookie) = cookie {
                        *store = cookie.clone();
                    }
                },
                move |cookie: &mut Option<ColorMode>, store: &ColorMode| {
                    *cookie = Some(store.clone())
                },
            ),
        );
    }

    if let Some(initial_value_from_url) = initial_value_from_url {
        let value = initial_value_from_url.into_signal().0.get_untracked();
        if initial_value_from_url_param_to_storage {
            set_store.set(value);
        } else {
            *set_store.write_untracked() = value;
        }
    }

    let state = Signal::derive(move || {
        let value = store.get();
        if value == ColorMode::Auto {
            system.get()
        } else {
            value
        }
    });

    #[cfg(not(feature = "ssr"))]
    {
        let modes: Vec<String> = custom_modes
            .into_iter()
            .chain(vec![
                ColorMode::Dark.to_string(),
                ColorMode::Light.to_string(),
            ])
            .collect();

        let target = target.into_element_maybe_signal();

        let update_html_attrs = {
            move |target: ElementMaybeSignal<web_sys::Element>,
                  attribute: String,
                  value: ColorMode| {
                let el = target.get();

                if let Some(el) = el {
                    let mut style: Option<web_sys::HtmlStyleElement> = None;

                    if !transition_enabled
                        && let Ok(styl) = document().create_element("style")
                        && let Some(head) = document().head()
                    {
                        let styl: web_sys::HtmlStyleElement = styl.unchecked_into();
                        let style_string = "*,*::before,*::after{-webkit-transition:none!important;-moz-transition:none!important;-o-transition:none!important;-ms-transition:none!important;transition:none!important}";
                        styl.set_text_content(Some(style_string));
                        let _ = head.append_child(&styl);
                        style = Some(styl);
                    }

                    if attribute == "class" {
                        for mode in &modes {
                            if &value.to_string() == mode {
                                let _ = el.class_list().add_1(mode);
                            } else {
                                let _ = el.class_list().remove_1(mode);
                            }
                        }
                    } else {
                        let _ = el.set_attribute(&attribute, &value.to_string());
                    }

                    if !transition_enabled
                        && let Some(style) = style
                        && let Some(head) = document().head()
                    {
                        // Calling getComputedStyle forces the browser to redraw
                        if let Ok(Some(style)) = window().get_computed_style(&style) {
                            let _ = style.get_property_value("opacity");
                        }

                        let _ = head.remove_child(&style);
                    }
                }
            }
        };

        let default_on_changed = move |mode: ColorMode| {
            update_html_attrs(target, attribute.clone(), mode);
        };

        let on_changed = move |mode: ColorMode| {
            on_changed(mode, Arc::new(default_on_changed.clone()));
        };

        Effect::new({
            let on_changed = on_changed.clone();

            move |_| {
                on_changed.clone()(state.get());
            }
        });

        on_cleanup(move || {
            on_changed(state.get());
        });
    }

    let mode = Signal::derive(move || if emit_auto { store.get() } else { state.get() });

    UseColorModeReturn {
        mode,
        set_mode: set_store,
        store,
        set_store,
        system,
        state,
    }
}

fn get_cookie_signal(
    cookie_name: &str,
    cookie_enabled: bool,
) -> (Signal<Option<ColorMode>>, WriteSignal<Option<ColorMode>>) {
    if cookie_enabled {
        use_cookie_with_options::<ColorMode, FromToStringCodec>(
            cookie_name,
            UseCookieOptions::default().path("/"),
        )
    } else {
        let (value, set_value) = signal(None);
        (value.into(), set_value)
    }
}

fn get_store_signal(
    initial_value: MaybeRwSignal<ColorMode>,
    storage_signal: Option<RwSignal<ColorMode>>,
    storage_key: &str,
    storage_enabled: bool,
    storage: StorageType,
    listen_to_storage_changes: bool,
) -> (Signal<ColorMode>, WriteSignal<ColorMode>) {
    if let Some(storage_signal) = storage_signal {
        let (store, set_store) = storage_signal.split();
        (store.into(), set_store)
    } else if storage_enabled {
        let (store, set_store, _) = use_storage_with_options::<ColorMode, FromToStringCodec>(
            storage,
            storage_key,
            UseStorageOptions::default()
                .listen_to_storage_changes(listen_to_storage_changes)
                .initial_value(initial_value),
        );
        (store, set_store)
    } else {
        initial_value.into_signal()
    }
}
