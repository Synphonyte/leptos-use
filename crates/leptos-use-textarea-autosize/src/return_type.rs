use leptos::prelude::*;

/// Return type of [`fn@crate::use_textarea_autosize`].
pub struct UseTextareaAutosizeReturn<F>
where
    F: Fn() + Clone + Send + Sync,
{
    /// The textarea content
    pub content: Signal<String>,

    /// Set the textarea content
    pub set_content: WriteSignal<String>,

    /// Function to trigger a textarea resize manually
    pub trigger_resize: F,
}
