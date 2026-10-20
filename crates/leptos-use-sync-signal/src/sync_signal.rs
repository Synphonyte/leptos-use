use crate::{SyncDirection, SyncSignalOptions, UseRwSignal};
use leptos::prelude::*;
use std::ops::Deref;
#[cfg(feature = "ssr")]
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
/// # use leptos::logging::log;
/// # use leptos_use_sync_signal::sync_signal;
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
/// # use leptos_use_sync_signal::sync_signal;
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
/// # use leptos::logging::log;
/// # use leptos_use_sync_signal::{sync_signal_with_options, SyncSignalOptions, SyncDirection};
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
/// # use leptos::logging::log;
/// # use leptos_use_sync_signal::{sync_signal_with_options, SyncSignalOptions};
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
/// # use leptos_use_sync_signal::{sync_signal_with_options, SyncSignalOptions};
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
/// # use leptos_use_sync_signal::{sync_signal_with_options, SyncSignalOptions};
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
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the signals are not continuously synced. If the option `immediate` is `true`, the
/// signals are synced once initially. If the option `immediate` is `false`, then this function
/// does nothing.
pub fn sync_signal<LeftRead, RightRead, LeftWrite, RightWrite, T>(
    left: impl Into<UseRwSignal<LeftRead, LeftWrite, T>>,
    right: impl Into<UseRwSignal<RightRead, RightWrite, T>>,
) -> impl Fn() + Clone
where
    T: Clone + Send + Sync + 'static,
    LeftRead: Read + ReadUntracked + Track + Copy + Send + Sync + 'static,
    <LeftRead as Read>::Value: Deref<Target = T>,
    <LeftRead as ReadUntracked>::Value: Deref<Target = T>,
    LeftWrite: Write<Value = T> + Copy + 'static,
    RightRead: Read + ReadUntracked + Track + Copy + Send + Sync + 'static,
    <RightRead as Read>::Value: Deref<Target = T>,
    <RightRead as ReadUntracked>::Value: Deref<Target = T>,
    RightWrite: Write<Value = T> + Copy + 'static,
{
    sync_signal_with_options(left, right, SyncSignalOptions::<T, T>::default())
}

/// Version of [`sync_signal`] that takes a `SyncSignalOptions`. See [`sync_signal`] for how to use.
pub fn sync_signal_with_options<LeftRead, RightRead, LeftWrite, RightWrite, Left, Right>(
    left: impl Into<UseRwSignal<LeftRead, LeftWrite, Left>>,
    right: impl Into<UseRwSignal<RightRead, RightWrite, Right>>,
    options: SyncSignalOptions<Left, Right>,
) -> impl Fn() + Clone
where
    Left: Clone + Send + Sync + 'static,
    LeftRead: Read + ReadUntracked + Track + Copy + Send + Sync + 'static,
    <LeftRead as Read>::Value: Deref<Target = Left>,
    <LeftRead as ReadUntracked>::Value: Deref<Target = Left>,
    LeftWrite: Write<Value = Left> + Copy + 'static,
    Right: Clone + Send + Sync + 'static,
    RightRead: Read + ReadUntracked + Track + Copy + Send + Sync + 'static,
    <RightRead as Read>::Value: Deref<Target = Right>,
    <RightRead as ReadUntracked>::Value: Deref<Target = Right>,
    RightWrite: Write<Value = Right> + Copy + 'static,
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
        #[cfg(feature = "ssr")]
        {
            if immediate {
                let assign_ltr = Rc::clone(&assign_ltr);
                right.try_update(move |right| assign_ltr(right, &left.get_untracked()));
            }
        }

        stop_watch_left = Some(Effect::watch(
            move || left.get(),
            move |new_value, _, _| {
                if !is_sync_update.get_value() || !matches!(direction, SyncDirection::Both) {
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
        #[cfg(feature = "ssr")]
        {
            if immediate && matches!(direction, SyncDirection::RightToLeft) {
                let assign_rtl = Rc::clone(&assign_rtl);
                left.try_update(move |left| assign_rtl(left, &right.get_untracked()));
            }
        }

        stop_watch_right = Some(Effect::watch(
            move || right.get(),
            move |new_value, _, _| {
                if !is_sync_update.get_value() || !matches!(direction, SyncDirection::Both) {
                    is_sync_update.set_value(true);
                    left.try_update(|left| assign_rtl(left, new_value));
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
