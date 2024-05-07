use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_sorted;

fn string_list(list: &[i32]) -> String {
    list.into_iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[component]
fn Demo() -> impl IntoView {
    let (list, set_list) = create_signal::<Vec<i32>>(vec![4, 2, 67, 34, 76, 22, 2, 4, 65, 23]);

    let sorted: Signal<Vec<i32>> = use_sorted(list);

    let on_input = move |evt| {
        set_list.update(|list| {
            *list = event_target_value(&evt)
                .split(",")
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        });
    };

    let input_text = move || string_list(&list());
    let sorted_text = move || string_list(&sorted());

    view! {
        <div>Input:</div>
        <input prop:value=input_text on:input=on_input type="text"/>
        <p>Output: {sorted_text}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
