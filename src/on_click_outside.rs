use crate::core::{ElementMaybeSignal, ElementsMaybeSignal};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::diagnostics::SpecialNonReactiveZone;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use leptos::prelude::*;
    use crate::utils::IS_IOS;
    use crate::{use_event_listener, use_event_listener_with_options, UseEventListenerOptions};
    use leptos::ev::{blur, click, pointerdown};
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::RwLock;
    use std::time::Duration;
    use wasm_bindgen::JsCast;

    static IOS_WORKAROUND: RwLock<bool> = RwLock::new(false);
}}

/// Listen for clicks outside of an element.
/// Useful for modals or dropdowns.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/on_click_outside)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos::html::Div;
/// # use leptos_use::on_click_outside;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let target = create_node_ref::<Div>();
///
/// on_click_outside(target, move |event| { log!("{:?}", event); });
///
/// view! {
///     <div node_ref=target>"Hello World"</div>
///     <div>"Outside element"</div>
/// }
/// # }
/// ```
///
/// > This function uses [Event.composedPath()](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)
/// which is **not** supported by IE 11, Edge 18 and below.
/// If you are targeting these browsers, we recommend you to include
/// [this code snippet](https://gist.github.com/sibbng/13e83b1dd1b733317ce0130ef07d4efd) on your project.
///
/// ## Excluding Elements
///
/// Use this to ignore clicks on certain elements.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos::html::Div;
/// # use leptos_use::{on_click_outside_with_options, OnClickOutsideOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// # let target = create_node_ref::<Div>();
/// #
/// on_click_outside_with_options(
///     target,
///     move |event| { log!("{:?}", event); },
///     OnClickOutsideOptions::default().ignore(["input", "#some-id"]),
/// );
/// #
/// # view! {
/// #     <div node_ref=target>"Hello World"</div>
/// # }
/// # }
///
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this amounts to a no-op.
pub fn on_click_outside<El, T, F>(target: El, handler: F) -> impl FnOnce() + Clone
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(web_sys::Event) + Clone + 'static,
{
    on_click_outside_with_options::<_, _, _, web_sys::EventTarget>(
        target,
        handler,
        OnClickOutsideOptions::default(),
    )
}

/// Version of `on_click_outside` that takes an `OnClickOutsideOptions`. See `on_click_outside` for more details.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn on_click_outside_with_options<El, T, F, I>(
    target: El,
    handler: F,
    options: OnClickOutsideOptions<I>,
) -> impl FnOnce() + Clone
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(web_sys::Event) + Clone + 'static,
    I: Into<web_sys::EventTarget> + Clone + 'static,
{
    #[cfg(feature = "ssr")]
    {
        || {}
    }

    #[cfg(not(feature = "ssr"))]
    {
        let OnClickOutsideOptions {
            ignore,
            capture,
            detect_iframes,
        } = options;

        // Fixes: https://github.com/vueuse/vueuse/issues/1520
        // How it works: https://stackoverflow.com/a/39712411
        if *IS_IOS {
            if let Ok(mut ios_workaround) = IOS_WORKAROUND.write() {
                if !*ios_workaround {
                    *ios_workaround = true;
                    if let Some(body) = document().body() {
                        let children = body.children();
                        for i in 0..children.length() {
                            let _ = children
                                .get_with_index(i)
                                .expect("checked index")
                                .add_event_listener_with_callback(
                                    "click",
                                    &js_sys::Function::default(),
                                );
                        }
                    }
                }
            }
        }

        let should_listen = Rc::new(Cell::new(true));

        let should_ignore = move |event: &web_sys::UiEvent| {
            let ignore = ignore.get_untracked();

            ignore.into_iter().flatten().any(|element| {
                let element: web_sys::EventTarget = element.into();

                event_target::<web_sys::EventTarget>(event) == element
                    || event.composed_path().includes(element.as_ref(), 0)
            })
        };

        let target = target.into();

        let listener = {
            let should_listen = Rc::clone(&should_listen);
            let mut handler = handler.clone();
            let target = target.clone();
            let should_ignore = should_ignore.clone();

            move |event: web_sys::UiEvent| {
                if let Some(el) = target.get_untracked() {
                    let el = el.into();

                    if el == event_target(&event) || event.composed_path().includes(el.as_ref(), 0)
                    {
                        return;
                    }

                    if event.detail() == 0 {
                        should_listen.set(!should_ignore(&event));
                    }

                    if !should_listen.get() {
                        should_listen.set(true);
                        return;
                    }

                    #[cfg(debug_assertions)]
                    let _z = SpecialNonReactiveZone::enter();

                    handler(event.into());
                }
            }
        };

        let remove_click_listener = {
            let mut listener = listener.clone();

            use_event_listener_with_options::<_, web_sys::Window, _, _>(
                window(),
                click,
                move |event| listener(event.into()),
                UseEventListenerOptions::default()
                    .passive(true)
                    .capture(capture),
            )
        };

        let remove_pointer_listener = {
            let target = target.clone();
            let should_listen = Rc::clone(&should_listen);

            use_event_listener_with_options::<_, web_sys::Window, _, _>(
                window(),
                pointerdown,
                move |event| {
                    if let Some(el) = target.get_untracked() {
                        should_listen.set(
                            !event.composed_path().includes(el.into().as_ref(), 0)
                                && !should_ignore(&event),
                        );
                    }
                },
                UseEventListenerOptions::default().passive(true),
            )
        };

        let remove_blur_listener = if detect_iframes {
            Some(use_event_listener::<_, web_sys::Window, _, _>(
                window(),
                blur,
                move |event| {
                    let target = target.clone();
                    let mut handler = handler.clone();

                    let _ = set_timeout_with_handle(
                        move || {
                            if let Some(el) = target.get_untracked() {
                                if let Some(active_element) = document().active_element() {
                                    if active_element.tag_name() == "IFRAME"
                                        && !el
                                            .into()
                                            .unchecked_into::<web_sys::Node>()
                                            .contains(Some(&active_element.into()))
                                    {
                                        handler(event.into());
                                    }
                                }
                            }
                        },
                        Duration::ZERO,
                    );
                },
            ))
        } else {
            None
        };

        move || {
            remove_click_listener();
            remove_pointer_listener();
            if let Some(f) = remove_blur_listener {
                f();
            }
        }
    }
}

/// Options for [`on_click_outside_with_options`].
#[derive(Clone, DefaultBuilder)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct OnClickOutsideOptions<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    /// List of elementss that should not trigger the callback. Defaults to `[]`.
    #[builder(skip)]
    ignore: ElementsMaybeSignal<T, web_sys::EventTarget>,

    /// Use capturing phase for internal event listener. Defaults to `true`.
    capture: bool,

    /// Run callback if focus moves to an iframe. Defaults to `false`.
    detect_iframes: bool,
}

impl<T> Default for OnClickOutsideOptions<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            capture: true,
            detect_iframes: false,
        }
    }
}

impl<T> OnClickOutsideOptions<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    /// List of elementss that should not trigger the callback. Defaults to `[]`.
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub fn ignore(self, ignore: impl Into<ElementsMaybeSignal<T, web_sys::EventTarget>>) -> Self {
        Self {
            ignore: ignore.into(),
            ..self
        }
    }
}
