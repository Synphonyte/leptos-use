use crate::{use_media_query, use_window};
use leptos::logging::error;
use leptos::reactive_graph::wrappers::read::Signal;
use leptos::prelude::*;
use paste::paste;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Reactive viewport breakpoints.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_breakpoints)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_breakpoints, BreakpointsTailwind, breakpoints_tailwind};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// let screen_width = use_breakpoints(breakpoints_tailwind());
///
/// use BreakpointsTailwind::*;
///
/// let sm_and_larger = screen_width.ge(Sm);
/// let larger_than_sm = screen_width.gt(Sm);
/// let lg_and_smaller = screen_width.le(Lg);
/// let smaller_than_lg = screen_width.lt(Lg);
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Breakpoints
///
/// There are many predefined breakpoints for major UI frameworks. The following are provided.
///
/// * [`breakpoints_tailwind`]
/// * [`breakpoints_bootstrap_v5`]
/// * [`breakpoints_material`]
/// * [`breakpoints_ant_design`]
/// * [`breakpoints_quasar`]
/// * [`breakpoints_semantic`]
/// * [`breakpoints_master_css`]
///
/// You can also provide your own breakpoints.
///
/// ```
/// # use std::collections::HashMap;
/// use leptos::prelude::*;
/// # use leptos_use::use_breakpoints;
/// #
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum MyBreakpoints {
///     Tablet,
///     Laptop,
///     Desktop,
/// }
///
/// fn my_breakpoints() -> HashMap<MyBreakpoints, u32> {
///     use MyBreakpoints::*;
///
///     HashMap::from([
///         (Tablet, 640),
///         (Laptop, 1024),
///         (Desktop, 1280),
///     ])
/// }
///
/// #[component]
/// fn Demo() -> impl IntoView {
///     let screen_width = use_breakpoints(my_breakpoints());
///
///     use MyBreakpoints::*;
///
///     let laptop = screen_width.between(Laptop, Desktop);
///
///     view! { }
/// }
/// ```
///
/// ## Non-reactive methods
///
/// For every reactive method there is also a non-reactive variant that is prefixed with `is_`
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_breakpoints, BreakpointsTailwind, breakpoints_tailwind};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// let screen_width = use_breakpoints(breakpoints_tailwind());
///
/// use BreakpointsTailwind::*;
///
/// let sm_and_larger = screen_width.is_ge(Sm);
/// let larger_than_sm = screen_width.is_gt(Sm);
/// let lg_and_smaller = screen_width.is_le(Lg);
/// let smaller_than_lg = screen_width.is_lt(Lg);
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// Since internally this uses [`fn@crate::use_media_query`], which returns always `false` on the server,
/// the returned methods also will return `false`.
pub fn use_breakpoints<K: Eq + Hash + Debug + Clone + Send + Sync>(
    breakpoints: HashMap<K, u32>,
) -> UseBreakpointsReturn<K> {
    UseBreakpointsReturn { breakpoints }
}

/// Return type of [`use_breakpoints`]
#[derive(Clone)]
pub struct UseBreakpointsReturn<K: Eq + Hash + Debug + Clone + Send + Sync> {
    breakpoints: HashMap<K, u32>,
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

    /// Reactive check if `min_key` <= `[screen size]` <= `max_key`
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

    /// Static check if `min_key` <= `[screen size]` <= `max_key`
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

/// Breakpoint keys for Tailwind V2
///
/// See <https://tailwindcss.com/docs/breakpoints>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsTailwind {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

/// Breakpoint definitions for Tailwind V2
///
/// See <https://tailwindcss.com/docs/breakpoints>
pub fn breakpoints_tailwind() -> HashMap<BreakpointsTailwind, u32> {
    HashMap::from([
        (BreakpointsTailwind::Sm, 640),
        (BreakpointsTailwind::Md, 768),
        (BreakpointsTailwind::Lg, 1024),
        (BreakpointsTailwind::Xl, 1280),
        (BreakpointsTailwind::Xxl, 1536),
    ])
}

/// Breakpoint keys for Bootstrap V5
///
/// See <https://getbootstrap.com/docs/5.0/layout/breakpoints>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsBootstrapV5 {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

/// Breakpoint definitions for Bootstrap V5
///
/// <https://getbootstrap.com/docs/5.0/layout/breakpoints>
pub fn breakpoints_bootstrap_v5() -> HashMap<BreakpointsBootstrapV5, u32> {
    HashMap::from([
        (BreakpointsBootstrapV5::Sm, 576),
        (BreakpointsBootstrapV5::Md, 768),
        (BreakpointsBootstrapV5::Lg, 992),
        (BreakpointsBootstrapV5::Xl, 1200),
        (BreakpointsBootstrapV5::Xxl, 1400),
    ])
}

/// Breakpoint keys for Material UI V5
///
/// See <https://mui.com/material-ui/customization/breakpoints/>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsMaterial {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

/// Breakpoint definitions for Material UI V5
///
/// See <https://mui.com/material-ui/customization/breakpoints/>
pub fn breakpoints_material() -> HashMap<BreakpointsMaterial, u32> {
    HashMap::from([
        (BreakpointsMaterial::Xs, 1),
        (BreakpointsMaterial::Sm, 600),
        (BreakpointsMaterial::Md, 900),
        (BreakpointsMaterial::Lg, 1200),
        (BreakpointsMaterial::Xl, 1536),
    ])
}

/// Breakpoint keys for Ant Design
///
/// See <https://ant.design/components/layout/#breakpoint-width>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsAntDesign {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

/// Breakpoint definitions for Ant Design
///
/// See <https://ant.design/components/layout/#breakpoint-width>
pub fn breakpoints_ant_design() -> HashMap<BreakpointsAntDesign, u32> {
    HashMap::from([
        (BreakpointsAntDesign::Xs, 480),
        (BreakpointsAntDesign::Sm, 576),
        (BreakpointsAntDesign::Md, 768),
        (BreakpointsAntDesign::Lg, 992),
        (BreakpointsAntDesign::Xl, 1200),
        (BreakpointsAntDesign::Xxl, 1600),
    ])
}

/// Breakpoint keys for Quasar V2
///
/// See <https://quasar.dev/style/breakpoints>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsQuasar {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

/// Breakpoint definitions for Quasar V2
///
/// See <https://quasar.dev/style/breakpoints>
pub fn breakpoints_quasar() -> HashMap<BreakpointsQuasar, u32> {
    HashMap::from([
        (BreakpointsQuasar::Xs, 1),
        (BreakpointsQuasar::Sm, 600),
        (BreakpointsQuasar::Md, 1024),
        (BreakpointsQuasar::Lg, 1440),
        (BreakpointsQuasar::Xl, 1920),
    ])
}

/// Breakpoint keys for Semantic UI
///
/// See <https://semantic-ui.com/elements/container.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsSemantic {
    Mobile,
    Tablet,
    SmallMonitor,
    LargeMonitor,
}

/// Breakpoint definitions for Semantic UI
///
/// See <https://semantic-ui.com/elements/container.html>
pub fn breakpoints_semantic() -> HashMap<BreakpointsSemantic, u32> {
    HashMap::from([
        (BreakpointsSemantic::Mobile, 1),
        (BreakpointsSemantic::Tablet, 768),
        (BreakpointsSemantic::SmallMonitor, 992),
        (BreakpointsSemantic::LargeMonitor, 1200),
    ])
}

/// Breakpoint keys for Master CSS
///
/// See <https://docs.master.co/css/breakpoints>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointsMasterCss {
    Xxxs,
    Xxs,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
    Xxxxl,
}

/// Breakpoint definitions for Master CSS
///
/// See <https://docs.master.co/css/breakpoints>
pub fn breakpoints_master_css() -> HashMap<BreakpointsMasterCss, u32> {
    HashMap::from([
        (BreakpointsMasterCss::Xxxs, 360),
        (BreakpointsMasterCss::Xxs, 480),
        (BreakpointsMasterCss::Xs, 600),
        (BreakpointsMasterCss::Sm, 768),
        (BreakpointsMasterCss::Md, 1024),
        (BreakpointsMasterCss::Lg, 1280),
        (BreakpointsMasterCss::Xl, 1440),
        (BreakpointsMasterCss::Xxl, 1600),
        (BreakpointsMasterCss::Xxxl, 1920),
        (BreakpointsMasterCss::Xxxxl, 2560),
    ])
}
