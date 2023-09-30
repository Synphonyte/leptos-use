use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay, Note};
use leptos_use::{use_idle, use_timestamp_with_options, UseIdleReturn, UseTimestampOptions};

#[component]
fn Demo() -> impl IntoView {
    let UseIdleReturn {
        idle, last_active, ..
    } = use_idle(5000);

    let now = use_timestamp_with_options(UseTimestampOptions::default().interval(1000));

    let idled_for = move || ((now() - last_active()) / 1000.0).ceil() as u64;

    view! {
        <Note class="mb-2">
            For demonstration purpose, the idle timeout is set to
            <b>
                5s
            </b>
            in this demo (default 1min).
        </Note>
        <div class="mb-2">
            Idle:
            <BooleanDisplay value=idle/>
        </div>
        <div>
            Inactive:
            <b>{idled_for} s</b>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
