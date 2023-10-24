use leptos::html::Div;
use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Div>();

    let (data, set_data) = create_signal(vec![1, 2, 3, 4, 5, 6]);

    let _ = use_infinite_scroll_with_options(
        el,
        move |_| async move {
            let len = data.with_untracked(|d| d.len());
            set_data.update(|data| *data = (1..len + 6).collect());
        },
        UseInfiniteScrollOptions::default().distance(10.0),
    );

    view! {
        <div
            node_ref=el
            class="flex flex-col gap-2 p-4 w-[300px] h-[300px] m-auto overflow-y-scroll bg-gray-500/5 rounded"
        >
            <For each=move || data.get() key=|i| *i let:item>
                <div class="h-15 bg-gray-500/5 rounded p-3">{item}</div>
            </For>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
