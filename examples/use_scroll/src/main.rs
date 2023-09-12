use leptos::html::Div;
use leptos::logging::log;
use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{use_scroll_with_options, ScrollBehavior, UseScrollOptions, UseScrollReturn};

use web_sys::Event;

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (smooth, set_smooth) = create_signal(false);
    let behavior = Signal::derive(move || {
        if smooth.get() {
            ScrollBehavior::Smooth
        } else {
            ScrollBehavior::Auto
        }
    });

    let on_stop = |_: Event| {
        log!("scrolling stopped");
    };

    let on_scroll = |_: Event| {
        log!("scrolling");
    };

    let UseScrollReturn {
        x,
        y,
        set_x,
        set_y,
        is_scrolling,
        arrived_state,
        directions,
        ..
    } = use_scroll_with_options(
        el,
        UseScrollOptions::default()
            .behavior(behavior)
            .on_stop(on_stop.clone())
            .on_scroll(on_scroll.clone()),
    );

    view! {
        <div class="flex">
            <div
                node_ref=el
                class="w-[300px] h-[300px] m-auto my-auto overflow-scroll bg-gray-500/5 rounded"
            >
                <div class="w-[500px] h-[400px] relative">
                    <div class="absolute left-0 top-0 bg-gray-500/5 px-2 py-1">"top_left"</div>
                    <div class="absolute left-0 bottom-0 bg-gray-500/5 px-2 py-1">
                        "bottom_left"
                    </div>
                    <div class="absolute right-0 top-0 bg-gray-500/5 px-2 py-1">"top_right"</div>
                    <div class="absolute right-0 bottom-0 bg-gray-500/5 px-2 py-1">
                        "bottom_right"
                    </div>

                    <div class="absolute left-1/3 top-1/3 bg-gray-500/5 px-2 py-1">"Scroll Me"</div>
                </div>
            </div>

            <div class="my-10 w-280px pl-4">
                <div class="px-6 py-4 rounded grid grid-cols-[120px_auto] gap-4 bg-gray-500/5">
                    <span class="text-right opacity-75 py-4">"X Position"</span>
                    <div class="text-primary">
                        <div>
                            <input
                                prop:value=move || format!("{:.1}", x.get())
                                on:input=move |e| {
                                    if let Ok(num) = event_target_value(&e).parse::<f64>() {
                                        set_x(num);
                                    }
                                }

                                type="number"
                                min="0"
                                max="200"
                                step="10"
                            />
                        </div>
                    </div>
                    <span class="text-right opacity-75 py-4">"Y Position"</span>
                    <div class="text-primary">
                        <div>
                            <input
                                prop:value=move || format!("{:.1}", y.get())
                                on:input=move |e| {
                                    if let Ok(num) = event_target_value(&e).parse::<f64>() {
                                        set_y(num);
                                    }
                                }

                                type="number"
                                min="0"
                                max="200"
                                step="10"
                            />
                        </div>
                    </div>
                    <label for_="smooth-scrolling-option" class="text-right opacity-75">
                        "Smooth scrolling"
                    </label>
                    <span>
                        <input
                            id="smooth-scrolling-option"
                            prop:checked=move || smooth.get()
                            on:input=move |e| set_smooth.set(event_target_checked(&e))
                            type="checkbox"
                        />
                    </span>
                    <span class="text-right opacity-75">"Is Scrolling"</span>
                    <BooleanDisplay value=is_scrolling/>
                    <div class="text-right opacity-75">"Top Arrived"</div>
                    <BooleanDisplay value=Signal::derive(move || arrived_state.get().top)/>
                    <div class="text-right opacity-75">"Right Arrived"</div>
                    <BooleanDisplay value=Signal::derive(move || arrived_state.get().right)/>
                    <div class="text-right opacity-75">"Bottom Arrived"</div>
                    <BooleanDisplay value=Signal::derive(move || arrived_state.get().bottom)/>
                    <div class="text-right opacity-75">"Left Arrived"</div>
                    <BooleanDisplay value=Signal::derive(move || arrived_state.get().left)/>
                    <div class="text-right opacity-75">"Scrolling Up"</div>
                    <BooleanDisplay value=Signal::derive(move || directions.get().top)/>
                    <div class="text-right opacity-75">"Scrolling Right"</div>
                    <BooleanDisplay value=Signal::derive(move || directions.get().right)/>
                    <div class="text-right opacity-75">"Scrolling Down"</div>
                    <BooleanDisplay value=Signal::derive(move || directions.get().bottom)/>
                    <div class="text-right opacity-75">"Scrolling Left"</div>
                    <BooleanDisplay value=Signal::derive(move || directions.get().left)/>
                </div>
            </div>
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
