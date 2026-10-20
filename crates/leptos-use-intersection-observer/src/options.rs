use crate::IntoElementMaybeSignal;
use default_struct_builder::DefaultBuilder;
use std::marker::PhantomData;

/// Options for [`use_intersection_observer_with_options`](crate::use_intersection_observer_with_options).
#[derive(DefaultBuilder)]
pub struct UseIntersectionObserverOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    /// If `true`, the `IntersectionObserver` will be attached immediately. Otherwise it
    /// will only be attached after the returned `resume` closure is called. That is
    /// `use_intersections_observer` will be started "paused".
    pub(crate) immediate: bool,

    /// A `web_sys::Element` or `web_sys::Document` object which is an ancestor of the intended `target`,
    /// whose bounding rectangle will be considered the viewport.
    /// Any part of the target not visible in the visible area of the `root` is not considered visible.
    /// Defaults to `None` (which means the root `document` will be used).
    /// Please note that setting this to a `Some(document)` may not be supported by all browsers.
    /// See [Browser Compatibility](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver#browser_compatibility)
    pub(crate) root: Option<El>,

    /// A string which specifies a set of offsets to add to the root's [bounding box](https://developer.mozilla.org/en-US/docs/Glossary/Bounding_box)
    /// when calculating intersections, effectively shrinking or growing the root for calculation purposes. The syntax is approximately the same as that for the CSS
    /// [`margin`](https://developer.mozilla.org/en-US/docs/Web/CSS/margin) property; see
    /// [The intersection root and root margin](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API#the_intersection_root_and_root_margin)
    /// for more information on how the margin works and the syntax. The default is `"0px"`.
    #[builder(into)]
    pub(crate) root_margin: String,

    // TODO : validate that each number is between 0 and 1 ?
    /// A `Vec` of numbers between 0.0 and 1.0, specifying a ratio of intersection area to total
    /// bounding box area for the observed target. A value of 0.0 means that even a single
    /// visible pixel counts as the target being visible. 1.0 means that the entire target
    /// element is visible. See [Thresholds](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API#thresholds)
    /// for a more in-depth description of how thresholds are used.
    /// The default is a single threshold of `[0.0]`.
    pub(crate) thresholds: Vec<f64>,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

impl<M> Default for UseIntersectionObserverOptions<web_sys::Element, M>
where
    web_sys::Element: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            immediate: true,
            root: None,
            root_margin: "0px".into(),
            thresholds: vec![0.0],
            _marker: PhantomData,
        }
    }
}
