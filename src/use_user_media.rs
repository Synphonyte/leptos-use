use crate::core::MaybeRwSignal;
use default_struct_builder::DefaultBuilder;
use leptos::*;
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
/// # use leptos::*;
/// # use leptos_use::{use_user_media, UseUserMediaReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let video_ref = create_node_ref::<leptos::html::Video>();
///
/// let UseUserMediaReturn { stream, start, .. } = use_user_media();
///
/// start();
///
/// create_effect(move |_|
///     video_ref.get().map(|v| {
///         match stream.get() {
///             Some(Ok(s)) => v.set_src_object(Some(&s)),
///             Some(Err(e)) => logging::error!("Failed to get media stream: {:?}", e),
///             None => logging::log!("No stream yet"),
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
pub fn use_user_media() -> UseUserMediaReturn<impl Fn() + Clone, impl Fn() + Clone> {
    use_user_media_with_options(UseUserMediaOptions::default())
}

/// Version of [`use_user_media`] that takes a `UseUserMediaOptions`. See [`use_user_media`] for how to use.
pub fn use_user_media_with_options(
    options: UseUserMediaOptions,
) -> UseUserMediaReturn<impl Fn() + Clone, impl Fn() + Clone> {
    let UseUserMediaOptions {
        enabled,
        video,
        audio,
        ..
    } = options;

    let (enabled, set_enabled) = enabled.into_signal();

    let (stream, set_stream) = create_signal(None::<Result<web_sys::MediaStream, JsValue>>);

    let _start = move || async move {
        #[cfg(not(feature = "ssr"))]
        {
            if stream.get_untracked().is_some() {
                return;
            }

            let stream = create_media(video, audio).await;

            set_stream.update(|s| *s = Some(stream));
        }

        #[cfg(feature = "ssr")]
        {
            let _ = video;
            let _ = audio;
        }
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
        #[cfg(not(feature = "ssr"))]
        {
            spawn_local(async move {
                _start().await;
                stream.with_untracked(move |stream| {
                    if let Some(Ok(_)) = stream {
                        set_enabled.set(true);
                    }
                });
            });
        }
    };

    let stop = move || {
        _stop();
        set_enabled.set(false);
    };

    let _ = watch(
        move || enabled.get(),
        move |enabled, _, _| {
            if *enabled {
                spawn_local(async move {
                    _start().await;
                });
            } else {
                _stop();
            }
        },
        true,
    );
    UseUserMediaReturn {
        stream: stream.into(),
        start,
        stop,
        enabled,
        set_enabled,
    }
}

#[cfg(not(feature = "ssr"))]
async fn create_media(video: MediaTrackConstraints, audio: MediaTrackConstraints) -> Result<web_sys::MediaStream, JsValue> {
    use crate::js_fut;
    use crate::use_window::use_window;

    let media = use_window()
      .navigator()
      .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
      .and_then(|n| n.media_devices())?;

    let constraints = web_sys::MediaStreamConstraints::new();
    if video.value.unwrap_or_default() {

        let video_constraints = web_sys::MediaTrackConstraints::new();

        video_constraints.set_facing_mode(&JsValue::from_str(video.facing_mode.unwrap_or(FacingMode::Environment).as_str())); // Use "environment" for the back camera

        constraints.set_video(&JsValue::from(video_constraints));

        // constraints.set_video(&JsValue::from(true));
        //
        //
        //
        // let facing_mode = match video.facing_mode {
        //     Some(facing_mode) => web_sys::VideoFacingModeEnum::unchecked_from_js((facing_mode as u32).into()).unchecked_into(),
        //     None => web_sys::VideoFacingModeEnum::Environment,
        // };
        //
        // track_constraints.set_facing_mode(&JsValue::from(facing_mode));
        //
        // constraints.set_video(&track_constraints);
    }
    if audio.value.unwrap_or_default() {
        let audio_constraints = web_sys::MediaTrackConstraints::new();

        audio_constraints.set_facing_mode(&JsValue::from_str(video.facing_mode.unwrap_or(FacingMode::Environment).as_str())); // Use "environment" for the back camera

        constraints.set_audio(&JsValue::from(audio_constraints));
    }

    let promise = media.get_user_media_with_constraints(&constraints)?;
    let res = js_fut!(promise).await?;

    Ok::<_, JsValue>(web_sys::MediaStream::unchecked_from_js(res))
}

/// Options for [`use_user_media_with_options`].
///
/// Either or both constraints must be specified.
/// If the browser cannot find all media tracks with the specified types that meet the constraints given,
/// then the returned promise is rejected with `NotFoundError`
#[derive(DefaultBuilder, Clone, Copy, Debug)]
pub struct UseUserMediaOptions {
    /// If the stream is enabled. Defaults to `false`.
    enabled: MaybeRwSignal<bool>,
    /// Constraint parameter describing video media type requested
    /// The default value is `false`.
    video: MediaTrackConstraints,
    /// Constraint parameter describing audio media type requested
    /// The default value is `false`.
    audio: MediaTrackConstraints,
}

impl Default for UseUserMediaOptions {
    fn default() -> Self {
        Self {
            enabled: false.into(),
            video: MediaTrackConstraintsBuilder::new().build(),
            audio: MediaTrackConstraintsBuilder::new().build(),
        }
    }
}

/// Return type of [`use_user_media`].
#[derive(Clone)]
pub struct UseUserMediaReturn<StartFn, StopFn>
where
  StartFn: Fn() + Clone,
  StopFn: Fn() + Clone,
{
    /// The current [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream) if it exists.
    /// Initially this is `None` until `start` resolved successfully.
    /// In case the stream couldn't be started, for example because the user didn't grant permission,
    /// this has the value `Some(Err(...))`.
    pub stream: Signal<Option<Result<web_sys::MediaStream, JsValue>>>,

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

#[derive(Clone, Copy, Debug)]
pub enum FacingMode {
    User,
    Environment,
    Left,
    Right,
}

impl FacingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            FacingMode::User => "user",
            FacingMode::Environment => "environment",
            FacingMode::Left => "left",
            FacingMode::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MediaTrackConstraints {
    pub facing_mode: Option<FacingMode>,
    pub value: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub frame_rate: Option<i32>,
    pub exact: Option<bool>,
}

impl MediaTrackConstraints {
    pub fn builder() -> MediaTrackConstraintsBuilder {
        MediaTrackConstraintsBuilder::default()
    }
}

#[derive(Default)]
pub struct MediaTrackConstraintsBuilder {
    facing_mode: Option<FacingMode>,
    value: Option<bool>,
    width: Option<i32>,
    height: Option<i32>,
    frame_rate: Option<i32>,
    exact: Option<bool>,
}

impl MediaTrackConstraintsBuilder {
    pub fn new() -> MediaTrackConstraintsBuilder {
        MediaTrackConstraintsBuilder {
            exact: false.into(),
            frame_rate: None,
            height: None,
            width: None,
            facing_mode: None,
            value: None,
        }
    }

    pub fn facing_mode(mut self, facing_mode: FacingMode) -> MediaTrackConstraintsBuilder {
        self.facing_mode = Some(facing_mode);
        self
    }

    pub fn value(mut self, value: bool) -> MediaTrackConstraintsBuilder {
        self.value = Some(value);
        self
    }

    pub fn build(self) -> MediaTrackConstraints {
        MediaTrackConstraints { facing_mode: self.facing_mode, value: self.value, width: self.width, height: self.height, frame_rate: self.frame_rate, exact: self.exact }
    }

}