use crate::core::IntoElementMaybeSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::marker::PhantomData;

#[cfg(not(feature = "ssr"))]
use crate::{use_intersection_observer_with_options, UseIntersectionObserverOptions};
use leptos::reactive::wrappers::read::Signal;

/// Tracks the visibility of an element within the viewport.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_visibility)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use::use_element_visibility;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Div>::new();
///
/// let is_visible = use_element_visibility(el);
///
/// view! {
///     <div node_ref=el>
///         <h1>{is_visible}</h1>
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns a `Signal` that always contains the value `false`.
///
/// ## See also
///
/// * [`fn@crate::use_intersection_observer`]
pub fn use_element_visibility<El, M>(target: El) -> Signal<bool>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    use_element_visibility_with_options::<El, M, web_sys::Element, _>(
        target,
        UseElementVisibilityOptions::default(),
    )
}

/// Version of [`use_element_visibility`] with that takes a `UseElementVisibilityOptions`. See [`use_element_visibility`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn use_element_visibility_with_options<El, M, ContainerEl, ContainerM>(
    target: El,
    options: UseElementVisibilityOptions<ContainerEl, ContainerM>,
) -> Signal<bool>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
    ContainerEl: IntoElementMaybeSignal<web_sys::Element, ContainerM>,
{
    let (is_visible, set_visible) = signal(false);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        use_intersection_observer_with_options(
            target.into_element_maybe_signal(),
            move |entries, _| {
                // In some circumstances Chrome passes a first (or only) entry which has a zero bounding client rect
                // and returns `is_intersecting` erroneously as `false`.
                if let Some(entry) = entries.into_iter().find(|entry| {
                    let rect = entry.bounding_client_rect();
                    rect.width() > 0.0 || rect.height() > 0.0
                }) {
                    set_visible.set(entry.is_intersecting());
                }
            },
            UseIntersectionObserverOptions::default().root(options.viewport),
        );
    }}

    is_visible.into()
}

/// Options for [`use_element_visibility_with_options`].
#[derive(DefaultBuilder)]
pub struct UseElementVisibilityOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    /// A `web_sys::Element` or `web_sys::Document` object which is an ancestor of the intended `target`,
    /// whose bounding rectangle will be considered the viewport.
    /// Any part of the target not visible in the visible area of the `root` is not considered visible.
    /// Defaults to `None` (which means the root `document` will be used).
    /// Please note that setting this to a `Some(document)` may not be supported by all browsers.
    /// See [Browser Compatibility](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver#browser_compatibility)
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    viewport: Option<El>,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

impl<M> Default for UseElementVisibilityOptions<web_sys::Element, M>
where
    web_sys::Element: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            viewport: None,
            _marker: PhantomData,
        }
    }
}
