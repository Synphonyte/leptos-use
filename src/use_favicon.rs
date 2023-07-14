#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::watch;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use wasm_bindgen::JsCast;

/// Reactive favicon.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_favicon)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_favicon;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let (icon, set_icon) = use_favicon(cx);
///
/// set_icon.set(Some("dark.png".to_string())); // change current icon
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Passing a Source Signal
///
/// You can pass a `Signal` to [`use_favicon_with_options`]. Change from the source signal will be
/// reflected in your favicon automatically.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_favicon_with_options, UseFaviconOptions, use_preferred_dark};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let is_dark = use_preferred_dark(cx);
///
/// let (icon, _) = use_favicon_with_options(
///     cx,
///     UseFaviconOptions::default().new_icon(
///         Signal::derive(cx, move || {
///             Some((if is_dark.get() { "dark.png" } else { "light.png" }).to_string())
///         }),
///     )
/// );
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server only the signals work but no favicon will be changed obviously.
pub fn use_favicon(cx: Scope) -> (ReadSignal<Option<String>>, WriteSignal<Option<String>>) {
    use_favicon_with_options(cx, UseFaviconOptions::default())
}

/// Version of [`use_favicon`] that accepts a `UseFaviconOptions`. See [`use_favicon`] for more details.
pub fn use_favicon_with_options(
    cx: Scope,
    options: UseFaviconOptions,
) -> (ReadSignal<Option<String>>, WriteSignal<Option<String>>) {
    let UseFaviconOptions {
        new_icon,
        base_url,
        rel,
    } = options;

    let (favicon, set_favicon) = create_signal(cx, new_icon.get_untracked());

    if matches!(&new_icon, MaybeSignal::Dynamic(_)) {
        create_effect(cx, move |_| set_favicon.set(new_icon.get()));
    }

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let link_selector = format!("link[rel*=\"{rel}\"]");

        let apply_icon = move |icon: &String| {
            if let Some(head) = document().head() {
                if let Ok(links) = head.query_selector_all(&link_selector) {
                    let href = format!("{base_url}{icon}");

                    for i in 0..links.length() {
                        let node = links.get(i).expect("checked length");
                        let link: web_sys::HtmlLinkElement = node.unchecked_into();
                        link.set_href(&href);
                    }
                }
            }
        };

        let _ = watch(
            cx,
            move || favicon.get(),
            move |new_icon, prev_icon, _| {
                if Some(new_icon) != prev_icon {
                    if let Some(new_icon) = new_icon {
                        apply_icon(new_icon);
                    }
                }
            },
        );
    }}

    (favicon, set_favicon)
}

/// Options for [`use_favicon_with_options`].
#[derive(DefaultBuilder)]
pub struct UseFaviconOptions {
    /// New input favicon. Can be a `Signal` in which case updates will change the favicon. Defaults to None.
    #[builder(into)]
    new_icon: MaybeSignal<Option<String>>,

    /// Base URL of the favicon. Defaults to "".
    #[builder(into)]
    base_url: String,
    /// Rel attribute of the <link> tag. Defaults to "icon".
    #[builder(into)]
    rel: String,
}

impl Default for UseFaviconOptions {
    fn default() -> Self {
        Self {
            new_icon: Default::default(),
            base_url: "".to_string(),
            rel: "icon".to_string(),
        }
    }
}
