use crate::{ConstraintRangeBuilder, IntoDeviceIds, MaybeRwSignal};
use default_struct_builder::DefaultBuilder;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

/// Options for [`use_user_media_with_options`](crate::use_user_media_with_options).
///
/// Either or both constraints must be specified.
/// If the browser cannot find all media tracks with the specified types that meet the constraints given,
/// then the returned promise is rejected with `NotFoundError`
#[derive(DefaultBuilder, Clone, Debug)]
pub struct UseUserMediaOptions {
    /// If the stream is enabled. Defaults to `false`.
    pub(crate) enabled: MaybeRwSignal<bool>,
    /// Constraint parameter describing video media type requested
    /// The default value is `true`.
    #[builder(into)]
    pub(crate) video: VideoConstraints,
    /// Constraint parameter describing audio media type requested
    /// The default value is `false`.
    #[builder(into)]
    pub(crate) audio: AudioConstraints,
}

impl Default for UseUserMediaOptions {
    fn default() -> Self {
        Self {
            enabled: false.into(),
            video: true.into(),
            audio: false.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConstraintExactIdeal<T> {
    Single(Option<T>),
    ExactIdeal { exact: Option<T>, ideal: Option<T> },
}

impl<T> Default for ConstraintExactIdeal<T>
where
    T: Default,
{
    fn default() -> Self {
        ConstraintExactIdeal::Single(Some(T::default()))
    }
}

impl<T> ConstraintExactIdeal<T> {
    pub fn exact(mut self, value: T) -> Self {
        if let ConstraintExactIdeal::ExactIdeal { exact: e, .. } = &mut self {
            *e = Some(value);
        }

        self
    }

    pub fn ideal(mut self, value: T) -> Self {
        if let ConstraintExactIdeal::ExactIdeal { ideal: i, .. } = &mut self {
            *i = Some(value);
        }

        self
    }
}

impl<T> ConstraintExactIdeal<T>
where
    T: Into<JsValue> + Clone,
{
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            ConstraintExactIdeal::Single(value) => value.clone().unwrap().into(),
            ConstraintExactIdeal::ExactIdeal { exact, ideal } => {
                let obj = Object::new();

                if let Some(value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &value.clone().into()).unwrap();
                }
                if let Some(value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &value.clone().into()).unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

impl From<&'static str> for ConstraintExactIdeal<&'static str> {
    fn from(value: &'static str) -> Self {
        ConstraintExactIdeal::Single(Some(value))
    }
}

#[derive(Clone, Debug)]
pub enum ConstraintBoolOrRange<T> {
    Bool(bool),
    Single(Option<T>),
    Range {
        min: Option<T>,
        max: Option<T>,
        exact: Option<T>,
        ideal: Option<T>,
    },
}

impl<T> ConstraintBoolOrRange<T>
where
    T: Into<JsValue> + Clone,
{
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            Self::Bool(value) => JsValue::from_bool(*value),
            Self::Single(value) => value.clone().unwrap().into(),
            Self::Range {
                min,
                max,
                exact,
                ideal,
            } => {
                let obj = Object::new();

                if let Some(min_value) = min {
                    Reflect::set(&obj, &JsValue::from_str("min"), &min_value.clone().into())
                        .unwrap();
                }
                if let Some(max_value) = max {
                    Reflect::set(&obj, &JsValue::from_str("max"), &max_value.clone().into())
                        .unwrap();
                }
                if let Some(value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &value.clone().into()).unwrap();
                }
                if let Some(value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &value.clone().into()).unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

impl<T: Default> Default for ConstraintBoolOrRange<T> {
    fn default() -> Self {
        ConstraintBoolOrRange::Single(Some(T::default()))
    }
}

impl<T> From<bool> for ConstraintBoolOrRange<T> {
    fn from(value: bool) -> Self {
        ConstraintBoolOrRange::Bool(value)
    }
}

impl<T> From<ConstraintRange<T>> for ConstraintBoolOrRange<T> {
    fn from(value: ConstraintRange<T>) -> Self {
        match value {
            ConstraintRange::Single(value) => ConstraintBoolOrRange::Single(value),
            ConstraintRange::Range {
                min,
                max,
                exact,
                ideal,
            } => ConstraintBoolOrRange::Range {
                min,
                max,
                exact,
                ideal,
            },
        }
    }
}

impl From<f64> for ConstraintBoolOrRange<f64> {
    fn from(value: f64) -> Self {
        Self::Single(Some(value))
    }
}

impl From<u32> for ConstraintBoolOrRange<u32> {
    fn from(value: u32) -> Self {
        Self::Single(Some(value))
    }
}

#[derive(Clone, Debug)]
pub enum ConstraintRange<T> {
    Single(Option<T>),
    Range {
        min: Option<T>,
        max: Option<T>,
        exact: Option<T>,
        ideal: Option<T>,
    },
}

impl<T> Default for ConstraintRange<T>
where
    T: Default,
{
    fn default() -> Self {
        ConstraintRange::Single(Some(T::default()))
    }
}

impl<T> ConstraintRange<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new(value: Option<T>) -> Self {
        ConstraintRange::Single(value)
    }
}

macro_rules! impl_constraint_range_builder {
    ($ty:ty) => {
        impl<T> ConstraintRangeBuilder<T> for $ty {
            fn min(mut self, value: T) -> Self {
                if let Self::Range { ref mut min, .. } = self {
                    *min = Some(value);
                }
                self
            }

            fn max(mut self, value: T) -> Self {
                if let Self::Range { ref mut max, .. } = self {
                    *max = Some(value);
                }
                self
            }

            fn exact(mut self, value: T) -> Self {
                if let Self::Range { exact, .. } = &mut self {
                    *exact = Some(value);
                }

                self
            }

            fn ideal(mut self, value: T) -> Self {
                if let Self::Range { ideal, .. } = &mut self {
                    *ideal = Some(value);
                }

                self
            }
        }
    };
}

impl_constraint_range_builder!(ConstraintRange<T>);

impl_constraint_range_builder!(ConstraintBoolOrRange<T>);

impl<T> ConstraintRange<T>
where
    T: Into<JsValue> + Clone,
{
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            ConstraintRange::Single(value) => value.clone().unwrap().into(),
            ConstraintRange::Range {
                min,
                max,
                exact,
                ideal,
            } => {
                let obj = Object::new();

                if let Some(min_value) = min {
                    Reflect::set(&obj, &JsValue::from_str("min"), &min_value.clone().into())
                        .unwrap();
                }
                if let Some(max_value) = max {
                    Reflect::set(&obj, &JsValue::from_str("max"), &max_value.clone().into())
                        .unwrap();
                }
                if let Some(value) = exact {
                    Reflect::set(&obj, &JsValue::from_str("exact"), &value.clone().into()).unwrap();
                }
                if let Some(value) = ideal {
                    Reflect::set(&obj, &JsValue::from_str("ideal"), &value.clone().into()).unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

impl From<f64> for ConstraintDouble {
    fn from(value: f64) -> Self {
        ConstraintRange::Single(Some(value))
    }
}

impl From<u32> for ConstraintULong {
    fn from(value: u32) -> Self {
        ConstraintRange::Single(Some(value))
    }
}

pub type ConstraintBool = ConstraintExactIdeal<bool>;

impl From<bool> for ConstraintBool {
    fn from(value: bool) -> Self {
        ConstraintExactIdeal::Single(Some(value))
    }
}

pub type ConstraintDouble = ConstraintRange<f64>;

pub type ConstraintULong = ConstraintRange<u32>;

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

pub type ConstraintFacingMode = ConstraintExactIdeal<FacingMode>;

impl From<FacingMode> for ConstraintFacingMode {
    fn from(value: FacingMode) -> Self {
        ConstraintFacingMode::Single(Some(value))
    }
}

impl ConstraintFacingMode {
    pub fn to_jsvalue(&self) -> JsValue {
        match self {
            ConstraintExactIdeal::Single(value) => JsValue::from_str((*value).unwrap().as_str()),
            ConstraintExactIdeal::ExactIdeal { exact, ideal } => {
                let obj = Object::new();

                if let Some(value) = exact {
                    Reflect::set(
                        &obj,
                        &JsValue::from_str("exact"),
                        &JsValue::from_str(value.as_str()),
                    )
                    .unwrap();
                }
                if let Some(value) = ideal {
                    Reflect::set(
                        &obj,
                        &JsValue::from_str("ideal"),
                        &JsValue::from_str(value.as_str()),
                    )
                    .unwrap();
                }

                JsValue::from(obj)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AudioConstraints {
    Bool(bool),
    Constraints(Box<AudioTrackConstraints>),
}

impl From<bool> for AudioConstraints {
    fn from(value: bool) -> Self {
        AudioConstraints::Bool(value)
    }
}

impl From<AudioTrackConstraints> for AudioConstraints {
    fn from(value: AudioTrackConstraints) -> Self {
        AudioConstraints::Constraints(Box::new(value))
    }
}

#[derive(Clone, Debug)]
pub enum VideoConstraints {
    Bool(bool),
    Constraints(Box<VideoTrackConstraints>),
}

impl From<bool> for VideoConstraints {
    fn from(value: bool) -> Self {
        VideoConstraints::Bool(value)
    }
}

impl From<VideoTrackConstraints> for VideoConstraints {
    fn from(value: VideoTrackConstraints) -> Self {
        VideoConstraints::Constraints(Box::new(value))
    }
}

#[derive(DefaultBuilder, Default, Clone, Debug)]
#[allow(dead_code)]
pub struct AudioTrackConstraints {
    #[builder(skip)]
    pub(crate) device_id: Vec<String>,

    #[builder(into)]
    pub(crate) auto_gain_control: Option<ConstraintBool>,
    #[builder(into)]
    pub(crate) channel_count: Option<ConstraintULong>,
    #[builder(into)]
    pub(crate) echo_cancellation: Option<ConstraintBool>,
    #[builder(into)]
    pub(crate) noise_suppression: Option<ConstraintBool>,
}

impl AudioTrackConstraints {
    pub fn new() -> Self {
        AudioTrackConstraints::default()
    }

    pub fn device_id<M>(mut self, value: impl IntoDeviceIds<M>) -> Self {
        self.device_id = value.into_device_ids();
        self
    }
}

#[derive(DefaultBuilder, Default, Clone, Debug)]
pub struct VideoTrackConstraints {
    #[builder(skip)]
    pub device_id: Vec<String>,

    #[builder(into)]
    pub facing_mode: Option<ConstraintFacingMode>,
    #[builder(into)]
    pub frame_rate: Option<ConstraintDouble>,
    #[builder(into)]
    pub height: Option<ConstraintULong>,
    #[builder(into)]
    pub width: Option<ConstraintULong>,
    #[builder(into)]
    pub viewport_offset_x: Option<ConstraintULong>,
    #[builder(into)]
    pub viewport_offset_y: Option<ConstraintULong>,
    #[builder(into)]
    pub viewport_height: Option<ConstraintULong>,
    #[builder(into)]
    pub viewport_width: Option<ConstraintULong>,
    #[builder(into)]
    pub zoom: Option<ConstraintBoolOrRange<f64>>,
}

impl VideoTrackConstraints {
    pub fn new() -> Self {
        VideoTrackConstraints::default()
    }

    pub fn device_id<M>(mut self, value: impl IntoDeviceIds<M>) -> Self {
        self.device_id = value.into_device_ids();
        self
    }
}
