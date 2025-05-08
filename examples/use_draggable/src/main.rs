use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use::core::Position;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};

#[component]
fn Demo() -> impl IntoView {
    let fixed_el = NodeRef::<Div>::new();
    let absolute_el = NodeRef::<Div>::new();

    let inner_width = use_window()
        .as_ref()
        .map(|w| w.inner_width().unwrap().as_f64().unwrap())
        .unwrap_or(0.0);

    let UseDraggableReturn {
        x: fixed_x,
        y: fixed_y,
        style: fixed_style,
        ..
    } = use_draggable_with_options(
        fixed_el,
        UseDraggableOptions::default()
            .initial_value(Position {
                x: inner_width / 2.2,
                y: 100.0,
            })
            .prevent_default(true),
    );

    let UseDraggableReturn {
        x: absolute_x,
        y: absolute_y,
        ..
    } = use_draggable_with_options(
        absolute_el,
        UseDraggableOptions::default()
            .initial_value(Position { x: 0., y: 0. })
            .target_offset(move |event_target: web_sys::EventTarget| {
                let target: web_sys::HtmlElement = event_target.unchecked_into();
                let (x, y): (f64, f64) = (target.offset_left().into(), target.offset_top().into());

                (x, y)
            })
            .prevent_default(true),
    );

    view! {
        <p class="italic op50 text-center">Check the floating boxes</p>
        <div
            node_ref=fixed_el
            class="fixed px-4 py-2 border border-gray-400/30 rounded shadow hover:shadow-lg bg-[--bg] select-none cursor-move z-30"
            style=move || format!("touch-action: none; {}", fixed_style())
        >
            "Fixed 👋 Drag me!"
            <div class="text-sm opacity-50">I am {move || fixed_x().round()} , {move || fixed_y().round()}</div>
        </div>

        <div class="relative w-1/3 h-40 bg-green-800">
            <div
                node_ref=absolute_el
                class="absolute w-1/2 h-1/2 px-4 py-2 border border-gray-400/30 rounded shadow hover:shadow-lg bg-[--bg] select-none cursor-move z-24"
                style=move || format!("touch-action: none; left: {}px; top: {}px;", absolute_x(), absolute_y())
            >
                "Absolute 👋 Drag me!"
                <div class="text-sm opacity-50">I am {move || absolute_x().round()} , {move || absolute_y().round()}</div>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
