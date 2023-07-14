use crate::core::ElementMaybeSignal;
use crate::use_event_listener;
use crate::utils::CloneableFnMutWithArg;
use default_struct_builder::DefaultBuilder;
use leptos::ev::{dragenter, dragleave, dragover, drop};
use leptos::*;

/// Create a zone where files can be dropped.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_drop_zone)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos::html::Div;
/// # use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let drop_zone_el = create_node_ref::<Div>(cx);
///
/// let on_drop = |event| {
///     // called when files are dropped on zone
/// };
///
/// let UseDropZoneReturn {
///     is_over_drop_zone,
///     ..
/// } = use_drop_zone_with_options(
///     cx,
///     drop_zone_el,
///     UseDropZoneOptions::default().on_drop(on_drop)
/// );
///
/// view! { cx,
///     <div node_ref=drop_zone_el>
///         "Drop files here"
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// Please refer to ["Functions with Target Elements"](https://leptos-use.rs/server_side_rendering.html#functions-with-target-elements)
pub fn use_drop_zone<El, T>(cx: Scope, target: El) -> UseDropZoneReturn
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    use_drop_zone_with_options(cx, target, UseDropZoneOptions::default())
}

/// Version of [`use_drop_zone`] that takes a `UseDropZoneOptions`. See [`use_drop_zone`] for how to use.
pub fn use_drop_zone_with_options<El, T>(
    cx: Scope,
    target: El,
    options: UseDropZoneOptions,
) -> UseDropZoneReturn
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    let UseDropZoneOptions {
        mut on_drop,
        mut on_enter,
        mut on_leave,
        mut on_over,
    } = options;

    let (is_over_drop_zone, set_over_drop_zone) = create_signal(cx, false);
    let (files, set_files) = create_signal(cx, Vec::<web_sys::File>::new());

    let counter = store_value(cx, 0_usize);

    let update_files = move |event: &web_sys::DragEvent| {
        if let Some(data_transfer) = event.data_transfer() {
            let files: Vec<web_sys::File> = data_transfer
                .files()
                .map(|f| js_sys::Array::from(&f).to_vec())
                .unwrap_or_default()
                .into_iter()
                .map(web_sys::File::from)
                .collect();

            set_files.update(move |f| *f = files);
        }
    };

    let _ = use_event_listener(cx, target.clone(), dragenter, move |event| {
        event.prevent_default();
        counter.update_value(|counter| *counter += 1);
        set_over_drop_zone.set(true);

        update_files(&event);

        on_enter(UseDropZoneEvent {
            files: files.get(),
            event,
        });
    });

    let _ = use_event_listener(cx, target.clone(), dragover, move |event| {
        event.prevent_default();
        update_files(&event);
        on_over(UseDropZoneEvent {
            files: files.get(),
            event,
        });
    });

    let _ = use_event_listener(cx, target.clone(), dragleave, move |event| {
        event.prevent_default();
        counter.update_value(|counter| *counter -= 1);
        if counter.get_value() == 0 {
            set_over_drop_zone.set(false);
        }

        update_files(&event);

        on_leave(UseDropZoneEvent {
            files: files.get(),
            event,
        });
    });

    let _ = use_event_listener(cx, target, drop, move |event| {
        event.prevent_default();
        counter.update_value(|counter| *counter = 0);
        set_over_drop_zone.set(false);

        update_files(&event);

        on_drop(UseDropZoneEvent {
            files: files.get(),
            event,
        });
    });

    UseDropZoneReturn {
        files: files.into(),
        is_over_drop_zone: is_over_drop_zone.into(),
    }
}

/// Options for [`use_drop_zone_with_options`].
#[derive(DefaultBuilder, Default, Clone, Debug)]
pub struct UseDropZoneOptions {
    /// Event handler for the [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event) event
    on_drop: Box<dyn CloneableFnMutWithArg<UseDropZoneEvent>>,
    /// Event handler for the [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event) event
    on_enter: Box<dyn CloneableFnMutWithArg<UseDropZoneEvent>>,
    /// Event handler for the [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event) event
    on_leave: Box<dyn CloneableFnMutWithArg<UseDropZoneEvent>>,
    /// Event handler for the [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event) event
    on_over: Box<dyn CloneableFnMutWithArg<UseDropZoneEvent>>,
}

/// Event passed as argument to the event handler functions of `UseDropZoneOptions`.
#[derive(Clone, Debug)]
pub struct UseDropZoneEvent {
    /// Files being handled
    pub files: Vec<web_sys::File>,
    /// The original drag event
    pub event: web_sys::DragEvent,
}

/// Return type of [`use_drop_zone`].
pub struct UseDropZoneReturn {
    /// Files being handled
    pub files: Signal<Vec<web_sys::File>>,
    /// Whether the files (dragged by the pointer) are over the drop zone
    pub is_over_drop_zone: Signal<bool>,
}
