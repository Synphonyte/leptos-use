use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_geolocation, UseGeolocationReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseGeolocationReturn {
        coords,
        located_at,
        error,
        resume,
        pause,
    } = use_geolocation();

    view! {
        <pre lang="json">
            coords:
            {move || {
                if let Some(coords) = coords() {
                    format!(
                        r#"{{
        accuracy: {},
        latitude: {},
        longitude: {},
        altitude: {:?},
        altitude_accuracy: {:?},
        heading: {:?},
        speed: {:?},
    }}"#,
                        coords.accuracy(),
                        coords.latitude(),
                        coords.longitude(),
                        coords.altitude(),
                        coords.altitude_accuracy(),
                        coords.heading(),
                        coords.speed(),
                    )
                } else {
                    "None".to_string()
                }
            }}
            ,
            located_at: {located_at} ,
            error:
            {move || if let Some(error) = error() { error.message() } else { "None".to_string() }} ,
        </pre>
        <button on:click=move |_| pause()>"Pause watch"</button>
        <button on:click=move |_| resume()>"Resume watch"</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
