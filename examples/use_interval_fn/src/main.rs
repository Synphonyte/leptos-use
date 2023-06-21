use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let greetings = [
        "Hello",
        "Hi",
        "Yo!",
        "Hey",
        "Hola",
        "こんにちは",
        "Bonjour",
        "Salut!",
        "你好",
        "Привет",
    ];

    let (word, set_word) = create_signal(cx, greetings[0]);
    let (interval, set_interval) = create_signal(cx, 500_u64);
    let (index, set_index) = create_signal(cx, 0);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        cx,
        move || {
            set_index.set((index.get() + 1) % greetings.len());
            set_word.set(greetings[index.get()]);
        },
        interval,
    );

    view! { cx,
        <p>{move || word.get()}</p>
        <p>
            "Interval:"
            <input
                prop:value=move || interval.get()
                on:input=move |e| set_interval.set(event_target_value(&e).parse().unwrap())
                type="number"
                placeholder="interval"
            />
        </p>
        <Show
            when=move || is_active.get()
            fallback=move |cx| {
                let resume = resume.clone();
                view! {cx,
                    <button on:click=move |_| resume()>"Resume"</button>
                }
            }
        >
            {
                let pause = pause.clone();
                view! {cx,
                    <button on:click=move |_| pause()>"Pause"</button>
                }
            }
        </Show>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
