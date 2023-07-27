use leptos::html::Div;
use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{
    use_intersection_observer_with_options, UseIntersectionObserverOptions,
    UseIntersectionObserverReturn,
};

#[component]
fn Demo() -> impl IntoView {
    let root = create_node_ref::<Div>();
    let target = create_node_ref::<Div>();
    let (is_visible, set_visible) = create_signal(false);

    let UseIntersectionObserverReturn {
        is_active,
        pause,
        resume,
        ..
    } = use_intersection_observer_with_options(
        target,
        move |entries, _| {
            set_visible.set(entries[0].is_intersecting());
        },
        UseIntersectionObserverOptions::default().root(Some(root)),
    );

    view! {
        <div class="text-center">
            <label class="checkbox">
                <input
                    type="checkbox"
                    prop:checked=move || is_active.get()
                    name="enabled"
                    on:input=move |e| {
                        if event_target_checked(&e) {
                            resume();
                        } else {
                            pause();
                        }
                    }
                />

                <span>"Enabled"</span>
            </label>
        </div>

        <div node_ref=root class="root">
            <p class="notice">"Scroll me down!"</p>
            <div node_ref=target class="target">
                <p>"Hello world!"</p>
            </div>
        </div>

        <div class="text-center">
            "Element "
            <BooleanDisplay
                value=is_visible
                true_str="inside"
                false_str="outside"
                class="font-bold"
            /> " the viewport"
        </div>

        <style>
            "
            .root {
            border: 2px dashed #ccc;
            height: 200px;
            margin: 2rem 1rem;
            overflow-y: scroll;
            }
            
            .notice {
            text-align: center;
            padding: 3em 0;
            margin-bottom: 300px;
            font-style: italic;
            font-size: 1.8rem;
            opacity: 0.8;
            }
            
            .target {
            border: 2px dashed var(--brand-color);
            padding: 10px;
            max-height: 150px;
            margin: 0 2rem 400px;
            }
            "
        </style>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
