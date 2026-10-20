use crate::Size;
use default_struct_builder::DefaultBuilder;

/// Options for [`fn@crate::use_window_size_with_options`].
#[derive(DefaultBuilder)]
pub struct UseWindowSizeOptions {
    /// The initial size before anything is measured (like on the server side).
    /// Defaults to `Size { width: INFINITY, height: INFINITY }`.
    pub(crate) initial_size: Size,

    /// Listen to the window ` orientationchange ` event. Defaults to `true`.
    pub(crate) listen_orientation: bool,

    /// Whether the scrollbar should be included in the width and height
    /// Only effective when `measure_type` is `MeasureType::Inner`.
    /// Defaults to `true`.
    pub(crate) include_scrollbar: bool,

    /// Use `window.innerWidth` or `window.outerWidth`.
    /// Defaults to `MeasureType::Inner`.
    pub(crate) measure_type: MeasureType,
}

/// Type of the `measure_type` option.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MeasureType {
    /// Use `window.innerWidth`
    #[default]
    Inner,
    /// Use `window.outerWidth`
    Outer,
}

impl Default for UseWindowSizeOptions {
    fn default() -> Self {
        Self {
            initial_size: Size {
                width: f64::INFINITY,
                height: f64::INFINITY,
            },
            listen_orientation: true,
            include_scrollbar: true,
            measure_type: MeasureType::default(),
        }
    }
}
