use leptos::prelude::*;

#[component]
pub fn LogDisplay(#[prop(into)] log: Signal<Vec<String>>) -> impl IntoView {
    view! {
        <div>
            <ul>
                {move || log().iter().map(|l| view! { <li>{l}</li> }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
