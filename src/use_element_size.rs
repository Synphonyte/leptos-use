use crate::core::{ElementMaybeSignal, Size};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive_graph::wrappers::read::Signal;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::{use_resize_observer_with_options, UseResizeObserverOptions};
    use crate::{watch_with_options, WatchOptions};
    use wasm_bindgen::JsCast;
}}

/// Reactive size of an HTML element.
///
/// Please refer to [ResizeObserver on MDN](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
/// for more details.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_size)
///
/// ## Usage
///
/// ```
/// # use leptos::{html::Div, prelude::*};
/// # use leptos_use::{use_element_size, UseElementSizeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Div>::new();
///
/// let UseElementSizeReturn { width, height } = use_element_size(el);
///
/// view! {
///     <div node_ref=el>
///         "Width: " {width}
///         "Height: " {height}
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server the returned signals always contain the value of the `initial_size` option.
///
/// ## See also
///
/// - [`fn@crate::use_resize_observer`]
pub fn use_element_size<El, T>(target: El) -> UseElementSizeReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
{
    use_element_size_with_options(target, UseElementSizeOptions::default())
}

/// Version of [`use_element_size`] that takes a `UseElementSizeOptions`. See [`use_element_size`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn use_element_size_with_options<El, T>(
    target: El,
    options: UseElementSizeOptions,
) -> UseElementSizeReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let UseElementSizeOptions { box_, initial_size } = options;

    let (width, set_width) = signal(initial_size.width);
    let (height, set_height) = signal(initial_size.height);

    #[cfg(not(feature = "ssr"))]
    {
        let box_ = box_.unwrap_or(web_sys::ResizeObserverBoxOptions::ContentBox);

        let target = target.into();

        let is_svg = {
            let target = target.clone();

            move || {
                if let Some(target) = target.get_untracked() {
                    target
                        .into()
                        .namespace_uri()
                        .map(|ns| ns.contains("svg"))
                        .unwrap_or(false)
                } else {
                    false
                }
            }
        };

        {
            let target = target.clone();

            let _ = use_resize_observer_with_options::<ElementMaybeSignal<T, web_sys::Element>, _, _>(
                target.clone(),
                move |entries, _| {
                    let entry = &entries[0];

                    let box_size = match box_ {
                        web_sys::ResizeObserverBoxOptions::ContentBox => entry.content_box_size(),
                        web_sys::ResizeObserverBoxOptions::BorderBox => entry.border_box_size(),
                        web_sys::ResizeObserverBoxOptions::DevicePixelContentBox => {
                            entry.device_pixel_content_box_size()
                        }
                        _ => unreachable!(),
                    };

                    if is_svg() {
                        if let Some(target) = target.get() {
                            if let Ok(Some(styles)) = window().get_computed_style(&target.into()) {
                                set_height.set(
                                    styles
                                        .get_property_value("height")
                                        .map(|v| v.parse().unwrap_or_default())
                                        .unwrap_or_default(),
                                );
                                set_width.set(
                                    styles
                                        .get_property_value("width")
                                        .map(|v| v.parse().unwrap_or_default())
                                        .unwrap_or_default(),
                                );
                            }
                        }
                    } else if !box_size.is_null()
                        && !box_size.is_undefined()
                        && box_size.length() > 0
                    {
                        let format_box_size = if box_size.is_array() {
                            box_size.to_vec()
                        } else {
                            vec![box_size.into()]
                        };

                        set_width.set(format_box_size.iter().fold(0.0, |acc, v| {
                            acc + v
                                .as_ref()
                                .clone()
                                .unchecked_into::<web_sys::ResizeObserverSize>()
                                .inline_size()
                        }));
                        set_height.set(format_box_size.iter().fold(0.0, |acc, v| {
                            acc + v
                                .as_ref()
                                .clone()
                                .unchecked_into::<web_sys::ResizeObserverSize>()
                                .block_size()
                        }))
                    } else {
                        // fallback
                        set_width.set(entry.content_rect().width());
                        set_height.set(entry.content_rect().height())
                    }
                },
                UseResizeObserverOptions::default().box_(box_),
            );
        }

        let _ = watch_with_options(
            move || target.get(),
            move |ele, _, _| {
                if ele.is_some() {
                    set_width.set(initial_size.width);
                    set_height.set(initial_size.height);
                } else {
                    set_width.set(0.0);
                    set_height.set(0.0);
                }
            },
            WatchOptions::default().immediate(false),
        );
    }

    UseElementSizeReturn {
        width: width.into(),
        height: height.into(),
    }
}

#[derive(DefaultBuilder, Default)]
/// Options for [`use_element_size_with_options`].
pub struct UseElementSizeOptions {
    /// Initial size returned before any measurements on the `target` are done. Also the value reported
    /// at first when the `target` is a signal and changes.
    initial_size: Size,

    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    #[builder(into)]
    pub box_: Option<web_sys::ResizeObserverBoxOptions>,
}

/// The return value of [`use_element_size`].
pub struct UseElementSizeReturn {
    /// The width of the element.
    pub width: Signal<f64>,
    /// The height of the element.
    pub height: Signal<f64>,
}
