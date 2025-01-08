use chrono::*;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_calendar_with_options, UseCalendarOptions, UseCalendarReturn};

#[component]
fn Demo() -> impl IntoView {
    let selected_date = RwSignal::new(Some(Local::now().date_naive()));
    let options = UseCalendarOptions::default()
        .first_day_of_the_week(6)
        .initial_date(selected_date);

    let UseCalendarReturn {
        weekdays,
        dates,
        previous_month,
        today,
        next_month,
    } = use_calendar_with_options(options);

    let current_month_year = Memo::new(move |_| {
        let current = dates
            .get()
            .into_iter()
            .find_map(|date| {
                if !date.is_other_month() && date.is_first_day_of_month() {
                    Some(*date)
                } else {
                    None
                }
            })
            .unwrap_or(Local::now().date_naive());
        format!(
            "{} {}",
            Month::try_from(current.month() as u8).unwrap().name(),
            current.year(),
        )
    });

    view! {
        <div class="w-[50%]">
            <div class="flex center-items justify-between">
                <button on:click=move |_| previous_month()>{"<<"}</button>
                <button on:click=move |_| today()>{"Today"}</button>
                <button on:click=move |_| next_month()>{">>"}</button>
            </div>
            <div class="flex center-items justify-center">{move || current_month_year.get()}</div>
            <div class="grid grid-cols-7">
                {move || {
                    weekdays
                        .get()
                        .iter()
                        .map(|weekday| {
                            view! {
                                <div class="p-1 text-center">
                                    {Weekday::try_from(*weekday as u8).unwrap().to_string()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
                {move || {
                    dates
                        .get()
                        .into_iter()
                        .map(|date| {
                            let is_selected = move || {
                                if let Some(selected_date) = selected_date.get() {
                                    *date == selected_date
                                } else {
                                    false
                                }
                            };
                            view! {
                                <div
                                    class="w-8 h-8 leading-8 cursor-pointer text-center p-4 justify-self-center border-2 border-solid rounded-full"
                                    class:text-red-500=date.is_today()
                                    class:text-gray-500=date.is_other_month()
                                    class:border-red-500=move || is_selected()
                                    class:border-transparent=move || !is_selected()
                                    on:click=move |_| selected_date.set(Some(*date))
                                >

                                    {date.day()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
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
