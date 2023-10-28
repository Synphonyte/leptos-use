use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::storage::{use_local_storage, JsonCodec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BananaState {
    pub name: String,
    pub wearing: String,
    pub descending: String,
    pub count: u32,
}

impl Default for BananaState {
    fn default() -> Self {
        Self {
            name: "Bananas".to_string(),
            wearing: "pyjamas".to_string(),
            descending: "stairs".to_string(),
            count: 2,
        }
    }
}

#[component]
fn Demo() -> impl IntoView {
    let (state, set_state, reset) = use_local_storage::<BananaState, JsonCodec>("banana-state");
    let (state2, _, _) = use_local_storage::<BananaState, JsonCodec>("banana-state");

    view! {
        <input
            class="block"
            prop:value=move || state.get().name
            on:input=move |e| set_state.update(|s| s.name = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state.get().wearing
            on:input=move |e| set_state.update(|s| s.wearing = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state.get().descending
            on:input=move |e| set_state.update(|s| s.descending = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state.get().count
            value=move || state.get().count
            on:input=move |e| {
                set_state
                    .update(|s| s.count = event_target_value(&e).parse::<f64>().unwrap() as u32)
            }

            type="number"
            min="0"
            step="1"
            max="1000"
        />
        <button on:click=move |_| reset()>"Delete from storage"</button>

        <p>
            "Second " <b>
                <code>"use_storage"</code>
            </b> ":"
        </p>

        <pre>{move || format!("{:#?}", state2.get())}</pre>

        <Note>
            "The values are persistent. When you reload the page the values will be the same."
        </Note>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
