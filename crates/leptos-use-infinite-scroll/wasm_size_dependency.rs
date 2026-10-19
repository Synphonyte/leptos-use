//! Uses `futures-util` and `gloo-timers` like `use_infinite_scroll` does: waiting for a future and
//! a timeout concurrently. Used to measure the wasm size of the crate only.

use futures_util::join;
use gloo_timers::future::sleep;
use leptos::prelude::*;
use std::hint::black_box;
use std::time::Duration;

#[component]
fn DependencyDemo() -> impl IntoView {
    let (done, set_done) = signal(false);

    leptos::task::spawn_local(async move {
        join!(
            async move { set_done.set(black_box(true)) },
            sleep(Duration::from_millis(black_box(100)))
        );
        sleep(Duration::ZERO).await;
    });

    view! { <p>{done}</p> }
}
