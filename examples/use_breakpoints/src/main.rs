use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{breakpoints_tailwind, use_breakpoints, BreakpointsTailwind};

#[component]
fn Demo() -> impl IntoView {
    let breakpoints = breakpoints_tailwind();

    let screen_size = use_breakpoints(breakpoints.clone());

    use BreakpointsTailwind::*;

    let sm_width = *breakpoints.get(&Sm).expect("It's there!");

    let current = screen_size.current();
    let xs = screen_size.lt(Sm);
    let xse = screen_size.le(Sm);
    let sm = screen_size.between(Sm, Md);
    let md = screen_size.between(Md, Lg);
    let lg = screen_size.between(Lg, Xl);
    let xl = screen_size.between(Xl, Xxl);
    let xxl = screen_size.ge(Xxl);

    let label_classes = "justify-self-end".to_string();
    let svg_classes = "align-middle ml-3 mr-1 opacity-60".to_string();

    view! {
        <div class="grid grid-cols-2 gap-x-4 gap-y-3">
            <div class=label_classes.clone()>"Current breakpoints :"</div>
            <code>{move || format!("{:?}", current.get())}</code>

            <div class=label_classes.clone()>
                <code class="font-bold">"xs"</code>
                <small>" (< " {move || sm_width.to_string()} "px)"</small>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M6 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-14z"></path>
                    <path d="M11 4h2"></path>
                    <path d="M12 17v.01"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=xs/>

            <div class=label_classes.clone()>
                <code class="font-bold">"xs"</code>
                <small>" (<= " {move || sm_width.to_string()} "px)"</small>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M6 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-14z"></path>
                    <path d="M11 4h2"></path>
                    <path d="M12 17v.01"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=xse/>

            <div class=label_classes.clone()>
                <code class="font-bold">"sm"</code>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M3 6m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z"></path>
                    <path d="M20 11v2"></path>
                    <path d="M7 12h-.01"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=sm/>

            <div class=label_classes.clone()>
                <code class="font-bold">"md"</code>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M5 4a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v16a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16z"></path>
                    <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=md/>

            <div class=label_classes.clone()>
                <code class="font-bold">"lg"</code>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M3 19l18 0"></path>
                    <path d="M5 6m0 1a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1z"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=lg/>

            <div class=label_classes.clone()>
                <code class="font-bold">"xl"</code>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10z"></path>
                    <path d="M7 20h10"></path>
                    <path d="M9 16v4"></path>
                    <path d="M15 16v4"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=xl/>

            <div class=label_classes.clone()>
                <code class="font-bold">"xxl"</code>

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class=svg_classes.clone()
                    width="24"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M13 16h-9a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v5.5"></path>
                    <path d="M7 20h6.5"></path>
                    <path d="M9 16v4"></path>
                    <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5"></path>
                    <path d="M19 21v1m0 -8v1"></path>
                </svg>
                ":"
            </div>

            <BooleanDisplay value=xxl/>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
