use cfg_if::cfg_if;

/// SSR safe `Date.now()`.
#[inline(always)]
pub(crate) fn now() -> f64 {
    cfg_if! { if #[cfg(feature = "ssr")] {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as f64
    } else {
        js_sys::Date::now()
    }}
}
