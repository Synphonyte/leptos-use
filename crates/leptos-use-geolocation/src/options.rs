use default_struct_builder::DefaultBuilder;

/// Options for [`use_geolocation_with_options`](crate::use_geolocation_with_options).
#[derive(DefaultBuilder, Clone)]
#[allow(dead_code)]
pub struct UseGeolocationOptions {
    /// If `true` the geolocation watch is started when this function is called.
    /// If `false` you have to call `resume` manually to start it. Defaults to `true`.
    pub(crate) immediate: bool,

    /// A boolean value that indicates the application would like to receive the best
    /// possible results. If `true` and if the device is able to provide a more accurate
    /// position, it will do so. Note that this can result in slower response times or
    /// increased power consumption (with a GPS chip on a mobile device for example).
    /// On the other hand, if `false`, the device can take the liberty to save
    /// resources by responding more quickly and/or using less power. Default: `false`.
    enable_high_accuracy: bool,

    /// A positive value indicating the maximum age in milliseconds of a possible cached position that is acceptable to return.
    /// If set to `0`, it means that the device cannot use a cached position and must attempt to retrieve the real current position.
    /// Default: 30000.
    maximum_age: u32,

    /// A positive value representing the maximum length of time (in milliseconds)
    /// the device is allowed to take in order to return a position.
    /// The default value is 27000.
    timeout: u32,
}

impl Default for UseGeolocationOptions {
    fn default() -> Self {
        Self {
            enable_high_accuracy: false,
            maximum_age: 30000,
            timeout: 27000,
            immediate: true,
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl UseGeolocationOptions {
    pub(crate) fn as_position_options(&self) -> web_sys::PositionOptions {
        let UseGeolocationOptions {
            enable_high_accuracy,
            maximum_age,
            timeout,
            ..
        } = self;

        let options = web_sys::PositionOptions::new();
        options.set_enable_high_accuracy(*enable_high_accuracy);
        options.set_maximum_age(*maximum_age);
        options.set_timeout(*timeout);

        options
    }
}
