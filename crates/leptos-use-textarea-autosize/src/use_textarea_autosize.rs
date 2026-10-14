use crate::{IntoElementMaybeSignal, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn};
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use send_wrapper::SendWrapper;

/// Automatically update the height of a textarea depending on the content.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_textarea_autosize)
///
/// ## Usage
///
/// ### Simple example
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Textarea;
/// # use leptos_use_textarea_autosize::{use_textarea_autosize, UseTextareaAutosizeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let textarea = NodeRef::new();
///
/// let UseTextareaAutosizeReturn {
///     content,
///     set_content,
///     trigger_resize
/// } = use_textarea_autosize(textarea);
///
/// view! {
///     <textarea
///         prop:value=content
///         on:input=move |evt| set_content.set(event_target_value(&evt))
///         node_ref=textarea
///         class="resize-none"
///         placeholder="What's on your mind?"
///     />
/// }
/// # }
/// ```
///
/// > Make sure that you set `box-sizing: border-box` on the textarea element.
/// >
/// > It's also recommended to reset the scrollbar styles for the textarea element to avoid
/// > incorrect height values for large amounts of text.
///
/// ```css
/// textarea {
///   -ms-overflow-style: none;
///   scrollbar-width: none;
/// }
///
/// textarea::-webkit-scrollbar {
///   display: none;
/// }
/// ```
///
/// ### With `rows` attribute
///
/// If you need support for the rows attribute on a textarea element, then you should set the
/// `style_prop` option to `"min-height"`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Textarea;
/// # use leptos_use_textarea_autosize::{use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let textarea = NodeRef::new();
///
/// let UseTextareaAutosizeReturn {
///     content,
///     set_content,
///     ..
/// } = use_textarea_autosize_with_options(
///     textarea,
///     UseTextareaAutosizeOptions::default().style_prop("min-height"),
/// );
///
/// view! {
///     <textarea
///         prop:value=content
///         on:input=move |evt| set_content.set(event_target_value(&evt))
///         node_ref=textarea
///         class="resize-none"
///         placeholder="What's on your mind?"
///         rows="3"
///     />
/// }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closure `trigger_resize` is a sendwrapped function. It can
/// only be called from the same thread that called `use_textarea_autosize`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this will always return an empty string as ´content` and a no-op `trigger_resize`.
pub fn use_textarea_autosize<El, M>(
    el: El,
) -> UseTextareaAutosizeReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementMaybeSignal<web_sys::Element, M> + Clone,
{
    use_textarea_autosize_with_options::<El, M>(el, UseTextareaAutosizeOptions::default())
}

/// Version of [`fn@crate::use_textarea_autosize`] that takes a `UseTextareaAutosizeOptions`. See [`fn@crate::use_textarea_autosize`] for how to use.
pub fn use_textarea_autosize_with_options<El, M>(
    el: El,
    options: UseTextareaAutosizeOptions,
) -> UseTextareaAutosizeReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementMaybeSignal<web_sys::Element, M> + Clone,
{
    #[cfg(not(feature = "ssr"))]
    {
        use crate::sendwrap_fn;
        use wasm_bindgen::JsCast;

        let el = el.into_element_maybe_signal();
        let textarea = Signal::derive(move || {
            el.get().map(|el| {
                SendWrapper::new(el.take().unchecked_into::<web_sys::HtmlTextAreaElement>())
            })
        });

        let UseTextareaAutosizeOptions {
            content,
            watch: watch_fn,
            on_resize,
            style_target,
            style_prop,
        } = options;

        let (content, set_content) = content.into_signal();

        let (textarea_scroll_height, set_textarea_scroll_height) = signal(1);
        let (textarea_old_width, set_textarea_old_width) = signal(0.0);

        let trigger_resize = sendwrap_fn!(move || {
            textarea.with_untracked(|textarea| {
                if let Some(textarea) = textarea {
                    let mut height = "".to_string();

                    let border_offset =
                        if let Ok(Some(style)) = window().get_computed_style(textarea) {
                            (parse_num(
                                &style
                                    .get_property_value("border-top-width")
                                    .unwrap_or_default(),
                            ) + parse_num(
                                &style
                                    .get_property_value("border-bottom-width")
                                    .unwrap_or_default(),
                            )) as i32
                        } else {
                            0
                        };

                    web_sys::HtmlElement::style(textarea)
                        .set_property(&style_prop, "1px")
                        .ok();
                    set_textarea_scroll_height.set(textarea.scroll_height() + border_offset + 1);

                    if let Some(style_target) = style_target.get() {
                        // If style target is provided update its height
                        style_target
                            .take()
                            .unchecked_into::<web_sys::HtmlElement>()
                            .style()
                            .set_property(
                                &style_prop,
                                &format!("{}px", textarea_scroll_height.get_untracked()),
                            )
                            .ok();
                    } else {
                        // else update textarea's height by updating height variable
                        height = format!("{}px", textarea_scroll_height.get_untracked());
                    }

                    web_sys::HtmlElement::style(textarea)
                        .set_property(&style_prop, &height)
                        .ok();
                }
            })
        });

        Effect::watch(
            move || {
                content.with(|_| ());
                textarea.with(|_| ());
            },
            {
                let trigger_resize = trigger_resize.clone();

                move |_, _, _| {
                    trigger_resize();
                }
            },
            true,
        );

        Effect::watch(
            move || textarea_scroll_height.track(),
            move |_, _, _| {
                on_resize();
            },
            false,
        );

        crate::use_resize_observer(textarea, {
            let trigger_resize = trigger_resize.clone();

            move |entries, _| {
                for entry in entries {
                    let width = entry.content_rect().width();

                    if width != textarea_old_width.get_untracked() {
                        set_textarea_old_width.set(width);
                        trigger_resize();
                    }
                }
            }
        });

        Effect::watch(
            move || watch_fn(),
            {
                let trigger_resize = trigger_resize.clone();

                move |_, _, _| {
                    trigger_resize();
                }
            },
            false,
        );

        UseTextareaAutosizeReturn {
            content,
            set_content,
            trigger_resize,
        }
    }

    #[cfg(feature = "ssr")]
    {
        let _ = el;
        let _ = options;

        let (content, set_content) = signal("".to_string());

        UseTextareaAutosizeReturn {
            content: content.into(),
            set_content,
            trigger_resize: || {},
        }
    }
}

#[cfg(not(feature = "ssr"))]
fn parse_num(s: &str) -> u32 {
    s.chars()
        .map_while(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit)
}
