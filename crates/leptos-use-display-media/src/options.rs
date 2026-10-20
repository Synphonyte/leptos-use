use crate::MaybeRwSignal;
use default_struct_builder::DefaultBuilder;

// NOTE: there's no video value because it has to be `true`. Otherwise the stream would always resolve to an Error.
/// Options for [`use_display_media`](crate::use_display_media).
#[derive(DefaultBuilder, Clone, Copy, Debug)]
pub struct UseDisplayMediaOptions {
    /// If the stream is enabled. Defaults to `false`.
    pub(crate) enabled: MaybeRwSignal<bool>,

    /// A value of `true` indicates that the returned [`MediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)
    /// will contain an audio track, if audio is supported and available for the display surface chosen by the user.
    /// The default value is `false`.
    pub(crate) audio: bool,
}

impl Default for UseDisplayMediaOptions {
    fn default() -> Self {
        Self {
            enabled: false.into(),
            audio: false,
        }
    }
}
