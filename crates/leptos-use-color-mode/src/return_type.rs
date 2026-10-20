use crate::ColorMode;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_color_mode`](crate::use_color_mode)
pub struct UseColorModeReturn {
    /// Main value signal of the color mode
    pub mode: Signal<ColorMode>,
    /// Main value setter signal of the color mode
    pub set_mode: WriteSignal<ColorMode>,

    /// Direct access to the returned signal of [`fn@crate::use_storage`] if enabled or [`UseColorModeOptions::storage_signal`](crate::UseColorModeOptions::storage_signal) if provided
    pub store: Signal<ColorMode>,
    /// Direct write access to the returned signal of [`fn@crate::use_storage`] if enabled or [`UseColorModeOptions::storage_signal`](crate::UseColorModeOptions::storage_signal) if provided
    pub set_store: WriteSignal<ColorMode>,

    /// Signal of the system's preferred color mode that you would get from a media query
    pub system: Signal<ColorMode>,

    /// When [`UseColorModeOptions::emit_auto`](crate::UseColorModeOptions::emit_auto) is `false` this is the same as `mode`. This will never report `ColorMode::Auto` but always on of the other modes.
    pub state: Signal<ColorMode>,
}
