#[cfg(not(feature = "ssr"))]
use crate::AudioConstraints;
#[cfg(not(feature = "ssr"))]
use crate::AudioTrackConstraints;
#[cfg(not(feature = "ssr"))]
use crate::VideoConstraints;
#[cfg(not(feature = "ssr"))]
use crate::VideoTrackConstraints;
use crate::{OptionLocalRwSignal, UseUserMediaOptions, UseUserMediaReturn};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

/// Reactive [`mediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) streaming.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_user_media)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::{log, error};
/// # use leptos_use_user_media::{use_user_media, UseUserMediaReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let video_ref = NodeRef::<leptos::html::Video>::new();
///
/// let UseUserMediaReturn { stream, start, .. } = use_user_media();
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
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server calls to `start` or any other way to enable the stream will be ignored
/// and the stream will always be `None`.
pub fn use_user_media()
-> UseUserMediaReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    use_user_media_with_options(UseUserMediaOptions::default())
}

/// Version of [`use_user_media`] that takes a `UseUserMediaOptions`. See [`use_user_media`] for how to use.
pub fn use_user_media_with_options(
    options: UseUserMediaOptions,
) -> UseUserMediaReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    let UseUserMediaOptions {
        enabled,
        video,
        audio,
        ..
    } = options;

    let (enabled, set_enabled) = enabled.into_signal();

    let stream = OptionLocalRwSignal::<Result<web_sys::MediaStream, JsValue>>::new();

    let _start = {
        let audio = audio.clone();
        let video = video.clone();

        move || async move {
            #[cfg(not(feature = "ssr"))]
            {
                if stream.get_untracked().is_some() {
                    return;
                }

                let new_stream = create_media(Some(video), Some(audio)).await;

                stream.update(|s| *s = Some(new_stream));
            }

            #[cfg(feature = "ssr")]
            {
                let _ = video;
                let _ = audio;
            }
        }
    };

    let _stop = move || {
        if let Some(sendwrapped_stream) = stream.get_untracked()
            && let Ok(stream) = sendwrapped_stream.as_ref()
        {
            for track in stream.get_tracks() {
                track.unchecked_ref::<web_sys::MediaStreamTrack>().stop();
            }
        }

        stream.set(None);
    };

    // Without this the tracks keep running (and the camera/microphone indicator
    // stays on) after the owner is disposed, with no handle left to stop them.
    #[cfg(not(feature = "ssr"))]
    on_cleanup(_stop);

    let start = {
        #[cfg(not(feature = "ssr"))]
        let _start = _start.clone();
        move || {
            #[cfg(not(feature = "ssr"))]
            {
                leptos::task::spawn_local({
                    let _start = _start.clone();

                    async move {
                        _start().await;
                        stream.with_untracked(move |stream| {
                            if let Some(sendwrapped_stream) = stream
                                && sendwrapped_stream.as_ref().is_ok()
                            {
                                set_enabled.set(true);
                            }
                        });
                    }
                });
            }
        }
    };

    let stop = move || {
        _stop();
        set_enabled.set(false);
    };

    Effect::watch(
        move || enabled.get(),
        move |enabled, _, _| {
            if *enabled {
                leptos::task::spawn_local({
                    #[cfg(not(feature = "ssr"))]
                    let _start = _start.clone();

                    async move {
                        _start().await;
                    }
                });
            } else {
                _stop();
            }
        },
        true,
    );

    UseUserMediaReturn {
        stream: stream.read_only(),
        start,
        stop,
        enabled,
        set_enabled,
    }
}

#[cfg(not(feature = "ssr"))]
async fn create_media(
    video: Option<VideoConstraints>,
    audio: Option<AudioConstraints>,
) -> Result<web_sys::MediaStream, JsValue> {
    use crate::use_window;
    use crate::{js, js_fut};
    use js_sys::Array;

    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let constraints = web_sys::MediaStreamConstraints::new();
    if let Some(video_shadow_constraints) = video {
        match video_shadow_constraints {
            VideoConstraints::Bool(b) => constraints.set_video(&JsValue::from(b)),
            VideoConstraints::Constraints(boxed_constraints) => {
                let VideoTrackConstraints {
                    device_id,
                    facing_mode,
                    frame_rate,
                    height,
                    width,
                    viewport_height,
                    viewport_width,
                    viewport_offset_x,
                    viewport_offset_y,
                    zoom,
                } = *boxed_constraints;

                let video_constraints = web_sys::MediaTrackConstraints::new();

                if !device_id.is_empty() {
                    video_constraints.set_device_id(
                        &Array::from_iter(device_id.into_iter().map(JsValue::from)).into(),
                    );
                }

                if let Some(value) = facing_mode {
                    video_constraints.set_facing_mode(&value.to_jsvalue());
                }

                if let Some(value) = frame_rate {
                    video_constraints.set_frame_rate(&value.to_jsvalue());
                }

                if let Some(value) = height {
                    video_constraints.set_height(&value.to_jsvalue());
                }

                if let Some(value) = width {
                    video_constraints.set_width(&value.to_jsvalue());
                }

                if let Some(value) = viewport_height {
                    video_constraints.set_viewport_height(&value.to_jsvalue());
                }

                if let Some(value) = viewport_width {
                    video_constraints.set_viewport_width(&value.to_jsvalue());
                }
                if let Some(value) = viewport_offset_x {
                    video_constraints.set_viewport_offset_x(&value.to_jsvalue());
                }

                if let Some(value) = viewport_offset_y {
                    video_constraints.set_viewport_offset_y(&value.to_jsvalue());
                }

                let js_value = JsValue::from(video_constraints);

                if let Some(value) = zoom {
                    // TODO : once web_sys implements this add it to the methods above
                    _ = js!(js_value["zoom"] = value.to_jsvalue());
                }

                constraints.set_video(&js_value);
            }
        }
    }
    if let Some(audio_shadow_constraints) = audio {
        match audio_shadow_constraints {
            AudioConstraints::Bool(b) => constraints.set_audio(&JsValue::from(b)),
            AudioConstraints::Constraints(boxed_constraints) => {
                let AudioTrackConstraints {
                    device_id,
                    auto_gain_control,
                    channel_count,
                    echo_cancellation,
                    noise_suppression,
                } = *boxed_constraints;

                let audio_constraints = web_sys::MediaTrackConstraints::new();

                if !device_id.is_empty() {
                    audio_constraints.set_device_id(
                        &Array::from_iter(device_id.into_iter().map(JsValue::from)).into(),
                    );
                }
                if let Some(value) = auto_gain_control {
                    audio_constraints.set_auto_gain_control(&JsValue::from(&value.to_jsvalue()));
                }
                if let Some(value) = channel_count {
                    audio_constraints.set_channel_count(&JsValue::from(&value.to_jsvalue()));
                }
                if let Some(value) = echo_cancellation {
                    audio_constraints.set_echo_cancellation(&JsValue::from(&value.to_jsvalue()));
                }
                if let Some(value) = noise_suppression {
                    audio_constraints.set_noise_suppression(&JsValue::from(&value.to_jsvalue()));
                }

                constraints.set_audio(&JsValue::from(audio_constraints));
            }
        }
    }

    let promise = media.get_user_media_with_constraints(&constraints)?;
    let res = js_fut!(promise).await?;

    Ok::<_, JsValue>(web_sys::MediaStream::unchecked_from_js(res))
}

pub trait ConstraintRangeBuilder<T> {
    fn min(self, value: T) -> Self;
    fn max(self, value: T) -> Self;
    fn exact(self, value: T) -> Self;
    fn ideal(self, value: T) -> Self;
}

pub trait IntoDeviceIds<M> {
    fn into_device_ids(self) -> Vec<String>;
}

impl<T> IntoDeviceIds<String> for T
where
    T: Into<String>,
{
    fn into_device_ids(self) -> Vec<String> {
        vec![self.into()]
    }
}

pub struct VecMarker;

impl<T, I> IntoDeviceIds<VecMarker> for T
where
    T: IntoIterator<Item = I>,
    I: Into<String>,
{
    fn into_device_ids(self) -> Vec<String> {
        self.into_iter().map(Into::into).collect()
    }
}
