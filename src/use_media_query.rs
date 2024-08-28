#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::use_event_listener;
use cfg_if::cfg_if;
use leptos::ev::change;
use leptos::reactive_graph::wrappers::read::Signal;
use leptos::prelude::*;
use std::cell::RefCell;
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
/// # use leptos::prelude::*;
/// # use leptos_use::use_media_query;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// let is_large_screen = use_media_query("(min-width: 1024px)");
///
/// let is_dark_preferred = use_media_query("(prefers-color-scheme: dark)");
/// #
/// #    view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this functions returns a Signal that is always `false`.
///
/// ## See also
///
/// * [`fn@crate::use_preferred_dark`]
/// * [`fn@crate::use_preferred_contrast`]
/// * [`fn@crate::use_prefers_reduced_motion`]
pub fn use_media_query(query: impl Into<MaybeSignal<String>>) -> Signal<bool> {
    let query = query.into();

    let (matches, set_matches) = signal(false);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let media_query: Rc<RefCell<Option<web_sys::MediaQueryList>>> = Rc::new(RefCell::new(None));
        let remove_listener: RemoveListener = Rc::new(RefCell::new(None));

        let listener = Rc::new(RefCell::new(Rc::new(|_| {}) as Rc<dyn Fn(web_sys::Event)>));

        let cleanup = {
            let remove_listener = Rc::clone(&remove_listener);

            move || {
                if let Some(remove_listener) = remove_listener.take().as_ref() {
                    remove_listener();
                }
            }
        };

        let update = {
            let cleanup = cleanup.clone();
            let listener = Rc::clone(&listener);

            Rc::new(move || {
                cleanup();

                let mut media_query = media_query.borrow_mut();
                *media_query = window().match_media(&query.get()).unwrap_or(None);

                if let Some(media_query) = media_query.as_ref() {
                    set_matches.set(media_query.matches());

                    let listener = Rc::clone(&*listener.borrow());

                    remove_listener.replace(Some(Box::new(use_event_listener(
                        media_query.clone(),
                        change,
                        move |e| listener(e),
                    ))));
                } else {
                    set_matches.set(false);
                }
            })
        };

        {
            let update = Rc::clone(&update);
            listener.replace(Rc::new(move |_| update()) as Rc<dyn Fn(web_sys::Event)>);
        }

        Effect::new(move |_| update());

        on_cleanup({
            let cleanup = send_wrapper::SendWrapper::new(cleanup);
            #[allow(clippy::redundant_closure)]
            move || cleanup()
        });
    }}

    matches.into()
}

type RemoveListener = Rc<RefCell<Option<Box<dyn Fn()>>>>;
