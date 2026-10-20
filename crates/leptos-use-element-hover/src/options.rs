use default_struct_builder::DefaultBuilder;

/// Options for [`use_element_hover_with_options`](crate::use_element_hover_with_options).
#[derive(DefaultBuilder, Default)]
pub struct UseElementHoverOptions {
    /// The time in ms the mouse has to be hovered over the element before the signal is changed to `true`. Defaults to `0`.
    pub(crate) delay_enter: u64,

    /// The time in ms after the mouse has left the element before the signal is changed to `false`. Defaults to `0`.
    pub(crate) delay_leave: u64,
}
