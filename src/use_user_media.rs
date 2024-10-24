use crate::core::MaybeRwSignal;
use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use js_sys::{Object, Reflect};

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

            let stream = create_media(Some(video), Some(audio)).await;

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
async fn create_media(
    video: Option<VideoConstraints>,
    audio: Option<AudioConstraints>,
) -> Result<web_sys::MediaStream, JsValue> {
    use crate::js_fut;
    use crate::use_window::use_window;

    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let constraints = web_sys::MediaStreamConstraints::new();
    if let Some(video_shadow_constraints) = video {
        match video_shadow_constraints {
            VideoConstraints::Bool(b) => constraints.set_video(&JsValue::from(b)),
            VideoConstraints::Constraints(m) => {
                let video_constraints = web_sys::MediaTrackConstraints::new();

                if let Some(facing_mode) = m.facing_mode {
                    let facing_mode = facing_mode.to_jsvalue();
                    video_constraints.set_facing_mode(&facing_mode);
                }

                constraints.set_video(&JsValue::from(video_constraints));
            }
        }

        //
        // video_constraints.set_facing_mode(&JsValue::from_str(video_constraints.facing_mode.unwrap_or(FacingMode::Environment).as_str())); // Use "environment" for the back camera
        //

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
    if let Some(audio_shadow_constraints) = audio {
        match audio_shadow_constraints {
            AudioConstraints::Bool(b) => constraints.set_audio(&JsValue::from(b)),
            AudioConstraints::Constraints(a) => {
                let audio_constraints = web_sys::MediaTrackConstraints::new();


                if let  Some(device_id) = a.device_id {

                    audio_constraints.set_device_id(&JsValue::from(&device_id.to_jsvalue()));
                }
                if let Some(auto_gain_control) = a.auto_gain_control {
                    audio_constraints.set_auto_gain_control(&JsValue::from(&auto_gain_control.to_jsvalue()));
                }
                if let Some(channel_count) = a.channel_count {
                    audio_constraints.set_channel_count(&JsValue::from(&channel_count.to_jsvalue()));
                }
                if let Some(echo_cancellation) = a.echo_cancellation {
                    audio_constraints.set_echo_cancellation(&JsValue::from(&echo_cancellation.to_jsvalue()));
                }
                if let Some(noise_suppression) = a.noise_suppression {
                    audio_constraints.set_noise_suppression(&JsValue::from(&noise_suppression.to_jsvalue()));
                }

                // Not yet implemented
                // if let Some(group_id) = a.group_id {
                //     audio_constraints.set_group_id(&JsValue::from(group_id));
                // }
                // if let Some(latency) = a.latency {
                //     audio_constraints.set_latency(&JsValue::from_str(&latency.to_string()));
                // }
                // if let Some(sample_rate) = a.sample_rate {
                //     audio_constraints.set_sample_rate(&JsValue::from(sample_rate));
                // }
                // if let Some(sample_size) = a.sample_size {
                //     audio_constraints.set_sample_size(&JsValue::from(sample_size));
                // }
                // if let Some(volume) = a.volume {
                //     audio_constraints.set_volume(&JsValue::from(volume));
                // }

                constraints.set_audio(&JsValue::from(audio_constraints));
            }
        }
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
#[derive(Clone, Copy, Debug)]
pub struct UseUserMediaOptions {
    /// If the stream is enabled. Defaults to `false`.
    enabled: MaybeRwSignal<bool>,
    /// Constraint parameter describing video media type requested
    /// The default value is `false`.
    video: VideoConstraints,
    /// Constraint parameter describing audio media type requested
    /// The default value is `false`.
    audio: AudioConstraints,
}

impl Default for UseUserMediaOptions {
    fn default() -> Self {
        Self {
            enabled: false.into(),
            video: false.into(),
            audio: false.into(),
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
pub enum Constrain<T> {
    Single(Option<T>), // A single value (exact constraint)
    Range {
        min: Option<T>,   // For numeric types (e.g., u64, f64)
        max: Option<T>,   // For numeric types
        exact: Option<T>, // Exact constraint
        ideal: Option<T>, // Ideal constraint
    },
    ExactIdeal {
        exact: Option<T>, // Optional exact constraint for boolean, string, etc.
        ideal: Option<T>, // Optional ideal constraint for boolean, string, etc.
    },
}

impl<T> Constrain<T>
where
    T: Clone + std::fmt::Debug,
{
    // Constructor for a single exact value
    pub fn new(value: Option<T>) -> Self {
        Constrain::Single(value)
    }

    // Constructor for a range of numeric constraints
    pub fn range() -> Self {
        Constrain::Range {
            min: None,
            max: None,
            exact: None,
            ideal: None,
        }
    }

    // Constructor for exact/ideal types (boolean, string, etc.)
    pub fn exact_ideal(exact: Option<T>, ideal: Option<T>) -> Self {
        Constrain::ExactIdeal { exact, ideal }
    }

    // Set the `min` constraint for numeric types
    pub fn min(mut self, value: T) -> Self {
        if let Constrain::Range { ref mut min, .. } = self {
            *min = Some(value);
        }
        self
    }

    // Set the `max` constraint for numeric types
    pub fn max(mut self, value: T) -> Self {
        if let Constrain::Range { ref mut max, .. } = self {
            *max = Some(value);
        }
        self
    }

    // Set the `exact` constraint
    pub fn exact(mut self, value: T) -> Self {
        match &mut self {
            Constrain::Range { ref mut exact, .. } => {
                *exact = Some(value);
            }
            Constrain::ExactIdeal {
                exact: ref mut e, ..
            } => {
                *e = Some(value);
            }
            _ => {}
        }
        self
    }

    // Set the `ideal` constraint
    pub fn ideal(mut self, value: T) -> Self {
        match &mut self {
            Constrain::Range { ref mut ideal, .. } => {
                *ideal = Some(value);
            }
            Constrain::ExactIdeal {
                ideal: ref mut i, ..
            } => {
                *i = Some(value);
            }
            _ => {}
        }
        self
    }
}

impl<T> Default for Constrain<T>
where
  T: Default,
{
    fn default() -> Self {
        Constrain::Single(Some(T::default()))
    }
}

impl ConstrainFacingMode
{
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            Constrain::Single(value) => JsValue::from_str(value.clone().unwrap().as_str()),  // Convert single value directly to JsValue
            Constrain::Range { min, max, exact, ideal } => {
                // Create a JavaScript object for the range constraints
                let obj = Object::new();

                if let Some(min_value) = min {
                    Reflect::set(&obj, &JsValue::from_str("min"), &JsValue::from_str(min_value.as_str())).unwrap();
                }
                if let Some(max_value) = max {
                    Reflect::set(&obj, &JsValue::from_str("max"), &JsValue::from_str(max_value.as_str())).unwrap();
                }
                if let Some(exact_value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &JsValue::from_str(exact_value.as_str())).unwrap();
                }
                if let Some(ideal_value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &JsValue::from_str(ideal_value.as_str())).unwrap();
                }

                JsValue::from(obj)
            }
            Constrain::ExactIdeal { exact, ideal } => {
                // Create a JavaScript object for the exact/ideal constraints
                let obj = Object::new();

                if let Some(exact_value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &JsValue::from_str(exact_value.as_str())).unwrap();
                }
                if let Some(ideal_value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &JsValue::from_str(ideal_value.as_str())).unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

impl<T> Constrain<T>
where
  T: Into<JsValue> + Clone,
{
    // Convert the Constrain<T> to a JsValue (for JS interop)
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            Constrain::Single(value) => JsValue::from(value.clone().unwrap().into()),  // Convert single value directly to JsValue
            Constrain::Range { min, max, exact, ideal } => {
                // Create a JavaScript object for the range constraints
                let obj = Object::new();

                if let Some(min_value) = min {
                    Reflect::set(&obj, &JsValue::from_str("min"), &min_value.clone().into()).unwrap();
                }
                if let Some(max_value) = max {
                    Reflect::set(&obj, &JsValue::from_str("max"), &max_value.clone().into()).unwrap();
                }
                if let Some(exact_value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &exact_value.clone().into()).unwrap();
                }
                if let Some(ideal_value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &ideal_value.clone().into()).unwrap();
                }

                JsValue::from(obj)
            }
            Constrain::ExactIdeal { exact, ideal } => {
                // Create a JavaScript object for the exact/ideal constraints
                let obj = Object::new();

                if let Some(exact_value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &exact_value.clone().into()).unwrap();
                }
                if let Some(ideal_value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &ideal_value.clone().into()).unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

pub type ConstrainDouble = Constrain<f64>;
pub type ConstrainULong = Constrain<u32>;

pub type ConstrainBool = Constrain<bool>;

pub type ConstrainDOMString = Constrain<&'static str>;

pub type ConstrainFacingMode = Constrain<FacingMode>;

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
pub enum AudioConstraints {
    Bool(bool),
    Constraints(AudioTrackConstraints),
}

#[derive(Clone, Copy, Debug)]
pub enum VideoConstraints {
    Bool(bool),
    Constraints(VideoTrackConstraints),
}

impl UseUserMediaOptions {
    pub fn video<T>(mut self, input: T) -> Self
    where
        T: Into<VideoConstraints>,
    {
        self.video = input.into();
        self
    }

    pub fn audio<T>(mut self, input: T) -> Self
    where
        T: Into<AudioConstraints>,
    {
        self.audio = input.into();
        self
    }
}

impl Into<VideoConstraints> for bool {
    fn into(self) -> VideoConstraints {
        VideoConstraints::Bool(self)
    }
}

impl Into<VideoConstraints> for VideoTrackConstraints {
    fn into(self) -> VideoConstraints {
        VideoConstraints::Constraints(self)
    }
}

impl Into<AudioConstraints> for bool {
    fn into(self) -> AudioConstraints {
        AudioConstraints::Bool(self)
    }
}

impl Into<AudioConstraints> for AudioTrackConstraints {
    fn into(self) -> AudioConstraints {
        AudioConstraints::Constraints(self)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct AudioTrackConstraints {
    pub device_id: Option<ConstrainDOMString>,

    pub auto_gain_control: Option<ConstrainBool>,
    pub channel_count: Option<ConstrainBool>,
    pub echo_cancellation: Option<ConstrainBool>,
    pub noise_suppression: Option<ConstrainBool>,

    // pub group_id: Option<ConstrainDOMString>,
    // pub latency: Option<ConstrainDouble>,
    // pub sample_rate: Option<ConstrainULong>,
    // pub sample_size: Option<ConstrainULong>,
    // pub volume: Option<ConstrainDouble>, // deprecated
}

impl AudioTrackConstraints {
    // Constructor for a new instance
    pub fn new() -> Self {
        AudioTrackConstraints::default() // Start with default empty constraints
    }

    // Builder methods accepting `Constrain<T>` directly
    pub fn device_id(mut self, value: &'static str) -> Self {
        self.device_id = Some(ConstrainDOMString::new(Some(&value)));
        self
    }

    pub fn auto_gain_control(mut self, value: bool) -> Self {
        self.auto_gain_control = Some(ConstrainBool::new(Some(value)));
        self
    }

    pub fn auto_gain_control_range(mut self, exact: Option<bool>, ideal: Option<bool>) -> Self {
        self.auto_gain_control = Some(Constrain::ExactIdeal { exact, ideal });
        self
    }

    pub fn channel_count(mut self, value: bool) -> Self {
        self.channel_count = Some(ConstrainBool::new(Some(value)));
        self
    }

    pub fn channel_count_range(mut self, exact: Option<bool>, ideal: Option<bool>) -> Self {
        self.channel_count = Some(Constrain::ExactIdeal { exact, ideal });
        self
    }

    pub fn echo_cancellation(mut self, value: bool) -> Self {
        self.echo_cancellation = Some(ConstrainBool::new(Some(value)));
        self
    }

    pub fn echo_cancellation_range(mut self, exact: Option<bool>, ideal: Option<bool>) -> Self {
        self.echo_cancellation = Some(Constrain::ExactIdeal { exact, ideal });
        self
    }

    pub fn noise_suppression(mut self, value: bool) -> Self {
        self.noise_suppression = Some(ConstrainBool::new(Some(value)));
        self
    }

    pub fn noise_suppression_range(mut self, exact: Option<bool>, ideal: Option<bool>) -> Self {
        self.noise_suppression = Some(Constrain::ExactIdeal { exact, ideal });
        self
    }

    // pub fn group_id(mut self, value: &'static str) -> Self {
    //     self.group_id = Some(ConstrainDOMString::new(Some(&value)));
    //     self
    // }
    //
    // pub fn latency(mut self, value: ConstrainDouble) -> Self {
    //     self.latency = Some(value);
    //     self
    // }
    //
    //
    // pub fn sample_rate(mut self, value: ConstrainULong) -> Self {
    //     self.sample_rate = Some(value);
    //     self
    // }
    //
    // pub fn sample_size(mut self, value: ConstrainULong) -> Self {
    //     self.sample_size = Some(value);
    //     self
    // }
    //
    // // Deprecated field for volume
    // pub fn volume(mut self, value: ConstrainDouble) -> Self {
    //     self.volume = Some(value);
    //     self
    // }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct VideoTrackConstraints {
    pub device_id: Option<ConstrainDOMString>,

    // pub group_id: Option<&'static str>,  // Static string for group ID

    pub facing_mode: Option<ConstrainFacingMode>,
    pub frame_rate: Option<ConstrainDouble>,
    pub height: Option<ConstrainULong>,
    pub width: Option<ConstrainULong>,
    pub viewport_offset_x: Option<ConstrainULong>,
    pub viewport_offset_y: Option<ConstrainULong>,
    pub viewport_height: Option<ConstrainULong>,
    pub viewport_width: Option<ConstrainULong>,
}

impl VideoTrackConstraints {

    // Constructor for a new instance
    pub fn new() -> Self {
        VideoTrackConstraints::default() // Start with default empty constraints
    }

    pub fn device_id(mut self, value: &'static str) -> Self {
        self.device_id = Some(ConstrainDOMString::new(Some(&value)));
        self
    }

    pub fn facing_mode(mut self, value: FacingMode) -> Self {
        self.facing_mode = Some(ConstrainFacingMode::Single(Some(value)));
        self
    }

    pub fn facing_mode_range(mut self, exact: Option<FacingMode>, ideal: Option<FacingMode>) -> Self {
        self.facing_mode = Some(Constrain::ExactIdeal { exact, ideal });
        self
    }

    pub fn frame_rate(mut self, value: f64) -> Self {
        self.frame_rate = Some(ConstrainDouble::new(Some(value)));
        self
    }

    pub fn frame_rate_range(mut self, min: Option<f64>, max: Option<f64>, exact: Option<f64>, ideal: Option<f64>) -> Self {
        self.frame_rate = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn height_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.height = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn width_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.width = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn viewport_offset_x(mut self, value: u32) -> Self {
        self.viewport_offset_x = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn viewport_offset_x_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.viewport_offset_x = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn viewport_offset_y(mut self, value: u32) -> Self {
        self.viewport_offset_y = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn viewport_offset_y_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.viewport_offset_y = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn viewport_height(mut self, value: u32) -> Self {
        self.viewport_height = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn viewport_height_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.viewport_height = Some(Constrain::Range { min, max, exact, ideal });
        self
    }

    pub fn viewport_width(mut self, value: u32) -> Self {
        self.viewport_width = Some(ConstrainULong::new(Some(value)));
        self
    }

    pub fn viewport_width_range(mut self, min: Option<u32>, max: Option<u32>, exact: Option<u32>, ideal: Option<u32>) -> Self {
        self.viewport_width = Some(Constrain::Range { min, max, exact, ideal });
        self
    }
}
