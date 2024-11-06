use leptos::prelude::*;

#[component]
pub fn BooleanDisplay(
    #[prop(into)] value: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(default = "true")] true_str: &'static str,
    #[prop(default = "false")] false_str: &'static str,
) -> impl IntoView {
    let true_class = "text-green-600 dark:text-green-500";
    let false_class = "text-[--brand-color]";

    let class = move || {
        format!(
            "{} {} opacity-75",
            if value.get() { true_class } else { false_class },
            class
        )
    };

    view! { <span class=class>{move || if value.get() { true_str } else { false_str }}</span> }
}
