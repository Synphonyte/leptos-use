use default_struct_builder::DefaultBuilder;

/// Options for [`use_resize_observer_with_options`](crate::use_resize_observer_with_options).
#[derive(DefaultBuilder, Clone, Default)]
pub struct UseResizeObserverOptions {
    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    #[builder(into)]
    pub box_: Option<web_sys::ResizeObserverBoxOptions>,
}

impl From<UseResizeObserverOptions> for web_sys::ResizeObserverOptions {
    fn from(val: UseResizeObserverOptions) -> Self {
        let options = web_sys::ResizeObserverOptions::new();
        options.set_box(
            val.box_
                .unwrap_or(web_sys::ResizeObserverBoxOptions::ContentBox),
        );
        options
    }
}
