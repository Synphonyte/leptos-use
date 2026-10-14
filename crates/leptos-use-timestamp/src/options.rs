use default_struct_builder::DefaultBuilder;
use std::rc::Rc;

/// Options for [`use_timestamp_with_controls_and_options`](crate::use_timestamp_with_controls_and_options).
#[derive(DefaultBuilder)]
pub struct UseTimestampOptions {
    /// Offset value in milliseconds that is added to the returned timestamp. Defaults to `0.0`.
    pub(crate) offset: f64,

    /// Whether to update the timestamp immediately. Defaults to `true`.
    pub(crate) immediate: bool,

    /// Update interval in milliseconds or `RequestAnimationFrame`. Defaults to `RequestAnimationFrame`.
    #[builder(into)]
    pub(crate) interval: TimestampInterval,

    /// Callback to be called whenever the timestamp is updated.
    pub(crate) callback: Rc<dyn Fn(f64)>,
}

/// Interval type for [`UseTimestampOptions`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TimestampInterval {
    /// use [`fn@crate::use_raf_fn`] for updating the timestamp
    RequestAnimationFrame,

    /// use [`fn@crate::use_interval_fn`] for updating the timestamp
    Interval(u64),
}

impl From<u64> for TimestampInterval {
    fn from(value: u64) -> Self {
        Self::Interval(value)
    }
}

impl Default for UseTimestampOptions {
    fn default() -> Self {
        Self {
            offset: 0.0,
            immediate: true,
            interval: TimestampInterval::RequestAnimationFrame,
            callback: Rc::new(|_| {}),
        }
    }
}
