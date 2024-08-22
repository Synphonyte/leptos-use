use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_favicon_with_options, UseFaviconOptions};

#[component]
fn Demo() -> impl IntoView {
    let (_, set_icon) =
        use_favicon_with_options(UseFaviconOptions::default().base_url("use_favicon/demo/img/"));

    let classes = "border border-solid border-b-4 rounded p-2 block border-gray-500/50 bg-[--bg] active:translate-y-1 active:border-b".to_string();
    let img_classes = "block".to_string();

    view! {
        <p>"Click on an icon to change the favicon"</p>
        <p class="flex gap-2">
            <a
                class=classes.clone()
                href="#"
                on:click=move |e| {
                    e.prevent_default();
                    set_icon.set(Some("favicon-leptos.ico".into()));
                }
            >

                <img
                    class=img_classes.clone()
                    width="32"
                    src="use_favicon/demo/img/favicon-leptos.ico"
                    alt="favicon-red"
                />
            </a>

            <a
                class=classes.clone()
                href="#"
                on:click=move |e| {
                    e.prevent_default();
                    set_icon.set(Some("favicon-red.svg".into()));
                }
            >

                <img
                    class=img_classes.clone()
                    width="32"
                    src="use_favicon/demo/img/favicon-red.svg"
                    alt="favicon-red"
                />
            </a>

            <a
                class=classes.clone()
                href="#"
                on:click=move |e| {
                    e.prevent_default();
                    set_icon.set(Some("favicon-green.svg".into()));
                }
            >

                <img
                    class=img_classes.clone()
                    width="32"
                    src="use_favicon/demo/img/favicon-green.svg"
                    alt="favicon-green"
                />
            </a>

            <a
                class=classes.clone()
                href="#"
                on:click=move |e| {
                    e.prevent_default();
                    set_icon.set(Some("favicon-blue.svg".into()));
                }
            >

                <img
                    class=img_classes.clone()
                    width="32"
                    src="use_favicon/demo/img/favicon-blue.svg"
                    alt="favicon-blue"
                />
            </a>

            <a
                class=classes.clone()
                href="#"
                on:click=move |e| {
                    e.prevent_default();
                    set_icon.set(Some("favicon-orange.svg".into()));
                }
            >

                <img
                    class=img_classes
                    width="32"
                    src="use_favicon/demo/img/favicon-orange.svg"
                    alt="favicon-orange"
                />
            </a>
        </p>
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
