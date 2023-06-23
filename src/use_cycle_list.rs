use crate::watch;
use default_struct_builder::DefaultBuilder;
use leptos::*;

/// Cycle through a list of items.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_cycle_list)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// use leptos_use::{use_cycle_list, UseCycleListReturn};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let UseCycleListReturn { state, next, prev, .. } = use_cycle_list(
///     cx,
///     vec!["Dog", "Cat", "Lizard", "Shark", "Whale", "Dolphin", "Octopus", "Seal"]
/// );
///
/// log!("{}", state()); // "Dog"
///
/// prev();
///
/// log!("{}", state()); // "Seal"
/// #
/// # view! { cx, }
/// # }
/// ```

pub fn use_cycle_list<T, L>(
    cx: Scope,
    list: L,
) -> UseCycleListReturn<
    T,
    impl Fn(usize) -> T + Clone,
    impl Fn() + Clone,
    impl Fn() + Clone,
    impl Fn(i64) -> T + Clone,
>
where
    T: Clone + PartialEq + 'static,
    L: Into<MaybeSignal<Vec<T>>>,
{
    use_cycle_list_with_options(cx, list, UseCycleListOptions::default())
}

pub fn use_cycle_list_with_options<T, L>(
    cx: Scope,
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
    T: Clone + PartialEq + 'static,
    L: Into<MaybeSignal<Vec<T>>>,
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
                initial_value.get()
            } else {
                first.expect("The provided list shouldn't be empty")
            }
        }
    };

    let (state, set_state) = create_signal(cx, get_initial_value());

    let index = {
        let list = list.clone();

        create_memo(cx, move |_| {
            list.with(|list| {
                let index = get_position(&state.get(), list);

                if let Some(index) = index {
                    index
                } else {
                    fallback_index
                }
            })
        })
    };

    let set = {
        let list = list.clone();

        move |i: usize| {
            list.with(|list| {
                let length = list.len();

                let index = i % length;
                let value = list[index].clone();

                set_state.update({
                    let value = value.clone();

                    move |v| *v = value
                });

                value
            })
        }
    };

    let shift = {
        let list = list.clone();
        let set = set.clone();

        move |delta: i64| {
            let index = list.with(|list| {
                let length = list.len() as i64;

                let i = index.get_untracked() as i64 + delta;
                (i % length) + length
            });

            set(index as usize)
        }
    };

    let next = {
        let shift = shift.clone();

        move || {
            shift(1);
        }
    };

    let prev = {
        let shift = shift.clone();

        move || {
            shift(-1);
        }
    };

    let _ = {
        let set = set.clone();

        watch(cx, move || list.get(), move |_, _, _| set(index.get()))
    };

    UseCycleListReturn {
        state: state.into(),
        set_state,
        index: index.into(),
        set_index: set,
        next,
        prev,
        shift,
    }
}

/// Options for [`use_cycle_list_with_options`].
#[derive(DefaultBuilder)]
pub struct UseCycleListOptions<T>
where
    T: Clone + PartialEq + 'static,
{
    /// The initial value of the state. Can be a Signal. If none is provided the first entry
    /// of the list will be used.
    #[builder(keep_type)]
    initial_value: Option<MaybeSignal<T>>,

    /// The default index when the current value is not found in the list.
    /// For example when `get_index_of` returns `None`.
    fallback_index: usize,

    /// Custom function to get the index of the current value. Defaults to `Iterator::position()`
    #[builder(keep_type)]
    get_position: fn(&T, &Vec<T>) -> Option<usize>,
}

impl<T> Default for UseCycleListOptions<T>
where
    T: Clone + PartialEq + 'static,
{
    fn default() -> Self {
        Self {
            initial_value: None,
            fallback_index: 0,
            get_position: |value: &T, list: &Vec<T>| list.iter().position(|v| v == value),
        }
    }
}

/// Return type of [`use_cycle_list`].
pub struct UseCycleListReturn<T, SetFn, NextFn, PrevFn, ShiftFn>
where
    T: Clone + PartialEq + 'static,
    SetFn: Fn(usize) -> T + Clone,
    NextFn: Fn() + Clone,
    PrevFn: Fn() + Clone,
    ShiftFn: Fn(i64) -> T + Clone,
{
    /// Current value
    pub state: Signal<T>,
    /// Set current value
    pub set_state: WriteSignal<T>,
    /// Current index of current value in list
    pub index: Signal<usize>,
    /// Set current index of current value in list
    pub set_index: SetFn,
    /// Go to next value (cyclic)
    pub next: NextFn,
    /// Go to previous value (cyclic)
    pub prev: PrevFn,
    /// Move by the specified amount from the current value (cyclic)
    pub shift: ShiftFn,
}
