use leptos::*;

#[component]
pub fn Note(cx: Scope, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = format!("note {class}");

    view! { cx,
        <div class=class>{ children(cx) }</div>
    }
}
