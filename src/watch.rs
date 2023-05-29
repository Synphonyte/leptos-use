use leptos::*;
use std::cell::RefCell;

/// A version of `create_effect` that listens to any dependency that is accessed inside `deps`.
/// Also a stop handler is returned.
/// If `immediate` is false, the `callback` will not run immediately but only after
/// the first change is detected of any signal that is accessed in `deps`.
/// The return value of `deps` is passed into `callback` as an argument together with the previous value
/// and the previous value that the `callback` itself returned last time.
///
/// # Usage
///
/// ```
/// # use std::time::Duration;
/// # use leptos::*;
/// # use leptos_use::watch;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let (num, set_num) = create_signal(cx, 0);
///
/// let stop = watch(
///     cx,
///     num,
///     move |num, _, _| {
///         log!("number {}", num);
///     },
///     true,
/// );
///
/// set_num(1); // > "number 1"
///
/// set_timeout_with_handle(move || {
///     stop(); // stop watching
///
///     set_num(2); // nothing happens
/// }, Duration::from_millis(1000));
/// #    view! { cx, }
/// # }
/// ```
pub fn watch<W, T, DFn, CFn>(
    cx: Scope,
    deps: DFn,
    callback: CFn,
    immediate: bool,
) -> impl Fn() + Clone
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + 'static,
    W: 'static,
    T: 'static,
{
    let (is_active, set_active) = create_signal(cx, true);

    let prev_deps_value: RefCell<Option<W>> = RefCell::new(None);
    let prev_callback_value: RefCell<Option<T>> = RefCell::new(None);

    create_effect(cx, move |did_run_before| {
        if !is_active() {
            return ();
        }

        let deps_value = deps();

        if !immediate && did_run_before.is_none() {
            prev_deps_value.replace(Some(deps_value));
            return ();
        }

        let callback_value = callback(
            &deps_value,
            prev_deps_value.borrow().as_ref(),
            prev_callback_value.take(),
        );
        prev_callback_value.replace(Some(callback_value));

        prev_deps_value.replace(Some(deps_value));

        ()
    });

    move || {
        set_active(false);
    }
}
