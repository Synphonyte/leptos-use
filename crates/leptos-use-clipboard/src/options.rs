use default_struct_builder::DefaultBuilder;

/// Options for [`use_clipboard_with_options`](crate::use_clipboard_with_options).
#[derive(DefaultBuilder)]
pub struct UseClipboardOptions {
    /// When `true` event handlers are added so that the returned signal `text` is updated whenever the clipboard changes.
    /// Defaults to `false`.
    ///
    /// > Please note that clipboard changes are only detected when copying or cutting text inside the same document.
    pub(crate) read: bool,

    /// After how many milliseconds after copying should the returned signal `copied` be set to `false`?
    /// Defaults to 1500.
    pub(crate) copied_reset_delay: f64,
}

impl Default for UseClipboardOptions {
    fn default() -> Self {
        Self {
            read: false,
            copied_reset_delay: 1500.0,
        }
    }
}
