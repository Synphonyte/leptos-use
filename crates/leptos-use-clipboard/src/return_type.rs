use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_clipboard`](crate::use_clipboard).
pub struct UseClipboardReturn<CopyFn>
where
    CopyFn: Fn(&str) + Clone,
{
    /// Whether the Clipboard API is supported.
    pub is_supported: Signal<bool>,

    /// The current state of the clipboard.
    pub text: Signal<Option<String>>,

    /// `true` for [`UseClipboardOptions::copied_reset_delay`](crate::UseClipboardOptions::copied_reset_delay) milliseconds after copying.
    pub copied: Signal<bool>,

    /// Copy the given text to the clipboard.
    pub copy: CopyFn,
}
