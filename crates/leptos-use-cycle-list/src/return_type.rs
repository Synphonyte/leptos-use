use leptos::prelude::*;

/// Return type of [`use_cycle_list`](crate::use_cycle_list).
pub struct UseCycleListReturn<T, SetFn, NextFn, PrevFn, ShiftFn>
where
    T: Clone + PartialEq + Send + Sync + 'static,
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
