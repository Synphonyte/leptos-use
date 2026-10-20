#[cfg(not(feature = "ssr"))]
use crate::UseDropZoneEvent;
use crate::{IntoElementMaybeSignal, UseDropZoneOptions, UseDropZoneReturn};
use cfg_if::cfg_if;
use leptos::prelude::*;
use send_wrapper::SendWrapper;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::use_event_listener;
    use leptos::ev::{dragenter, dragleave, dragover, drop};
}}

/// Create a zone where files can be dropped.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_drop_zone)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use_drop_zone::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let drop_zone_el = NodeRef::<Div>::new();
///
/// let on_drop = |event| {
///     // called when files are dropped on zone
/// };
///
/// let UseDropZoneReturn {
///     is_over_drop_zone,
///     ..
/// } = use_drop_zone_with_options(
///     drop_zone_el,
///     UseDropZoneOptions::default().on_drop(on_drop)
/// );
///
/// view! {
///     <div node_ref=drop_zone_el>
///         "Drop files here"
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the returned `file` signal always contains an empty `Vec` and
/// `is_over_drop_zone` contains always `false`
pub fn use_drop_zone<El, M>(target: El) -> UseDropZoneReturn
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    use_drop_zone_with_options(target, UseDropZoneOptions::default())
}

/// Version of [`use_drop_zone`] that takes a `UseDropZoneOptions`. See [`use_drop_zone`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn use_drop_zone_with_options<El, M>(
    target: El,
    options: UseDropZoneOptions,
) -> UseDropZoneReturn
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    let (is_over_drop_zone, set_over_drop_zone) = signal(false);
    let (files, set_files) = signal(Vec::<SendWrapper<web_sys::File>>::new());

    #[cfg(not(feature = "ssr"))]
    {
        use std::ops::Deref;

        let UseDropZoneOptions {
            on_drop,
            on_enter,
            on_leave,
            on_over,
        } = options;

        let counter = StoredValue::new(0_usize);

        let update_files = move |event: &web_sys::DragEvent| {
            if let Some(data_transfer) = event.data_transfer() {
                let files: Vec<_> = data_transfer
                    .files()
                    .map(|f| js_sys::Array::from(&f).to_vec())
                    .unwrap_or_default()
                    .into_iter()
                    .map(web_sys::File::from)
                    .map(SendWrapper::new)
                    .collect();

                set_files.update(move |f| *f = files);
            }
        };

        let target = target.into_element_maybe_signal();

        let use_drop_zone_event = move |event| UseDropZoneEvent {
            files: files
                .read_untracked()
                .iter()
                .map(|f| f.deref().clone())
                .collect(),
            event,
        };

        let _ = use_event_listener(target, dragenter, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter += 1);
            set_over_drop_zone.set(true);

            update_files(&event);

            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_enter(use_drop_zone_event(event));
        });

        let _ = use_event_listener(target, dragover, move |event| {
            event.prevent_default();
            update_files(&event);

            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_over(use_drop_zone_event(event));
        });

        let _ = use_event_listener(target, dragleave, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter = counter.saturating_sub(1));
            if counter.get_value() == 0 {
                set_over_drop_zone.set(false);
            }

            update_files(&event);

            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_leave(use_drop_zone_event(event));
        });

        let _ = use_event_listener(target, drop, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter = 0);
            set_over_drop_zone.set(false);

            update_files(&event);

            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_drop(use_drop_zone_event(event));
        });
    }

    UseDropZoneReturn {
        files: files.into(),
        is_over_drop_zone: is_over_drop_zone.into(),
    }
}
