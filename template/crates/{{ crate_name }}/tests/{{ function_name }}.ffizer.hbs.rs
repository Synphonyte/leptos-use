#![cfg({{#if (eq unstable_apis "true")}}all(target_arch = "wasm32", web_sys_unstable_apis){{else}}target_arch = "wasm32"{{/if}})]

use any_spawner::Executor;
use leptos::prelude::*;
use {{ crate_lib }}::{{ function_name }};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn works_in_the_browser() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let _ = {{ function_name }}();
    Executor::tick().await;

    // TODO: drive the hook (DOM changes, events, timers) and assert on what it returns.
}
