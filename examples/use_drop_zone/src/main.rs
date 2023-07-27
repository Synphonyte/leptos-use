use leptos::html::Div;
use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};

#[component]
fn Demo() -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);

    let drop_zone_el = create_node_ref::<Div>();

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |_| set_dropped(true))
            .on_enter(move |_| set_dropped(false)),
    );

    view! {         <div class="flex">
            <div class="w-full h-auto relative">
                <p>Drop files into dropZone</p>
                <img width="64" src="use_drop_zone/demo/img/leptos-use-logo.svg" alt="Drop me"/>
                <div
                    node_ref=drop_zone_el
                    class="flex flex-col w-full min-h-[200px] h-auto bg-gray-400/10 justify-center items-center pt-6"
                >
                    <div>
                        is_over_drop_zone: <BooleanDisplay value=is_over_drop_zone />
                    </div>
                    <div>
                        dropped: <BooleanDisplay value=dropped />
                    </div>
                    <div class="flex flex-wrap justify-center items-center">
                        <For each=files key=|f| f.name() view=move |file| {
                            view! {                                 <div class="w-200px bg-black-200/10 ma-2 pa-6">
                                    <p>Name: {file.name()}</p>
                                    <p>Size: {file.size()}</p>
                                    <p>Type: {file.type_()}</p>
                                    <p>Last modified: {file.last_modified()}</p>
                                </div>
                            }
                        } />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo /> }
    })
}
