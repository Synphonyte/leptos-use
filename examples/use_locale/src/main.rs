use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_locale;
use unic_langid::langid_slice;

#[component]
fn Demo() -> impl IntoView {
    let locale = use_locale(langid_slice!["en", "de", "fr"]);

    view! {
        <p>Locale: <code class="font-bold">{move || locale.get().to_string()}</code></p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
