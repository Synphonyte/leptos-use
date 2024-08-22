use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_intl_number_format, NumberStyle, UseIntlNumberFormatOptions};

#[component]
fn Demo() -> impl IntoView {
    let (number, set_number) = signal(123456.78);

    let de_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("de-DE")
            .style(NumberStyle::Currency)
            .currency("EUR"),
    );
    let de_num = de_nf.format::<f64>(number);

    let ja_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("ja-JP")
            .style(NumberStyle::Currency)
            .currency("JPY"),
    );
    let ja_num = ja_nf.format::<f64>(number);

    let in_nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("en-IN")
            .maximum_significant_digits(3),
    );
    let in_num = in_nf.format::<f64>(number);

    view! {
        <input
            class="block"
            prop:value=number
            on:input=move |e| set_number(event_target_value(&e).parse().unwrap())
            type="range"
            min="-1000000"
            max="1000000"
            step="0.01"
        />
        <p>"Number: " {number}</p>
        <p>"German currency (EUR): " {de_num}</p>
        <p>"Japanese currency (JPY): " {ja_num}</p>
        <p>"Indian 3 max significant digits: " {in_num}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
