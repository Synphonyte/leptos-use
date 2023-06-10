use crate::use_event_listener;
use crate::utils::CloneableFnMutWithArg;
use leptos::ev::change;
use leptos::*;
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

/// Reactive [Media Query](https://developer.mozilla.org/en-US/docs/Web/CSS/Media_Queries/Testing_media_queries).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_media_query)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_media_query;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let is_large_screen = use_media_query(cx, "(min-width: 1024px)");
///
/// let is_dark_preferred = use_media_query(cx, "(prefers-color-scheme: dark)");
/// #
/// #    view! { cx, }
/// # }
/// ```
///
pub fn use_media_query(cx: Scope, query: impl Into<MaybeSignal<String>>) -> Signal<bool> {
    let query = query.into();

    let (matches, set_matches) = create_signal(cx, false);

    let media_query: Rc<RefCell<Option<web_sys::MediaQueryList>>> = Rc::new(RefCell::new(None));
    let remove_listener: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));

    let rem_listener = Rc::clone(&remove_listener);

    let listener: Rc<OnceCell<Box<dyn CloneableFnMutWithArg<web_sys::Event>>>> =
        Rc::new(OnceCell::new());

    let cleanup = move || {
        if let Some(remove_listener) = rem_listener.take().as_ref() {
            remove_listener();
        }
    };

    let clean = cleanup.clone();
    let listen = Rc::clone(&listener);

    let update = move || {
        clean();

        let mut media_query = media_query.borrow_mut();
        *media_query = window().match_media(&query.get()).unwrap_or(None);

        if let Some(media_query) = media_query.as_ref() {
            set_matches(media_query.matches());

            remove_listener.replace(Some(use_event_listener(
                cx,
                media_query.clone(),
                change,
                listen
                    .get()
                    .expect("cell should be initialized by now")
                    .clone(),
            )));
        } else {
            set_matches(false);
        }
    };

    let upd = update.clone();
    listener
        .set(Box::new(move |_| upd()) as Box<dyn CloneableFnMutWithArg<web_sys::Event>>)
        .expect("cell is empty");

    create_effect(cx, move |_| update());

    on_cleanup(cx, cleanup);

    matches.into()
}
