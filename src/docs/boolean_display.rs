use leptos::*;

#[component]
pub fn BooleanDisplay(
    cx: Scope,
    #[prop(into)] value: MaybeSignal<bool>,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let true_class = "text-green-600 dark:text-green-500";
    let false_class = "text-[--brand-color]";
    let true_str = "true";
    let false_str = "false";

    let class = move || {
        format!(
            "{} {} opacity-75",
            if value.get() { true_class } else { false_class },
            class
        )
    };

    view! { cx,
        <span class=class>
            { move || if value() { true_str} else { false_str } }
        </span>
    }
}
