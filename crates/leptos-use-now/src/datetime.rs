use cfg_if::cfg_if;

/// SSR safe `Date.now()`.
#[inline(always)]
pub fn now() -> f64 {
    cfg_if! { if #[cfg(feature = "ssr")] {
        use std::time::{SystemTime, UNIX_EPOCH};

        // If the system clock is at or before the Unix epoch we degrade to 0
        // instead of panicking.
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64
    } else {
        js_sys::Date::now()
    }}
}
