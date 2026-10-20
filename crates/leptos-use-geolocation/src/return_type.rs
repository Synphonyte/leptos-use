use crate::OptionLocalSignal;
use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_geolocation`](crate::use_geolocation).
pub struct UseGeolocationReturn<ResumeFn, PauseFn>
where
    ResumeFn: Fn() + Clone + Send + Sync,
    PauseFn: Fn() + Clone + Send + Sync,
{
    /// The coordinates of the current device like latitude and longitude.
    /// See [`GeolocationCoordinates`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)..
    pub coords: OptionLocalSignal<web_sys::Coordinates>,

    /// The timestamp of the current coordinates.
    pub located_at: Signal<Option<f64>>,

    /// The last error received from `navigator.geolocation`.
    pub error: OptionLocalSignal<web_sys::PositionError>,

    /// Resume the geolocation watch.
    pub resume: ResumeFn,

    /// Pause the geolocation watch.
    pub pause: PauseFn,
}
