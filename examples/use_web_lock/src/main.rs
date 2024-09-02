use gloo_timers::future::sleep;
use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_web_lock;
use std::time::Duration;

async fn my_process(_lock: web_sys::Lock) -> i32 {
    sleep(Duration::from_millis(2000)).await;

    42
}

#[component]
fn Demo() -> impl IntoView {
    let (res, set_res) = create_signal("Not started yet".to_string());

    let on_click = move |_| {
        set_res.set("Running...".to_string());

        spawn_local(async move {
            let res = use_web_lock("my_lock", my_process).await;

            match res {
                Ok(res) => {
                    set_res.set(format!("Result: {}", res));
                }
                Err(e) => {
                    set_res.set(format!("Error: {:?}", e));
                }
            }
        });
    };

    view! {
        <button on:click=on_click>Run locked task</button>
        <p>{res}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
