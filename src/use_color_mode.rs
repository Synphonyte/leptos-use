use crate::core::url;
use crate::core::{ElementMaybeSignal, IntoElementMaybeSignal, MaybeRwSignal};
use crate::storage::{use_storage_with_options, StorageType, UseStorageOptions};
use crate::utils::get_header;
use crate::{
    sync_signal_with_options, use_cookie, use_preferred_dark_with_options, SyncSignalOptions,
    UsePreferredDarkOptions,
};
use codee::string::FromToStringCodec;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use wasm_bindgen::JsCast;

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
/// # use leptos_use::{use_color_mode, UseColorModeReturn};
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
/// # use leptos_use::{ColorMode, use_color_mode, UseColorModeReturn};
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
/// # use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
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
/// # use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
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
/// On the server this will try to read the
/// [`Sec-CH-Prefers-Color-Scheme` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
/// to determine the color mode. If the header is not present it will return `ColorMode::Light`.
/// Please have a look at the linked documentation above for that header to see browser support
/// as well as potential server requirements.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`, for `spin` enable `"spin"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum, Actix nor Spin, or the default implementation is not to your liking,
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
/// * [`fn@crate::storage::use_storage`]
/// * [`fn@crate::use_cookie`]
pub fn use_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(UseColorModeOptions::default())
}

/// Version of [`use_color_mode`] that takes a `UseColorModeOptions`. See [`use_color_mode`] for how to use.
pub fn use_color_mode_with_options<El, M>(options: UseColorModeOptions<El, M>) -> UseColorModeReturn
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
    M: ?Sized,
{
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

    let modes: Vec<String> = custom_modes
        .into_iter()
        .chain(vec![
            ColorMode::Dark.to_string(),
            ColorMode::Light.to_string(),
        ])
        .collect();

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
    if let Some(param) = initial_value_from_url_param.as_ref() {
        if let Some(value) = url::params::get(param) {
            initial_value_from_url = ColorMode::from_str(&value).map(MaybeRwSignal::Static).ok()
        }
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

    let target = target.into_element_maybe_signal();

    let update_html_attrs = {
        move |target: ElementMaybeSignal<web_sys::Element>, attribute: String, value: ColorMode| {
            let el = target.get_untracked();

            if let Some(el) = el {
                let mut style: Option<web_sys::HtmlStyleElement> = None;
                if !transition_enabled {
                    if let Ok(styl) = document().create_element("style") {
                        if let Some(head) = document().head() {
                            let styl: web_sys::HtmlStyleElement = styl.unchecked_into();
                            let style_string = "*,*::before,*::after{-webkit-transition:none!important;-moz-transition:none!important;-o-transition:none!important;-ms-transition:none!important;transition:none!important}";
                            styl.set_text_content(Some(style_string));
                            let _ = head.append_child(&styl);
                            style = Some(styl);
                        }
                    }
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

                if !transition_enabled {
                    if let Some(style) = style {
                        if let Some(head) = document().head() {
                            // Calling getComputedStyle forces the browser to redraw
                            if let Ok(Some(style)) = window().get_computed_style(&style) {
                                let _ = style.get_property_value("opacity");
                            }

                            let _ = head.remove_child(&style);
                        }
                    }
                }
            }
        }
    };

    let default_on_changed = move |mode: ColorMode| {
        update_html_attrs(target.clone(), attribute.clone(), mode);
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

/// Color modes
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum ColorMode {
    #[default]
    Auto,
    Light,
    Dark,
    Custom(String),
}

fn get_cookie_signal(
    cookie_name: &str,
    cookie_enabled: bool,
) -> (Signal<Option<ColorMode>>, WriteSignal<Option<ColorMode>>) {
    if cookie_enabled {
        use_cookie::<ColorMode, FromToStringCodec>(cookie_name)
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

impl Display for ColorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ColorMode::*;

        match self {
            Auto => write!(f, "auto"),
            Light => write!(f, "light"),
            Dark => write!(f, "dark"),
            Custom(v) => write!(f, "{}", v),
        }
    }
}

impl From<&str> for ColorMode {
    fn from(s: &str) -> Self {
        match s {
            "auto" => ColorMode::Auto,
            "" => ColorMode::Auto,
            "light" => ColorMode::Light,
            "dark" => ColorMode::Dark,
            _ => ColorMode::Custom(s.to_string()),
        }
    }
}

impl From<String> for ColorMode {
    fn from(s: String) -> Self {
        ColorMode::from(s.as_str())
    }
}

impl FromStr for ColorMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ColorMode::from(s))
    }
}

#[derive(DefaultBuilder)]
pub struct UseColorModeOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
    M: ?Sized,
{
    /// Element that the color mode will be applied to. Defaults to `"html"`.
    target: El,

    /// HTML attribute applied to the target element. Defaults to `"class"`.
    #[builder(into)]
    attribute: String,

    /// Initial value of the color mode. Defaults to `"Auto"`.
    #[builder(into)]
    initial_value: MaybeRwSignal<ColorMode>,

    /// Discover the initial value of the color mode from an URL parameter. Defaults to `None`.
    #[builder(into)]
    initial_value_from_url_param: Option<String>,

    /// Write the initial value of the discovered color mode from URL parameter to storage.
    /// This only has an effect if `initial_value_from_url_param` is specified.
    /// Defaults to `false`.
    initial_value_from_url_param_to_storage: bool,

    /// Custom modes that you plan to use as `ColorMode::Custom(x)`. Defaults to `vec![]`.
    custom_modes: Vec<String>,

    /// Custom handler that is called on updates.
    /// If specified this will override the default behavior.
    /// To get the default behaviour back you can call the provided `default_handler` function.
    /// It takes two parameters:
    ///     - `mode: ColorMode`: The color mode to change to.
    ///     -`default_handler: Arc<dyn Fn(ColorMode)>`: The default handler that would have been called if the `on_changed` handler had not been specified.
    on_changed: OnChangedFn,

    /// When provided, `useStorage` will be skipped.
    /// Defaults to `None`.
    #[builder(into)]
    storage_signal: Option<RwSignal<ColorMode>>,

    /// Key to persist the data into localStorage/sessionStorage.
    /// Defaults to `"leptos-use-color-scheme"`.
    #[builder(into)]
    storage_key: String,

    /// Storage type, can be `Local` or `Session` or custom.
    /// Defaults to `Local`.
    storage: StorageType,

    /// If the color mode should be persisted.
    /// Defaults to `true`.
    storage_enabled: bool,

    /// Name of the cookie that should be used to persist the color mode.
    /// Defaults to `"leptos-use-color-scheme"`.
    #[builder(into)]
    cookie_name: String,

    /// If the color mode should be persisted through a cookie.
    /// Defaults to `false`.
    cookie_enabled: bool,

    /// Emit `auto` mode from state
    ///
    /// When set to `true`, preferred mode won't be translated into `light` or `dark`.
    /// This is useful when the fact that `auto` mode was selected needs to be known.
    ///
    /// Defaults to `false`.
    emit_auto: bool,

    /// If transitions on color mode change are enabled. Defaults to `false`.
    transition_enabled: bool,

    /// Listen to changes to this storage key from somewhere else.
    /// Defaults to true.
    listen_to_storage_changes: bool,

    /// Getter function to return the string value of the
    /// [`Sec-CH-Prefers-Color-Scheme`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
    /// header.
    /// When you use one of the features `"axum"`, `"actix"` or `"spin"` there's a valid default
    /// implementation provided.
    #[allow(dead_code)]
    ssr_color_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

type OnChangedFn = Arc<dyn Fn(ColorMode, Arc<dyn Fn(ColorMode) + Send + Sync>) + Send + Sync>;

impl Default for UseColorModeOptions<&'static str, str> {
    fn default() -> Self {
        Self {
            target: "html",
            attribute: "class".into(),
            initial_value: ColorMode::Auto.into(),
            initial_value_from_url_param: None,
            initial_value_from_url_param_to_storage: false,
            custom_modes: vec![],
            on_changed: Arc::new(move |mode, default_handler| (default_handler)(mode)),
            storage_signal: None,
            storage_key: "leptos-use-color-scheme".into(),
            storage: StorageType::default(),
            storage_enabled: true,
            cookie_name: "leptos-use-color-scheme".into(),
            cookie_enabled: false,
            emit_auto: false,
            transition_enabled: false,
            listen_to_storage_changes: true,
            ssr_color_header_getter: Arc::new(move || {
                get_header!(
                    HeaderName::from_static("sec-ch-prefers-color-scheme"),
                    use_color_mode,
                    ssr_color_header_getter
                )
            }),
            _marker: PhantomData,
        }
    }
}

/// Return type of [`use_color_mode`]
pub struct UseColorModeReturn {
    /// Main value signal of the color mode
    pub mode: Signal<ColorMode>,
    /// Main value setter signal of the color mode
    pub set_mode: WriteSignal<ColorMode>,

    /// Direct access to the returned signal of [`fn@crate::storage::use_storage`] if enabled or [`UseColorModeOptions::storage_signal`] if provided
    pub store: Signal<ColorMode>,
    /// Direct write access to the returned signal of [`fn@crate::storage::use_storage`] if enabled or [`UseColorModeOptions::storage_signal`] if provided
    pub set_store: WriteSignal<ColorMode>,

    /// Signal of the system's preferred color mode that you would get from a media query
    pub system: Signal<ColorMode>,

    /// When [`UseColorModeOptions::emit_auto`] is `false` this is the same as `mode`. This will never report `ColorMode::Auto` but always on of the other modes.
    pub state: Signal<ColorMode>,
}
