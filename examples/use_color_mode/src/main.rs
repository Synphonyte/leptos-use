use leptos::html::Col;
use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{
    use_color_mode_with_options, use_cycle_list, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListReturn,
};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(cx, UseColorModeOptions::default());

    let UseCycleListReturn { state, next, .. } = use_cycle_list(
        cx,
        vec![
            ColorMode::Light,
            ColorMode::Custom("rust".into()),
            ColorMode::Custom("coal".into()),
            ColorMode::Custom("navy".into()),
            ColorMode::Custom("ayu".into()),
        ],
    );

    view! { cx,
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
