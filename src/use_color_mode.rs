use crate::core::{ElementMaybeSignal, MaybeRwSignal};
#[cfg(feature = "storage")]
use crate::storage::{use_storage_with_options, UseStorageOptions};
#[cfg(feature = "storage")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::core::StorageType;
use crate::use_preferred_dark;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::marker::PhantomData;
use std::rc::Rc;
use wasm_bindgen::JsCast;

/// Reactive color mode (dark / light / customs) with auto data persistence.
///
/// > Data persistence is only enabled when the crate feature **`storage`** is enabled. You
/// can use the function without it but the mode won't be persisted.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_color_mode)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// use leptos_use::{use_color_mode, UseColorModeReturn};
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
/// By default, it will match with users' browser preference using [`use_preferred_dark`] (a.k.a. `ColorMode::Auto`).
/// When reading the signal, it will by default return the current color mode (`ColorMode::Dark`, `ColorMode::Light` or
/// your custom modes `ColorMode::Custom("some-custom")`). The `ColorMode::Auto` variant can
/// be included in the returned modes by enabling the `emit_auto` option and using [`use_color_mode_with_options`].
/// When writing to the signal (`set_mode`), it will trigger DOM updates and persist the color mode to local
/// storage (or your custom storage). You can pass `ColorMode::Auto` to set back to auto mode.
///
/// ```
/// # use leptos::*;
/// use leptos_use::{ColorMode, use_color_mode, UseColorModeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
/// #
/// mode.get(); // ColorMode::Dark or ColorMode::Light
///
/// set_mode.set(ColorMode::Dark); // change to dark mode and persist (with feature `storage`)
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
/// # use leptos::*;
/// use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
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
/// ## Server-Side Rendering
///
/// On the server this will by default return `ColorMode::Light`. Persistence is disabled, of course.
///
/// ## See also
///
/// * [`use_dark`]
/// * [`use_preferred_dark`]
/// * [`use_storage`]
pub fn use_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(UseColorModeOptions::default())
}

/// Version of [`use_color_mode`] that takes a `UseColorModeOptions`. See [`use_color_mode`] for how to use.
pub fn use_color_mode_with_options<El, T>(options: UseColorModeOptions<El, T>) -> UseColorModeReturn
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let UseColorModeOptions {
        target,
        attribute,
        initial_value,
        on_changed,
        storage_signal,
        custom_modes,
        storage_key,
        storage,
        storage_enabled,
        emit_auto,
        transition_enabled,
        listen_to_storage_changes,
        _marker,
    } = options;

    let modes: Vec<String> = custom_modes
        .into_iter()
        .chain(vec![
            ColorMode::Dark.to_string(),
            ColorMode::Light.to_string(),
        ])
        .collect();

    let preferred_dark = use_preferred_dark();

    let system = Signal::derive(move || {
        if preferred_dark.get() {
            ColorMode::Dark
        } else {
            ColorMode::Light
        }
    });

    let (store, set_store) = get_store_signal(
        initial_value,
        storage_signal,
        &storage_key,
        storage_enabled,
        storage,
        listen_to_storage_changes,
    );

    let state = Signal::derive(move || {
        let value = store.get();
        if value == ColorMode::Auto {
            system.get()
        } else {
            value
        }
    });

    let target = (target).into();

    let update_html_attrs = {
        move |target: ElementMaybeSignal<T, web_sys::Element>,
              attribute: String,
              value: ColorMode| {
            let el = target.get_untracked();

            if let Some(el) = el {
                let el = el.into();

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
        on_changed(mode, Rc::new(default_on_changed.clone()));
    };

    create_effect({
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
#[cfg_attr(feature = "storage", derive(Serialize, Deserialize))]
pub enum ColorMode {
    #[default]
    Auto,
    Light,
    Dark,
    Custom(String),
}

cfg_if! { if #[cfg(feature = "storage")] {
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
            let (store, set_store, _) = use_storage_with_options(
                storage_key,
                initial_value,
                UseStorageOptions::default()
                    .listen_to_storage_changes(listen_to_storage_changes)
                    .storage_type(storage),
            );

            (store, set_store)
        } else {
            initial_value.into_signal()
        }
    }
} else {
    fn get_store_signal(
        initial_value: MaybeRwSignal<ColorMode>,
        storage_signal: Option<RwSignal<ColorMode>>,
        _storage_key: &String,
        _storage_enabled: bool,
        _storage: StorageType,
        _listen_to_storage_changes: bool,
    ) -> (Signal<ColorMode>, WriteSignal<ColorMode>) {
        if let Some(storage_signal) = storage_signal {
            let (store, set_store) = storage_signal.split();
            (store.into(), set_store)
        } else {
            initial_value.into_signal()
        }
    }
}}

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

#[derive(DefaultBuilder)]
pub struct UseColorModeOptions<El, T>
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    /// Element that the color mode will be applied to. Defaults to `"html"`.
    target: El,

    /// HTML attribute applied to the target element. Defaults to `"class"`.
    #[builder(into)]
    attribute: String,

    /// Initial value of the color mode. Defaults to `"Auto"`.
    #[builder(into)]
    initial_value: MaybeRwSignal<ColorMode>,

    /// Custom modes that you plan to use as `ColorMode::Custom(x)`. Defaults to `vec![]`.
    custom_modes: Vec<String>,

    /// Custom handler that is called on updates.
    /// If specified this will override the default behavior.
    /// To get the default behaviour back you can call the provided `default_handler` function.
    /// It takes two parameters:
    /// - `mode: ColorMode`: The color mode to change to.
    ///  -`default_handler: Rc<dyn Fn(ColorMode)>`: The default handler that would have been called if the `on_changed` handler had not been specified.
    on_changed: OnChangedFn,

    /// When provided, `useStorage` will be skipped.
    /// Storage requires the *create feature* **`storage`** to be enabled.
    /// Defaults to `None`.
    #[builder(into)]
    storage_signal: Option<RwSignal<ColorMode>>,

    /// Key to persist the data into localStorage/sessionStorage.
    /// Storage requires the *create feature* **`storage`** to be enabled.
    /// Defaults to `"leptos-use-color-scheme"`.
    #[builder(into)]
    storage_key: String,

    /// Storage type, can be `Local` or `Session` or custom.
    /// Storage requires the *create feature* **`storage`** to be enabled.
    /// Defaults to `Local`.
    storage: StorageType,

    /// If the color mode should be persisted. If `true` this required the
    /// *create feature* **`storage`** to be enabled.
    /// Defaults to `true` and is forced to `false` if the feature **`storage`** is not enabled.
    storage_enabled: bool,

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
    /// Storage requires the *create feature* **`storage`** to be enabled.
    /// Defaults to true.
    listen_to_storage_changes: bool,

    #[builder(skip)]
    _marker: PhantomData<T>,
}

type OnChangedFn = Rc<dyn Fn(ColorMode, Rc<dyn Fn(ColorMode)>)>;

impl Default for UseColorModeOptions<&'static str, web_sys::Element> {
    fn default() -> Self {
        Self {
            target: "html",
            attribute: "class".into(),
            initial_value: ColorMode::Auto.into(),
            custom_modes: vec![],
            on_changed: Rc::new(move |mode, default_handler| (default_handler)(mode)),
            storage_signal: None,
            storage_key: "leptos-use-color-scheme".into(),
            storage: StorageType::default(),
            storage_enabled: true,
            emit_auto: false,
            transition_enabled: false,
            listen_to_storage_changes: true,
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

    /// Direct access to the returned signal of [`use_storage`] if enabled or [`UseColorModeOptions::storage_signal`] if provided
    pub store: Signal<ColorMode>,
    /// Direct write access to the returned signal of [`use_storage`] if enabled or [`UseColorModeOptions::storage_signal`] if provided
    pub set_store: WriteSignal<ColorMode>,

    /// Signal of the system's preferred color mode that you would get from a media query
    pub system: Signal<ColorMode>,

    /// When [`UseColorModeOptions::emit_auto`] is `false` this is the same as `mode`. This will never report `ColorMode::Auto` but always on of the other modes.
    pub state: Signal<ColorMode>,
}
