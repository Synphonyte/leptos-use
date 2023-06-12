use crate::core::ElementMaybeSignal;
use crate::{use_intersection_observer_with_options, UseIntersectionObserverOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::marker::PhantomData;

/// Tracks the visibility of an element within the viewport.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_visibility)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_element_visibility;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let el = create_node_ref(cx);
///
/// let is_visible = use_element_visibility(cx, el);
///
/// view! { cx,
///     <div node_ref=el>
///         <h1>{is_visible}</h1>
///     </div>
/// }
/// # }
/// ```
///
/// ## See also
///
/// * [`use_intersection_observer`]
pub fn use_element_visibility<El, T>(cx: Scope, target: El) -> Signal<bool>
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    use_element_visibility_with_options::<El, T, web_sys::Element, web_sys::Element>(
        cx,
        target,
        UseElementVisibilityOptions::default(),
    )
}

pub fn use_element_visibility_with_options<El, T, ContainerEl, ContainerT>(
    cx: Scope,
    target: El,
    options: UseElementVisibilityOptions<ContainerEl, ContainerT>,
) -> Signal<bool>
where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    (Scope, ContainerEl): Into<ElementMaybeSignal<ContainerT, web_sys::Element>>,
    ContainerT: Into<web_sys::Element> + Clone + 'static,
{
    let (is_visible, set_visible) = create_signal(cx, false);

    use_intersection_observer_with_options(
        cx,
        (cx, target).into(),
        move |entries, _| {
            set_visible(entries[0].is_intersecting());
        },
        UseIntersectionObserverOptions::default().root(options.viewport),
    );

    is_visible.into()
}

/// Options for [`use_element_visibility_with_options`].
#[derive(DefaultBuilder)]
pub struct UseElementVisibilityOptions<El, T>
where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    /// A `web_sys::Element` or `web_sys::Document` object which is an ancestor of the intended `target`,
    /// whose bounding rectangle will be considered the viewport.
    /// Any part of the target not visible in the visible area of the `root` is not considered visible.
    /// Defaults to `None` (which means the root `document` will be used).
    /// Please note that setting this to a `Some(document)` may not be supported by all browsers.
    /// See [Browser Compatibility](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver#browser_compatibility)
    viewport: Option<El>,

    #[builder(skip)]
    _marker: PhantomData<T>,
}

impl Default for UseElementVisibilityOptions<web_sys::Element, web_sys::Element> {
    fn default() -> Self {
        Self {
            viewport: None,
            _marker: PhantomData,
        }
    }
}
