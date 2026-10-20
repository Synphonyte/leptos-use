use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_device_orientation`](crate::use_device_orientation).
#[derive(Clone)]
pub struct UseDeviceOrientationReturn {
    pub is_supported: Signal<bool>,
    pub absolute: Signal<bool>,
    pub alpha: Signal<Option<f64>>,
    pub beta: Signal<Option<f64>>,
    pub gamma: Signal<Option<f64>>,
}
