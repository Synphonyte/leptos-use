use crate::{IntoElementMaybeSignal, MaybeRwSignal, StorageType, get_header};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

/// Color modes
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum ColorMode {
    #[default]
    Auto,
    Light,
    Dark,
    Custom(String),
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ColorMode::*;

        match self {
            Auto => write!(f, "auto"),
            Light => write!(f, "light"),
            Dark => write!(f, "dark"),
            Custom(v) => write!(f, "{v}"),
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
{
    /// Element that the color mode will be applied to. Defaults to the root element of the
    /// document (`<html>`).
    pub(crate) target: El,

    /// HTML attribute applied to the target element. Defaults to `"class"`.
    #[builder(into)]
    pub(crate) attribute: String,

    /// Initial value of the color mode. Defaults to `"Auto"`.
    #[builder(into)]
    pub(crate) initial_value: MaybeRwSignal<ColorMode>,

    /// Discover the initial value of the color mode from an URL parameter. Defaults to `None`.
    #[builder(into)]
    pub(crate) initial_value_from_url_param: Option<String>,

    /// Write the initial value of the discovered color mode from URL parameter to storage.
    /// This only has an effect if `initial_value_from_url_param` is specified.
    /// Defaults to `false`.
    pub(crate) initial_value_from_url_param_to_storage: bool,

    /// Custom modes that you plan to use as `ColorMode::Custom(x)`. Defaults to `vec![]`.
    pub(crate) custom_modes: Vec<String>,

    /// Custom handler that is called on updates.
    /// If specified this will override the default behavior.
    /// To get the default behaviour back you can call the provided `default_handler` function.
    /// It takes two parameters:
    ///     - `mode: ColorMode`: The color mode to change to.
    ///     -`default_handler: Arc<dyn Fn(ColorMode)>`: The default handler that would have been called if the `on_changed` handler had not been specified.
    pub(crate) on_changed: OnChangedFn,

    /// When provided, `useStorage` will be skipped.
    /// Defaults to `None`.
    #[builder(into)]
    pub(crate) storage_signal: Option<RwSignal<ColorMode>>,

    /// Key to persist the data into localStorage/sessionStorage.
    /// Defaults to `"leptos-use-color-scheme"`.
    #[builder(into)]
    pub(crate) storage_key: String,

    /// Storage type, can be `Local` or `Session` or custom.
    /// Defaults to `Local`.
    pub(crate) storage: StorageType,

    /// If the color mode should be persisted.
    /// Defaults to `true`.
    pub(crate) storage_enabled: bool,

    /// Name of the cookie that should be used to persist the color mode.
    /// Defaults to `"leptos-use-color-scheme"`.
    #[builder(into)]
    pub(crate) cookie_name: String,

    /// If the color mode should be persisted through a cookie.
    /// Defaults to `false`.
    pub(crate) cookie_enabled: bool,

    /// Emit `auto` mode from state
    ///
    /// When set to `true`, preferred mode won't be translated into `light` or `dark`.
    /// This is useful when the fact that `auto` mode was selected needs to be known.
    ///
    /// Defaults to `false`.
    pub(crate) emit_auto: bool,

    /// If transitions on color mode change are enabled. Defaults to `false`.
    pub(crate) transition_enabled: bool,

    /// Listen to changes to this storage key from somewhere else.
    /// Defaults to true.
    pub(crate) listen_to_storage_changes: bool,

    /// Getter function to return the string value of the
    /// [`Sec-CH-Prefers-Color-Scheme`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
    /// header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default
    /// implementation provided.
    #[allow(dead_code)]
    pub(crate) ssr_color_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,

    #[builder(skip)]
    pub(crate) _marker: PhantomData<M>,
}

type OnChangedFn = Arc<dyn Fn(ColorMode, Arc<dyn Fn(ColorMode) + Send + Sync>) + Send + Sync>;

impl<M> Default for UseColorModeOptions<Option<web_sys::Element>, M>
where
    Option<web_sys::Element>: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            target: document_element(),
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

/// The `<html>` element. Unlike the selector `"html"` this doesn't need the code that looks up
/// elements by selector and waits for them to appear.
fn document_element() -> Option<web_sys::Element> {
    #[cfg(feature = "ssr")]
    {
        None
    }

    #[cfg(not(feature = "ssr"))]
    {
        document().document_element()
    }
}
