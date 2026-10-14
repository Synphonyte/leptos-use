use leptos::prelude::*;

/// Return type of [`fn@crate::use_screen_orientation`].
pub struct UseScreenOrientationReturn<LFn, UFn>
where
    LFn: Fn(ScreenOrientationLock) + Clone + Send + Sync + 'static,
    UFn: Fn() + Clone + Send + Sync + 'static,
{
    /// Current screen orientation.
    pub orientation: Signal<ScreenOrientation>,

    /// The document's current orientation angle.
    pub angle: Signal<u16>,

    /// Locks the screen orientation to the specified orientation.
    ///
    /// Typically orientation locking is only enabled on mobile devices,
    /// and when the browser context is full screen.
    pub lock_orientation: LFn,

    /// Unlocks the screen orientation.
    pub unlock_orientation: UFn,
}

/// Represents the possible screen orientations. Returned by [`UseScreenOrientationReturn::orientation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenOrientation {
    /// The "primary" portrait mode.
    /// If the natural orientation is a portrait mode (screen height is greater than width),
    /// this will be the same as the natural orientation, and correspond to an angle of 0 degrees.
    /// If the natural orientation is a landscape mode,
    /// then the user agent can choose either portrait orientation as the `PortraitPrimary` and `PortraitSecondary`;
    /// one of those will be assigned the angle of 90 degrees and the other will have an angle of 270 degrees.
    PortraitPrimary,
    /// The secondary portrait orientation.
    /// If the natural orientation is a portrait mode, this will have an angle of 180 degrees
    /// (in other words, the device is upside down relative to its natural orientation).
    /// If the natural orientation is a landscape mode,
    /// this can be either orientation as selected by the user agent: whichever was not selected for `PortraitPrimary`.
    PortraitSecondary,
    /// The "primary" landscape mode.
    /// If the natural orientation is a landscape mode (screen width is greater than height),
    /// this will be the same as the natural orientation, and correspond to an angle of 0 degrees.
    /// If the natural orientation is a portrait mode,
    /// then the user agent can choose either landscape orientation as the `landscape-primary`
    /// with an angle of either 90 or 270 degrees (`LandscapeSecondary` will be the other orientation and angle).
    LandscapePrimary,
    /// The secondary landscape mode.
    /// If the natural orientation is a landscape mode,
    /// this orientation is upside down relative to the natural orientation, and will have an angle of 180 degrees.
    /// If the natural orientation is a portrait mode, this can be either orientation as selected by the user agent:
    /// whichever was not selected for `LandscapePrimary`.
    LandscapeSecondary,
}

/// Represents the possible screen orientation lock modes. Passed to [`UseScreenOrientationReturn::lock_orientation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenOrientationLock {
    /// Any of `PortraitPrimary`, `PortraitSecondary`, `LandscapePrimary` or `LandscapeSecondary`.
    Any,
    /// The natural orientation of the screen from the underlying operating system: either `PortraitPrimary` or `LandscapePrimary`.
    Natural,
    /// An orientation where screen width is greater than the screen height.
    /// Depending on the platform convention, this may be `LandscapePrimary`, `LandscapeSecondary`, or both.
    Landscape,
    /// An orientation where screen height is greater than the screen width.
    /// Depending on the platform convention, this may be `PortraitPrimary`, `PortraitSecondary`, or both.
    Portrait,
    /// The "primary" portrait mode.
    /// If the natural orientation is a portrait mode (screen height is greater than width),
    /// this will be the same as the natural orientation, and correspond to an angle of 0 degrees.
    /// If the natural orientation is a landscape mode,
    /// then the user agent can choose either portrait orientation as the `PortraitPrimary` and `PortraitSecondary`;
    /// one of those will be assigned the angle of 90 degrees and the other will have an angle of 270 degrees.
    PortraitPrimary,
    /// The secondary portrait orientation.
    /// If the natural orientation is a portrait mode, this will have an angle of 180 degrees
    /// (in other words, the device is upside down relative to its natural orientation).
    /// If the natural orientation is a landscape mode,
    /// this can be either orientation as selected by the user agent: whichever was not selected for `PortraitPrimary`.
    PortraitSecondary,
    /// The "primary" landscape mode.
    /// If the natural orientation is a landscape mode (screen width is greater than height),
    /// this will be the same as the natural orientation, and correspond to an angle of 0 degrees.
    /// If the natural orientation is a portrait mode,
    /// then the user agent can choose either landscape orientation as the `landscape-primary`
    /// with an angle of either 90 or 270 degrees (`LandscapeSecondary` will be the other orientation and angle).
    LandscapePrimary,
    /// The secondary landscape mode.
    /// If the natural orientation is a landscape mode,
    /// this orientation is upside down relative to the natural orientation, and will have an angle of 180 degrees.
    /// If the natural orientation is a portrait mode, this can be either orientation as selected by the user agent:
    /// whichever was not selected for `LandscapePrimary`.
    LandscapeSecondary,
}

#[cfg(not(feature = "ssr"))]
impl From<web_sys::OrientationType> for ScreenOrientation {
    fn from(value: web_sys::OrientationType) -> Self {
        match value {
            web_sys::OrientationType::PortraitPrimary => ScreenOrientation::PortraitPrimary,
            web_sys::OrientationType::PortraitSecondary => ScreenOrientation::PortraitSecondary,
            web_sys::OrientationType::LandscapePrimary => ScreenOrientation::LandscapePrimary,
            web_sys::OrientationType::LandscapeSecondary => ScreenOrientation::LandscapeSecondary,
            _ => unreachable!(),
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl From<ScreenOrientation> for web_sys::OrientationType {
    fn from(value: ScreenOrientation) -> Self {
        match value {
            ScreenOrientation::PortraitPrimary => web_sys::OrientationType::PortraitPrimary,
            ScreenOrientation::PortraitSecondary => web_sys::OrientationType::PortraitSecondary,
            ScreenOrientation::LandscapePrimary => web_sys::OrientationType::LandscapePrimary,
            ScreenOrientation::LandscapeSecondary => web_sys::OrientationType::LandscapeSecondary,
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl From<web_sys::OrientationLockType> for ScreenOrientationLock {
    fn from(value: web_sys::OrientationLockType) -> Self {
        match value {
            web_sys::OrientationLockType::Any => ScreenOrientationLock::Any,
            web_sys::OrientationLockType::Natural => ScreenOrientationLock::Natural,
            web_sys::OrientationLockType::Landscape => ScreenOrientationLock::Landscape,
            web_sys::OrientationLockType::Portrait => ScreenOrientationLock::Portrait,
            web_sys::OrientationLockType::PortraitPrimary => ScreenOrientationLock::PortraitPrimary,
            web_sys::OrientationLockType::PortraitSecondary => {
                ScreenOrientationLock::PortraitSecondary
            }
            web_sys::OrientationLockType::LandscapePrimary => {
                ScreenOrientationLock::LandscapePrimary
            }
            web_sys::OrientationLockType::LandscapeSecondary => {
                ScreenOrientationLock::LandscapeSecondary
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl From<ScreenOrientationLock> for web_sys::OrientationLockType {
    fn from(value: ScreenOrientationLock) -> Self {
        match value {
            ScreenOrientationLock::Any => web_sys::OrientationLockType::Any,
            ScreenOrientationLock::Natural => web_sys::OrientationLockType::Natural,
            ScreenOrientationLock::Landscape => web_sys::OrientationLockType::Landscape,
            ScreenOrientationLock::Portrait => web_sys::OrientationLockType::Portrait,
            ScreenOrientationLock::PortraitPrimary => web_sys::OrientationLockType::PortraitPrimary,
            ScreenOrientationLock::PortraitSecondary => {
                web_sys::OrientationLockType::PortraitSecondary
            }
            ScreenOrientationLock::LandscapePrimary => {
                web_sys::OrientationLockType::LandscapePrimary
            }
            ScreenOrientationLock::LandscapeSecondary => {
                web_sys::OrientationLockType::LandscapeSecondary
            }
        }
    }
}
