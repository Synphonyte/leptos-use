use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{sync_signal, sync_signal_with_options, SyncDirection, SyncSignalOptions};

#[component]
fn Demo() -> impl IntoView {
    let (a1, set_a1) = signal(String::new());
    let (b1, set_b1) = signal(String::new());

    let _ = sync_signal((a1, set_a1), (b1, set_b1));

    let a2 = RwSignal::new(String::new());
    let b2 = RwSignal::new(String::new());

    let _ = sync_signal_with_options(
        a2,
        b2,
        SyncSignalOptions::default().direction(SyncDirection::LeftToRight),
    );

    let a3 = RwSignal::new(String::new());
    let b3 = RwSignal::new("not immediate".to_string());

    let _ = sync_signal_with_options(
        a3,
        b3,
        SyncSignalOptions::default()
            .direction(SyncDirection::RightToLeft)
            .immediate(false),
    );

    view! {
        <div class="flex items-center gap-2">
            <input class="block" bind:value=(a1, set_a1) placeholder="A" type="text" />

            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"
                />
            </svg>

            <input class="block" bind:value=(b1, set_b1) placeholder="B" type="text" />
        </div>

        <div class="flex items-center gap-2 my-5">
            <input class="block" bind:value=a2 placeholder="A" type="text" />

            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3"
                />
            </svg>

            <input class="block" bind:value=b2 placeholder="B" type="text" />
        </div>

        <div class="flex items-center gap-2">
            <input class="block" bind:value=a3 placeholder="A" type="text" />

            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18"
                />
            </svg>

            <input class="block" bind:value=b3 placeholder="B" type="text" />
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo /> }
    });

    unmount_handle.forget();
}
