use crate::core::UseRwSignal;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::rc::Rc;

/// Two-way Signals synchronization.
///
/// > Note: Please consider first if you can achieve your goals with the
/// > ["Good Options" described in the Leptos book](https://book.leptos.dev/reactivity/working_with_signals.html#making-signals-depend-on-each-other)
/// > firstly. Only if you really have to, use this function. This is in effect the
/// > ["If you really must..."](https://book.leptos.dev/reactivity/working_with_signals.html#if-you-really-must).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/sync_signal)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::sync_signal;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = create_signal(1);
/// let (b, set_b) = create_signal(2);
///
/// let stop = sync_signal((a, set_a), (b, set_b));
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 1, b: 1
///
/// set_b.set(3);
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 3, b: 3
///
/// set_a.set(4);
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 4, b: 4
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
/// # use leptos::*;
/// # use leptos_use::sync_signal;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = create_signal(1);
/// let (b, set_b) = create_signal(2);
/// let c_rw = create_rw_signal(3);
/// let d_rw = create_rw_signal(4);
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
/// # use leptos::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions, SyncDirection};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = create_signal(1);
/// let (b, set_b) = create_signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::default().direction(SyncDirection::LeftToRight)
/// );
///
/// set_b.set(3); // doesn't sync
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 1, b: 3
///
/// set_a.set(4);
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 4, b: 4
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
/// # use leptos::*;
/// # use leptos_use::{sync_signal_with_options, SyncSignalOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = create_signal(10);
/// let (b, set_b) = create_signal(2);
///
/// let stop = sync_signal_with_options(
///     (a, set_a),
///     (b, set_b),
///     SyncSignalOptions::default()
///         .transform_ltr(|left| *left * 2)
///         .transform_rtl(|right| *right / 2)
/// );
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 10, b: 20
///
/// set_b.set(30);
///
/// logging::log!("a: {}, b: {}", a.get(), b.get()); // a: 15, b: 30
/// #
/// # view! { }
/// # }
/// ```
///
/// #### Different Types
///
/// `SyncSignalOptions::default()` is only defined if the two signal types are identical or
/// implement `From` for each other. Otherwise, you have to initialize the options with
/// `with_transforms` instead of `default`.
///
/// ```
/// # use leptos_use::SyncSignalOptions;
/// # use std::str::FromStr;
/// #
/// let options = SyncSignalOptions::with_transforms(
///     |left: &String| i32::from_str(left).unwrap_or_default(),
///     |right: &i32| right.to_string(),
/// );
/// ```
///
pub fn sync_signal<T>(
    left: impl Into<UseRwSignal<T>>,
    right: impl Into<UseRwSignal<T>>,
) -> impl Fn() + Clone
where
    T: Clone + PartialEq + 'static,
{
    sync_signal_with_options(left, right, SyncSignalOptions::default())
}

/// Version of [`sync_signal`] that takes a `SyncSignalOptions`. See [`sync_signal`] for how to use.
pub fn sync_signal_with_options<L, R>(
    left: impl Into<UseRwSignal<L>>,
    right: impl Into<UseRwSignal<R>>,
    options: SyncSignalOptions<L, R>,
) -> impl Fn() + Clone
where
    L: Clone + PartialEq + 'static,
    R: Clone + PartialEq + 'static,
{
    let SyncSignalOptions {
        immediate,
        direction,
        transform_ltr,
        transform_rtl,
        assign_ltr,
        assign_rtl,
    } = options;

    let left = left.into();
    let right = right.into();

    let mut stop_watch_left = None;
    let mut stop_watch_right = None;

    if matches!(direction, SyncDirection::Both | SyncDirection::LeftToRight) {
        stop_watch_left = Some(watch(
            move || left.get(),
            move |new_value, _, _| {
                let new_value = (*transform_ltr)(new_value);

                if right.with_untracked(|right| right != &new_value) {
                    right.update(|right| assign_ltr(right, new_value));
                }
            },
            immediate,
        ));
    }

    if matches!(direction, SyncDirection::Both | SyncDirection::RightToLeft) {
        stop_watch_right = Some(watch(
            move || right.get(),
            move |new_value, _, _| {
                let new_value = (*transform_rtl)(new_value);

                if left.with_untracked(|left| left != &new_value) {
                    left.update(|left| assign_rtl(left, new_value));
                }
            },
            immediate,
        ));
    }

    move || {
        if let Some(stop_watch_left) = &stop_watch_left {
            stop_watch_left();
        }
        if let Some(stop_watch_right) = &stop_watch_right {
            stop_watch_right();
        }
    }
}

/// Direction of syncing.
pub enum SyncDirection {
    LeftToRight,
    RightToLeft,
    Both,
}

pub type AssignFn<T> = Rc<dyn Fn(&mut T, T)>;

/// Options for [`sync_signal_with_options`].
#[derive(DefaultBuilder)]
pub struct SyncSignalOptions<L, R> {
    /// If `true`, the signals will be immediately synced when this function is called.
    /// If `false`, a signal is only updated when the other signal's value changes.
    /// Defaults to `true`.
    immediate: bool,

    /// Direction of syncing. Defaults to `SyncDirection::Both`.
    direction: SyncDirection,

    /// Transforms the left signal into the right signal.
    /// Defaults to identity.
    #[builder(skip)]
    transform_ltr: Rc<dyn Fn(&L) -> R>,

    /// Transforms the right signal into the left signal.
    /// Defaults to identity.
    #[builder(skip)]
    transform_rtl: Rc<dyn Fn(&R) -> L>,

    /// Assigns the left signal to the right signal.
    /// Defaults to `*r = l`.
    #[builder(skip)]
    assign_ltr: AssignFn<R>,

    /// Assigns the right signal to the left signal.
    /// Defaults to `*l = r`.
    #[builder(skip)]
    assign_rtl: AssignFn<L>,
}

impl<L, R> SyncSignalOptions<L, R> {
    /// Transforms the left signal into the right signal.
    /// Defaults to identity.
    pub fn transform_ltr(self, transform_ltr: impl Fn(&L) -> R + 'static) -> Self {
        Self {
            transform_ltr: Rc::new(transform_ltr),
            ..self
        }
    }

    /// Transforms the right signal into the left signal.
    /// Defaults to identity.
    pub fn transform_rtl(self, transform_rtl: impl Fn(&R) -> L + 'static) -> Self {
        Self {
            transform_rtl: Rc::new(transform_rtl),
            ..self
        }
    }

    /// Assigns the left signal to the right signal.
    /// Defaults to `*r = l`.
    pub fn assign_ltr(self, assign_ltr: impl Fn(&mut R, R) + 'static) -> Self {
        Self {
            assign_ltr: Rc::new(assign_ltr),
            ..self
        }
    }

    /// Assigns the right signal to the left signal.
    /// Defaults to `*l = r`.
    pub fn assign_rtl(self, assign_rtl: impl Fn(&mut L, L) + 'static) -> Self {
        Self {
            assign_rtl: Rc::new(assign_rtl),
            ..self
        }
    }
    /// Initializes options with transforms
    pub fn with_transforms(
        transform_ltr: impl Fn(&L) -> R + 'static,
        transform_rtl: impl Fn(&R) -> L + 'static,
    ) -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transform_ltr: Rc::new(transform_ltr),
            transform_rtl: Rc::new(transform_rtl),
            assign_ltr: Rc::new(|right, left| *right = left),
            assign_rtl: Rc::new(|left, right| *left = right),
        }
    }
}

impl<L, R> Default for SyncSignalOptions<L, R>
where
    L: Clone + From<R>,
    R: Clone + From<L>,
{
    fn default() -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transform_ltr: Rc::new(|x| x.clone().into()),
            transform_rtl: Rc::new(|x| x.clone().into()),
            assign_ltr: Rc::new(|right, left| *right = left),
            assign_rtl: Rc::new(|left, right| *left = right),
        }
    }
}
