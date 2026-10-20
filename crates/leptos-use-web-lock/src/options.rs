use default_struct_builder::DefaultBuilder;
use web_sys::LockMode;

/// Options for [`fn@crate::use_web_lock_with_options`].
#[allow(dead_code)]
#[derive(DefaultBuilder)]
pub struct UseWebLockOptions {
    /// The default mode is `LockMode::Exclusive`, but `LockMode::Shared` can be specified.
    /// There can be only one `Exclusive` holder of a lock, but multiple `Shared` requests can be
    /// granted at the same time. This can be used to implement the
    /// [readers-writer pattern](https://en.wikipedia.org/wiki/Readers%E2%80%93writer_lock).
    mode: LockMode,

    /// If `true`, the lock request will fail if the lock cannot be granted immediately without
    /// waiting. The callback is invoked with `null`. Defaults to `false`.
    if_available: bool,

    /// If `true`, then any held locks with the same name will be released, and the request will
    /// be granted, preempting any queued requests for it. Defaults to `false`.
    steal: bool,
    // TODO : add abort signal (this also requires to create a wrapper for AbortSignal similar to UseWindow)
}

#[cfg(not(feature = "ssr"))]
impl UseWebLockOptions {
    pub(crate) fn to_web_sys(&self) -> web_sys::LockOptions {
        let options = web_sys::LockOptions::new();
        options.set_mode(self.mode);
        options.set_if_available(self.if_available);
        options.set_steal(self.steal);

        options
    }
}

impl Default for UseWebLockOptions {
    fn default() -> Self {
        Self {
            mode: LockMode::Exclusive,
            if_available: false,
            steal: false,
        }
    }
}
