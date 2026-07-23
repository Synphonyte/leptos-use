use leptos::prelude::*;
use leptos_use::{UseCycleListReturn, use_cycle_list};

// The list is only checked for emptiness while the initial value is derived.
// When a reactive list becomes empty afterwards, every cycling helper divided
// by a zero length and panicked with
// "attempt to calculate the remainder with a divisor of zero".
#[test]
fn cycling_an_emptied_list_does_not_panic() {
    let owner = Owner::new();
    owner.set();

    let list = RwSignal::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

    let UseCycleListReturn {
        state,
        set_index,
        next,
        prev,
        shift,
        ..
    } = use_cycle_list(list);

    list.set(vec![]);

    next();
    prev();
    shift(3);
    shift(-3);
    set_index(2);

    assert_eq!(
        state.get_untracked(),
        "a".to_string(),
        "state must stay untouched while the list is empty"
    );
}

// Once the list is refilled the helpers have to resume cycling normally.
#[test]
fn cycling_resumes_after_the_list_is_refilled() {
    let owner = Owner::new();
    owner.set();

    let list = RwSignal::new(vec!["a".to_string(), "b".to_string()]);

    let UseCycleListReturn { state, next, .. } = use_cycle_list(list);

    list.set(vec![]);
    next();

    list.set(vec!["a".to_string(), "b".to_string()]);
    next();

    assert_eq!(state.get_untracked(), "b".to_string());
}
