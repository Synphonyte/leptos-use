use crate::{js, js_fut, sendwrap_fn, use_event_listener, use_supported, UseTimeoutFnReturn};
use default_struct_builder::DefaultBuilder;
use leptos::ev::{copy, cut};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

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
/// # use leptos_use::{use_clipboard, UseClipboardReturn};
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

    let UseTimeoutFnReturn { start, .. } = crate::use_timeout_fn::use_timeout_fn(
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

    if is_supported.get() && read {
        let _ = use_event_listener(window(), copy, update_text);
        let _ = use_event_listener(window(), cut, update_text);
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

/// Options for [`use_clipboard_with_options`].
#[derive(DefaultBuilder)]
pub struct UseClipboardOptions {
    /// When `true` event handlers are added so that the returned signal `text` is updated whenever the clipboard changes.
    /// Defaults to `false`.
    ///
    /// > Please note that clipboard changes are only detected when copying or cutting text inside the same document.
    read: bool,

    /// After how many milliseconds after copying should the returned signal `copied` be set to `false`?
    /// Defaults to 1500.
    copied_reset_delay: f64,
}

impl Default for UseClipboardOptions {
    fn default() -> Self {
        Self {
            read: false,
            copied_reset_delay: 1500.0,
        }
    }
}

/// Return type of [`use_clipboard`].
pub struct UseClipboardReturn<CopyFn>
where
    CopyFn: Fn(&str) + Clone,
{
    /// Whether the Clipboard API is supported.
    pub is_supported: Signal<bool>,

    /// The current state of the clipboard.
    pub text: Signal<Option<String>>,

    /// `true` for [`UseClipboardOptions::copied_reset_delay`] milliseconds after copying.
    pub copied: Signal<bool>,

    /// Copy the given text to the clipboard.
    pub copy: CopyFn,
}
