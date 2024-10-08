use crate::core::MaybeRwSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use wasm_bindgen::{JsCast, JsValue};

/// Reactive [`mediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia) streaming.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_display_media)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_display_media, UseDisplayMediaReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let video_ref = NodeRef::<leptos::html::Video>::new();
///
/// let UseDisplayMediaReturn { stream, start, .. } = use_display_media();
///
/// start();
///
/// Effect::new(move |_|
///     video_ref.get().map(|v| {
///         match stream.get() {
///             Some(Ok(s)) => v.set_src_object(Some(&s)),
///             Some(Err(e)) => error!("Failed to get media stream: {:?}", e),
///             None => log!("No stream yet"),
///         }
///     })
/// );
///
/// view! { <video node_ref=video_ref controls=false autoplay=true muted=true></video> }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server calls to `start` or any other way to enable the stream will be ignored
/// and the stream will always be `None`.
pub fn use_display_media() -> UseDisplayMediaReturn<impl Fn() + Clone, impl Fn() + Clone> {
    use_display_media_with_options(UseDisplayMediaOptions::default())
}

/// Version of [`use_display_media`] that accepts a [`UseDisplayMediaOptions`].
pub fn use_display_media_with_options(
    options: UseDisplayMediaOptions,
) -> UseDisplayMediaReturn<impl Fn() + Clone, impl Fn() + Clone> {
    let UseDisplayMediaOptions { enabled, audio } = options;

    let (enabled, set_enabled) = enabled.into_signal();

    let (stream, set_stream) = signal_local(None::<Result<web_sys::MediaStream, JsValue>>);

    let _start = move || async move {
        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            if stream.get_untracked().is_some() {
                return;
            }

            let stream = create_media(audio).await;

            set_stream.update(|s| *s = Some(stream));
        } else {
            let _ = audio;
        }}
    };

    let _stop = move || {
        if let Some(Ok(stream)) = stream.get_untracked() {
            for track in stream.get_tracks() {
                track.unchecked_ref::<web_sys::MediaStreamTrack>().stop();
            }
        }

        set_stream.set(None);
    };

    let start = move || {
        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            leptos::task::spawn_local(async move {
                _start().await;
                stream.with_untracked(move |stream| {
                    if let Some(Ok(_)) = stream {
                        set_enabled.set(true);
                    }
                });
            });
        }}
    };

    let stop = move || {
        _stop();
        set_enabled.set(false);
    };

    Effect::watch(
        move || enabled.get(),
        move |enabled, _, _| {
            if *enabled {
                leptos::task::spawn_local(async move {
                    _start().await;
                });
            } else {
                _stop();
            }
        },
        true,
    );

    UseDisplayMediaReturn {
        stream: stream.into(),
        start,
        stop,
        enabled,
        set_enabled,
    }
}

#[cfg(not(feature = "ssr"))]
async fn create_media(audio: bool) -> Result<web_sys::MediaStream, JsValue> {
    use crate::js_fut;
    use crate::use_window::use_window;

    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let constraints = web_sys::DisplayMediaStreamConstraints::new();
    if audio {
        constraints.set_audio(&JsValue::from(true));
    }

    let promise = media.get_display_media_with_constraints(&constraints)?;
    let res = js_fut!(promise).await?;

    Ok::<_, JsValue>(web_sys::MediaStream::unchecked_from_js(res))
}

// NOTE: there's no video value because it has to be `true`. Otherwise the stream would always resolve to an Error.
/// Options for [`use_display_media`].
#[derive(DefaultBuilder, Clone, Copy, Debug)]
pub struct UseDisplayMediaOptions {
    /// If the stream is enabled. Defaults to `false`.
    enabled: MaybeRwSignal<bool>,

    /// A value of `true` indicates that the returned [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)
    /// will contain an audio track, if audio is supported and available for the display surface chosen by the user.
    /// The default value is `false`.
    audio: bool,
}

impl Default for UseDisplayMediaOptions {
    fn default() -> Self {
        Self {
            enabled: false.into(),
            audio: false,
        }
    }
}

/// Return type of [`use_display_media`]
#[derive(Clone)]
pub struct UseDisplayMediaReturn<StartFn, StopFn>
where
    StartFn: Fn() + Clone,
    StopFn: Fn() + Clone,
{
    /// The current [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream) if it exists.
    /// Initially this is `None` until `start` resolved successfully.
    /// In case the stream couldn't be started, for example because the user didn't grant permission,
    /// this has the value `Some(Err(...))`.
    pub stream: Signal<Option<Result<web_sys::MediaStream, JsValue>>, LocalStorage>,

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
