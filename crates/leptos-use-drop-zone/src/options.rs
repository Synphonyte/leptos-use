use default_struct_builder::DefaultBuilder;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// Options for [`use_drop_zone_with_options`](crate::use_drop_zone_with_options).
#[derive(DefaultBuilder, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseDropZoneOptions {
    /// Event handler for the [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event) event
    pub(crate) on_drop: Arc<dyn Fn(UseDropZoneEvent) + Send + Sync>,
    /// Event handler for the [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event) event
    pub(crate) on_enter: Arc<dyn Fn(UseDropZoneEvent) + Send + Sync>,
    /// Event handler for the [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event) event
    pub(crate) on_leave: Arc<dyn Fn(UseDropZoneEvent) + Send + Sync>,
    /// Event handler for the [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event) event
    pub(crate) on_over: Arc<dyn Fn(UseDropZoneEvent) + Send + Sync>,
}

impl Default for UseDropZoneOptions {
    fn default() -> Self {
        Self {
            on_drop: Arc::new(|_| {}),
            on_enter: Arc::new(|_| {}),
            on_leave: Arc::new(|_| {}),
            on_over: Arc::new(|_| {}),
        }
    }
}

impl Debug for UseDropZoneOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UseDropZoneOptions")
    }
}

/// Event passed as argument to the event handler functions of `UseDropZoneOptions`.
#[derive(Clone, Debug)]
pub struct UseDropZoneEvent {
    /// Files being handled
    pub files: Vec<web_sys::File>,
    /// The original drag event
    pub event: web_sys::DragEvent,
}
