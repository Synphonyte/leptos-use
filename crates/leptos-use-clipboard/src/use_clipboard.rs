use crate::{
    UseClipboardOptions, UseClipboardReturn, UseEventListenerOptions, UseTimeoutFnReturn, js,
    js_fut, sendwrap_fn, use_event_listener_with_options, use_supported,
};
use leptos::ev::{copy, cut};
use leptos::prelude::*;

/// Reactive [Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API).
///
/// Provides the ability to respond to clipboard commands (cut, copy, and paste)
/// as well as to asynchronously read from and write to the system clipboard.
/// Access to the contents of the clipboard is gated behind the
/// [Permissions API](https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API).
/// Without user permission, reading or altering the clipboard contents is not permitted.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_clipboard)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_clipboard::{use_clipboard, UseClipboardReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseClipboardReturn { is_supported, text, copied, copy } = use_clipboard();
///
/// view! {
///     <Show
///         when=move || is_supported.get()
///         fallback=move || view! { <p>Your browser does not support Clipboard API</p> }
///     >
///         <button on:click={
///             let copy = copy.clone();
///             move |_| copy("Hello!")
///         }>
///             <Show when=move || copied.get() fallback=move || "Copy">
///                 "Copied!"
///             </Show>
///         </button>
///     </Show>
/// }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `copy` is a sendwrapped function. It can
/// only be called from the same thread that called `use_clipboard`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the returnd `text` signal will always be `None` and `copy` is a no-op.
pub fn use_clipboard() -> UseClipboardReturn<impl Fn(&str) + Clone + Send + Sync> {
    use_clipboard_with_options(UseClipboardOptions::default())
}

/// Version of [`use_clipboard`] that takes a `UseClipboardOptions`. See [`use_clipboard`] for how to use.
pub fn use_clipboard_with_options(
    options: UseClipboardOptions,
) -> UseClipboardReturn<impl Fn(&str) + Clone + Send + Sync> {
    let UseClipboardOptions {
        copied_reset_delay,
        read,
    } = options;

    let is_supported = use_supported(|| {
        js!("clipboard" in &window()
            .navigator())
    });

    let (text, set_text) = signal(None);
    let (copied, set_copied) = signal(false);

    let UseTimeoutFnReturn { start, .. } = crate::use_timeout_fn(
        move |_: ()| {
            set_copied.set(false);
        },
        copied_reset_delay,
    );

    let update_text = move |_| {
        if is_supported.get() {
            leptos::task::spawn_local(async move {
                let clipboard = window().navigator().clipboard();
                if let Ok(text) = js_fut!(clipboard.read_text()).await {
                    set_text.set(text.as_string());
                }
            })
        }
    };

    // `is_supported` is only updated in an effect, so it is still `false` here. `update_text`
    // checks it when an event arrives.
    if read && !cfg!(feature = "ssr") {
        let listener_options = UseEventListenerOptions::default().passive(true);
        let _ = use_event_listener_with_options(window(), copy, update_text, listener_options);
        let _ = use_event_listener_with_options(window(), cut, update_text, listener_options);
    }

    let do_copy = {
        let start = start.clone();

        sendwrap_fn!(move |value: &str| {
            if is_supported.get() {
                let start = start.clone();
                let value = value.to_owned();

                leptos::task::spawn_local(async move {
                    let clipboard = window().navigator().clipboard();
                    if js_fut!(clipboard.write_text(&value)).await.is_ok() {
                        set_text.set(Some(value));
                        set_copied.set(true);
                        start(());
                    }
                });
            }
        })
    };

    UseClipboardReturn {
        is_supported,
        text: text.into(),
        copied: copied.into(),
        copy: do_copy,
    }
}
