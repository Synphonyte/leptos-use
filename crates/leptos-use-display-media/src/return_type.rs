use crate::OptionLocalSignal;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use wasm_bindgen::JsValue;

/// Return type of [`use_display_media`](crate::use_display_media)
#[derive(Clone)]
pub struct UseDisplayMediaReturn<StartFn, StopFn>
where
    StartFn: Fn() + Clone + Send + Sync,
    StopFn: Fn() + Clone + Send + Sync,
{
    /// The current [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream) if it exists.
    /// Initially this is `None` until `start` resolved successfully.
    /// In case the stream couldn't be started, for example because the user didn't grant permission,
    /// this has the value `Some(Err(...))`.
    pub stream: OptionLocalSignal<Result<web_sys::MediaStream, JsValue>>,

    /// Starts the screen streaming. Triggers the ask for permission if not already granted.
    pub start: StartFn,

    /// Stops the screen streaming
    pub stop: StopFn,

    /// A value of `true` indicates that the returned [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)
    /// has resolved successfully and thus the stream is enabled.
    pub enabled: Signal<bool>,

    /// A value of `true` is the same as calling `start()` whereas `false` is the same as calling `stop()`.
    pub set_enabled: WriteSignal<bool>,
}
