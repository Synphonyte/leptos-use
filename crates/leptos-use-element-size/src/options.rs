use crate::Size;
use default_struct_builder::DefaultBuilder;

#[derive(DefaultBuilder, Default)]
/// Options for [`use_element_size_with_options`](crate::use_element_size_with_options).
pub struct UseElementSizeOptions {
    /// Initial size returned before any measurements on the `target` are done. Also the value reported
    /// at first when the `target` is a signal and changes.
    pub(crate) initial_size: Size,

    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    #[builder(into)]
    pub box_: Option<web_sys::ResizeObserverBoxOptions>,
}
