use crate::{MaybeRwSignal, UseCycleListOptions, UseCycleListReturn};
use leptos::prelude::*;

/// Cycle through a list of items.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_cycle_list)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// use leptos_use_cycle_list::{use_cycle_list, UseCycleListReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseCycleListReturn { state, next, prev, .. } = use_cycle_list(
///     vec!["Dog", "Cat", "Lizard", "Shark", "Whale", "Dolphin", "Octopus", "Seal"]
/// );
///
/// log!("{}", state.get()); // "Dog"
///
/// prev();
///
/// log!("{}", state.get()); // "Seal"
/// #
/// # view! { }
/// # }
/// ```
pub fn use_cycle_list<T, L>(
    list: L,
) -> UseCycleListReturn<
    T,
    impl Fn(usize) -> T + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn(i64) -> T + Clone + Send + Sync,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    L: Into<Signal<Vec<T>>>,
{
    use_cycle_list_with_options(list, UseCycleListOptions::default())
}

pub fn use_cycle_list_with_options<T, L>(
    list: L,
    options: UseCycleListOptions<T>,
) -> UseCycleListReturn<
    T,
    impl Fn(usize) -> T + Clone,
    impl Fn() + Clone,
    impl Fn() + Clone,
    impl Fn(i64) -> T + Clone,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    L: Into<Signal<Vec<T>>>,
{
    let UseCycleListOptions {
        initial_value,
        fallback_index,
        get_position,
    } = options;

    let list = list.into();

    let get_initial_value = {
        let list = list.get_untracked();
        let first = list.first().cloned();

        move || {
            if let Some(initial_value) = initial_value {
                initial_value
            } else {
                MaybeRwSignal::from(first.expect("The provided list shouldn't be empty"))
            }
        }
    };

    let (state, set_state) = get_initial_value().into_signal();

    let index = Signal::derive(move || {
        let index = get_position(&state.get(), &list.read());

        if let Some(index) = index {
            index
        } else {
            fallback_index
        }
    });

    let set = move |i: usize| {
        let length = list.read().len();

        if length == 0 {
            return state.get_untracked();
        }

        let index = i % length;
        let value = list.read()[index].clone();

        set_state.update({
            let value = value.clone();

            move |v| *v = value
        });

        value
    };

    let shift = move |delta: i64| {
        let length = list.read().len() as i64;

        if length == 0 {
            return state.get_untracked();
        }

        let i = index.get_untracked() as i64 + delta;
        let index = (i % length) + length;

        set(index as usize)
    };

    let next = move || {
        shift(1);
    };

    let prev = move || {
        shift(-1);
    };

    let _ = Effect::watch(move || list.get(), move |_, _, _| set(index.get()), false);

    UseCycleListReturn {
        state,
        set_state,
        index,
        set_index: set,
        next,
        prev,
        shift,
    }
}
