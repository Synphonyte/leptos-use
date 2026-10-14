use crate::{ReconnectLimit, UseEventSourceOnEventReturn};
use default_struct_builder::DefaultBuilder;
use std::marker::PhantomData;
use std::sync::Arc;

/// Options for [`use_event_source_with_options`](crate::use_event_source_with_options).
#[derive(DefaultBuilder)]
pub struct UseEventSourceOptions<T>
where
    T: 'static,
{
    /// Retry times. Defaults to `ReconnectLimit::Limited(3)`. Use `ReconnectLimit::Infinite` for
    /// infinite retries.
    pub(crate) reconnect_limit: ReconnectLimit,

    /// Retry interval in ms. Defaults to 3000.
    pub(crate) reconnect_interval: u64,

    /// On maximum retry times reached.
    pub(crate) on_failed: Arc<dyn Fn() + Send + Sync>,

    /// If `true` the `EventSource` connection will immediately be opened when calling this function.
    /// If `false` you have to manually call the `open` function.
    /// Defaults to `true`.
    pub(crate) immediate: bool,

    /// List of named events to listen for on the `EventSource`.
    #[builder(into)]
    pub(crate) named_events: Vec<String>,

    /// The `on_event` is called before processing any event inside of [`use_event_source`](crate::use_event_source).
    /// Return `UseEventSourceOnEventReturn::Ignore` to ignore further processing of the respective event
    /// in [`use_event_source`](crate::use_event_source), or `UseEventSourceOnEventReturn::Use` to process the event as usual.
    ///
    /// Beware that ignoring processing the `open` and `error` events may yield unexpected results.
    ///
    /// You may want to use [`UseEventSourceMessage::try_from()`](crate::UseEventSourceMessage::try_from) to access the event data.
    ///
    /// Default handler returns `UseEventSourceOnEventReturn::Use`.
    pub(crate) on_event: Arc<dyn Fn(&web_sys::Event) -> UseEventSourceOnEventReturn + Send + Sync>,

    /// If CORS should be set to `include` credentials. Defaults to `false`.
    pub(crate) with_credentials: bool,

    pub(crate) _marker: PhantomData<T>,
}

impl<T> Default for UseEventSourceOptions<T> {
    fn default() -> Self {
        Self {
            reconnect_limit: ReconnectLimit::default(),
            reconnect_interval: 3000,
            on_failed: Arc::new(|| {}),
            immediate: true,
            named_events: vec![],
            on_event: Arc::new(|_| UseEventSourceOnEventReturn::ProcessMessage),
            with_credentials: false,
            _marker: PhantomData,
        }
    }
}
