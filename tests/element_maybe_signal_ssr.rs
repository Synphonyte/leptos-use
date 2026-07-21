#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use leptos_use::core::{
    ElementMaybeSignal, ElementsMaybeSignal, IntoElementMaybeSignal, IntoElementsMaybeSignal,
};
use send_wrapper::SendWrapper;

// There are no DOM elements on the server, and the conversions say so: every
// other branch short-circuits to `None` under `ssr` precisely so that a
// `SendWrapper` created on one worker thread is never touched from another.
// `SendWrapper` asserts the accessing thread on every access and panics
// otherwise, and work-stealing runtimes such as tokio move a render between
// threads at await points.
//
// The two branches below skipped that guard and unwrapped the value anyway.

#[test]
fn option_send_wrapper_signal_yields_none_on_the_server() {
    let owner = Owner::new();
    owner.set();

    let (element, _) = signal(Some(SendWrapper::new("element".to_string())));

    let signal: ElementMaybeSignal<String> = element.into_element_maybe_signal();

    assert!(
        signal.get().is_none(),
        "the server must not unwrap the SendWrapper"
    );
}

#[test]
fn vec_send_wrapper_signal_yields_nothing_on_the_server() {
    let owner = Owner::new();
    owner.set();

    let (elements, _) = signal(vec![
        SendWrapper::new("first".to_string()),
        SendWrapper::new("second".to_string()),
    ]);

    let signal: ElementsMaybeSignal<String> = elements.into_elements_maybe_signal();

    assert!(
        signal.get().iter().all(Option::is_none),
        "the server must not unwrap the SendWrappers"
    );
}
