use leptos::reactive::wrappers::read::Signal;
use send_wrapper::SendWrapper;

/// Return type of [`use_drop_zone`](crate::use_drop_zone).
#[derive(Clone, Copy)]
pub struct UseDropZoneReturn {
    /// Files being handled
    pub files: Signal<Vec<SendWrapper<web_sys::File>>>,
    /// Whether the files (dragged by the pointer) are over the drop zone
    pub is_over_drop_zone: Signal<bool>,
}
