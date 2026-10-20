use default_struct_builder::DefaultBuilder;
use std::rc::Rc;

/// Options for [`use_interval_with_options`](crate::use_interval_with_options)
#[derive(DefaultBuilder)]
pub struct UseIntervalOptions {
    /// Start the timer immediately. Defaults to `true`.
    pub(crate) immediate: bool,

    /// Callback on every interval.
    pub(crate) callback: Rc<dyn Fn(u64)>,
}

impl Default for UseIntervalOptions {
    fn default() -> Self {
        Self {
            immediate: true,
            callback: Rc::new(|_: u64| {}),
        }
    }
}
