use leptos::prelude::*;

#[component]
pub fn Note(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = format!("note {class}");

    view! { <div class=class>{children()}</div> }
}
