use crate::{use_media_query, use_window};
use leptos::logging::error;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use paste::paste;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Return type of [`use_breakpoints`](crate::use_breakpoints)
#[derive(Clone)]
pub struct UseBreakpointsReturn<K: Eq + Hash + Debug + Clone + Send + Sync> {
    pub(crate) breakpoints: HashMap<K, u32>,
}

macro_rules! query_suffix {
    (>) => {
        ".1"
    };
    (<) => {
        ".9"
    };
    (=) => {
        ""
    };
}

macro_rules! value_expr {
    ($v:ident, >) => {
        $v
    };
    ($v:ident, <) => {
        $v - 1
    };
    ($v:ident, =) => {
        $v
    };
}

macro_rules! format_media_query {
    ($cmp:tt, $suffix:tt, $v:ident) => {
        format!(
            "({}-width: {}{}px)",
            $cmp,
            value_expr!($v, $suffix),
            query_suffix!($suffix)
        )
    };
}

macro_rules! impl_cmp_reactively {
    (   #[$attr:meta]
        $fn:ident, $cmp:tt, $suffix:tt) => {
        paste! {
            // Reactive check if
            #[$attr]
            pub fn $fn(&self, key: K) -> Signal<bool> {
                if let Some(value) = self.breakpoints.get(&key) {
                    use_media_query(format_media_query!($cmp, $suffix, value))
                } else {
                    self.not_found_signal(key)
                }
            }

            // Static check if
            #[$attr]
            pub fn [<is_ $fn>](&self, key: K) -> bool {
                if let Some(value) = self.breakpoints.get(&key) {
                    Self::match_(&format_media_query!($cmp, $suffix, value))
                } else {
                    self.not_found(key)
                }
            }
        }
    };
}

impl<K> UseBreakpointsReturn<K>
where
    K: Eq + Hash + Debug + Clone + Send + Sync + 'static,
{
    fn match_(query: &str) -> bool {
        if let Ok(Some(query_list)) = use_window().match_media(query) {
            return query_list.matches();
        }

        false
    }

    fn not_found_signal(&self, key: K) -> Signal<bool> {
        error!("Breakpoint \"{:?}\" not found", key);
        Signal::derive(|| false)
    }

    fn not_found(&self, key: K) -> bool {
        error!("Breakpoint \"{:?}\" not found", key);
        false
    }

    impl_cmp_reactively!(
        /// `[screen size]` > `key`
        gt, "min", >
    );
    impl_cmp_reactively!(
        /// `[screen size]` >= `key`
        ge, "min", =
    );
    impl_cmp_reactively!(
        /// `[screen size]` < `key`
        lt, "max", <
    );
    impl_cmp_reactively!(
        /// `[screen size]` <= `key`
        le, "max", =
    );

    fn between_media_query(min: &u32, max: &u32) -> String {
        format!("(min-width: {min}px) and (max-width: {}.9px)", max - 1)
    }

    /// Reactive check if `min_key` <= `[screen size]` < `max_key`
    pub fn between(&self, min_key: K, max_key: K) -> Signal<bool> {
        if let Some(min) = self.breakpoints.get(&min_key) {
            if let Some(max) = self.breakpoints.get(&max_key) {
                use_media_query(Self::between_media_query(min, max))
            } else {
                self.not_found_signal(max_key)
            }
        } else {
            self.not_found_signal(min_key)
        }
    }

    /// Static check if `min_key` <= `[screen size]` < `max_key`
    pub fn is_between(&self, min_key: K, max_key: K) -> bool {
        if let Some(min) = self.breakpoints.get(&min_key) {
            if let Some(max) = self.breakpoints.get(&max_key) {
                Self::match_(&Self::between_media_query(min, max))
            } else {
                self.not_found(max_key)
            }
        } else {
            self.not_found(min_key)
        }
    }

    /// Reactive Vec of all breakpoints that fulfill `[screen size]` >= `key`
    pub fn current(&self) -> Signal<Vec<K>> {
        let breakpoints = self.breakpoints.clone();
        let keys: Vec<_> = breakpoints.keys().cloned().collect();

        let ge = move |key: &K| {
            let value = breakpoints
                .get(key)
                .expect("only used with keys() from the HashMap");

            use_media_query(format_media_query!("min", =, value))
        };

        let signals: Vec<_> = keys.iter().map(ge.clone()).collect();

        Signal::derive(move || {
            keys.iter()
                .cloned()
                .zip(signals.iter().cloned())
                .filter_map(|(key, signal)| signal.get().then_some(key))
                .collect::<Vec<_>>()
        })
    }
}
