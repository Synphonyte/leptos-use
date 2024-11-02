use crate::core::UseRwSignal;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::rc::Rc;

/// Two-way Signals synchronization.
///
/// > Note: Please consider first if you can achieve your goals with the
/// > ["Good Options" described in the Leptos book](https://book.leptos.dev/reactivity/working_with_signals.html#making-signals-depend-on-each-other)
/// > Only if you really have to, use this function. This is, in effect, the
/// > ["If you really must..." option](https://book.leptos.dev/reactivity/working_with_signals.html#if-you-really-must).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/sync_signal)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::sync_signal;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(1);
/// let (b, set_b) = signal(2);
///
/// let stop = sync_signal((a, set_a), (b, set_b));
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 1, b: 1
///
/// set_b.set(3);
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 3, b: 3
///
/// set_a.set(4);
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 4, b: 4
/// #
/// # view! { }
/// # }
/// ```
///
/// ### `RwSignal`
///
/// You can mix and match `RwSignal`s and `Signal`-`WriteSignal` pairs.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::sync_signal;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(1);
/// let (b, set_b) = signal(2);
/// let c_rw = RwSignal::new(3);
/// let d_rw = RwSignal::new(4);
///
/// sync_signal((a, set_a), c_rw);
/// sync_signal(d_rw, (b, set_b));
/// sync_signal(c_rw, d_rw);
///
/// #
/// # view! { }
/// # }
/// ```
///
/// ### One directional
///
/// You can synchronize a signal only from left to right or right to left.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions, SyncDirection};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(1);
/// let (b, set_b) = signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::default().direction(SyncDirection::LeftToRight)
/// );
///
/// set_b.set(3); // doesn't sync
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 1, b: 3
///
/// set_a.set(4);
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 4, b: 4
/// #
/// # view! { }
/// # }
/// ```
///
/// ### Custom Transform
///
/// You can optionally provide custom transforms between the two signals.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(10);
/// let (b, set_b) = signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::with_transforms(
///         |left| *left * 2,
///         |right| *right / 2,
///     ),
/// );
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 10, b: 20
///
/// set_b.set(30);
///
/// log!("a: {}, b: {}", a.get(), b.get()); // a: 15, b: 30
/// #
/// # view! { }
/// # }
/// ```
///
/// #### Different Types
///
/// `SyncSignalOptions::default()` is only defined if the two signal types are identical.
/// Otherwise, you have to initialize the options with `with_transforms` or `with_assigns` instead
/// of `default`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions};
/// # use std::str::FromStr;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal("10".to_string());
/// let (b, set_b) = signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::with_transforms(
///         |left: &String| i32::from_str(left).unwrap_or_default(),
///         |right: &i32| right.to_string(),
///     ),
/// );
/// #
/// # view! { }
/// # }
/// ```
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions};
/// # use std::str::FromStr;
/// #
/// #[derive(Clone)]
/// pub struct Foo {
///     bar: i32,
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(Foo { bar: 10 });
/// let (b, set_b) = signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::with_assigns(
///         |b: &mut i32, a: &Foo| *b = a.bar,
///         |a: &mut Foo, b: &i32| a.bar = *b,
///     ),
/// );
/// #
/// # view! { }
/// # }
/// ```
pub fn sync_signal<T>(
    left: impl Into<UseRwSignal<T>>,
    right: impl Into<UseRwSignal<T>>,
) -> impl Fn() + Clone
where
    T: Clone + Send + Sync + 'static,
{
    sync_signal_with_options(left, right, SyncSignalOptions::<T, T>::default())
}

/// Version of [`sync_signal`] that takes a `SyncSignalOptions`. See [`sync_signal`] for how to use.
pub fn sync_signal_with_options<L, R>(
    left: impl Into<UseRwSignal<L>>,
    right: impl Into<UseRwSignal<R>>,
    options: SyncSignalOptions<L, R>,
) -> impl Fn() + Clone
where
    L: Clone + Send + Sync + 'static,
    R: Clone + Send + Sync + 'static,
{
    let SyncSignalOptions {
        immediate,
        direction,
        transforms,
    } = options;

    let (assign_ltr, assign_rtl) = transforms.assigns();

    let left = left.into();
    let right = right.into();

    let mut stop_watch_left = None;
    let mut stop_watch_right = None;

    let is_sync_update = StoredValue::new(false);

    if matches!(direction, SyncDirection::Both | SyncDirection::LeftToRight) {
        stop_watch_left = Some(Effect::watch(
            move || left.get(),
            move |new_value, _, _| {
                if !is_sync_update.get_value() {
                    is_sync_update.set_value(true);
                    right.try_update(|right| {
                        assign_ltr(right, new_value);
                    });
                } else {
                    is_sync_update.set_value(false);
                }
            },
            immediate,
        ));
    }

    if matches!(direction, SyncDirection::Both | SyncDirection::RightToLeft) {
        stop_watch_right = Some(Effect::watch(
            move || right.get(),
            move |new_value, _, _| {
                if !is_sync_update.get_value() {
                    is_sync_update.set_value(true);
                    left.try_update(|left| {
                        assign_rtl(left, new_value)
                    });
                } else {
                    is_sync_update.set_value(false);
                }
            },
            immediate,
        ));
    }

    move || {
        if let Some(stop_watch_left) = &stop_watch_left {
            stop_watch_left.stop();
        }
        if let Some(stop_watch_right) = &stop_watch_right {
            stop_watch_right.stop();
        }
    }
}

/// Direction of syncing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SyncDirection {
    LeftToRight,
    RightToLeft,
    #[default]
    Both,
}

pub type AssignFn<T, S> = Rc<dyn Fn(&mut T, &S)>;

/// Transforms or assigns for syncing.
pub enum SyncTransforms<L, R> {
    /// Transform the signal into each other by calling the transform functions.
    /// The values are then simply assigned.
    Transforms {
        /// Transforms the left signal into the right signal.
        ltr: Rc<dyn Fn(&L) -> R>,
        /// Transforms the right signal into the left signal.
        rtl: Rc<dyn Fn(&R) -> L>,
    },

    /// Assign the signals to each other. Instead of using `=` to assign the signals,
    /// these functions are called.
    Assigns {
        /// Assigns the left signal to the right signal.
        ltr: AssignFn<R, L>,
        /// Assigns the right signal to the left signal.
        rtl: AssignFn<L, R>,
    },
}

impl<T> Default for SyncTransforms<T, T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::Assigns {
            ltr: Rc::new(|right, left| *right = left.clone()),
            rtl: Rc::new(|left, right| *left = right.clone()),
        }
    }
}

impl<L, R> SyncTransforms<L, R>
where
    L: 'static,
    R: 'static,
{
    /// Returns assign functions for both directions that respect the value of this enum.
    pub fn assigns(&self) -> (AssignFn<R, L>, AssignFn<L, R>) {
        match self {
            SyncTransforms::Transforms { ltr, rtl } => {
                let ltr = Rc::clone(ltr);
                let rtl = Rc::clone(rtl);
                (
                    Rc::new(move |right, left| *right = ltr(left)),
                    Rc::new(move |left, right| *left = rtl(right)),
                )
            }
            SyncTransforms::Assigns { ltr, rtl } => (Rc::clone(ltr), Rc::clone(rtl)),
        }
    }
}

/// Options for [`sync_signal_with_options`].
#[derive(DefaultBuilder)]
pub struct SyncSignalOptions<L, R> {
    /// If `true`, the signals will be immediately synced when this function is called.
    /// If `false`, a signal is only updated when the other signal's value changes.
    /// Defaults to `true`.
    immediate: bool,

    /// Direction of syncing. Defaults to `SyncDirection::Both`.
    direction: SyncDirection,

    /// How to transform or assign the values to each other
    /// If `L` and `R` are identical this defaults to the simple `=` operator. If the types are
    /// not the same, then you have to choose to either use [`SyncSignalOptions::with_transforms`]
    /// or [`SyncSignalOptions::with_assigns`].
    #[builder(skip)]
    transforms: SyncTransforms<L, R>,
}

impl<L, R> SyncSignalOptions<L, R> {
    /// Initializes options with transforms functions that convert the signals into each other.
    pub fn with_transforms(
        transform_ltr: impl Fn(&L) -> R + 'static,
        transform_rtl: impl Fn(&R) -> L + 'static,
    ) -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transforms: SyncTransforms::Transforms {
                ltr: Rc::new(transform_ltr),
                rtl: Rc::new(transform_rtl),
            },
        }
    }

    /// Initializes options with assign functions that replace the default `=` operator.
    pub fn with_assigns(
        assign_ltr: impl Fn(&mut R, &L) + 'static,
        assign_rtl: impl Fn(&mut L, &R) + 'static,
    ) -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transforms: SyncTransforms::Assigns {
                ltr: Rc::new(assign_ltr),
                rtl: Rc::new(assign_rtl),
            },
        }
    }
}

impl<T> Default for SyncSignalOptions<T, T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            immediate: true,
            direction: Default::default(),
            transforms: Default::default(),
        }
    }
}
