use crate::core::ElementMaybeSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

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
/// # use leptos::*;
/// # use leptos::html::Div;
/// # use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let drop_zone_el = create_node_ref::<Div>();
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
/// On the server the returned `file` signal always contains an empty `Vec` and
/// `is_over_drop_zone` contains always `false`
pub fn use_drop_zone<El, T>(target: El) -> UseDropZoneReturn
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    use_drop_zone_with_options(target, UseDropZoneOptions::default())
}

/// Version of [`use_drop_zone`] that takes a `UseDropZoneOptions`. See [`use_drop_zone`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn use_drop_zone_with_options<El, T>(
    target: El,
    options: UseDropZoneOptions,
) -> UseDropZoneReturn
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    let (is_over_drop_zone, set_over_drop_zone) = create_signal(false);
    let files= create_rw_signal(Vec::<web_sys::File>::new());

    #[cfg(not(feature = "ssr"))]
    {
        let UseDropZoneOptions {
            on_drop,
            on_enter,
            on_leave,
            on_over,
            extend_files
        } = options;

        let counter = store_value(0_usize);

        let update_files = move |event: &web_sys::DragEvent| {
            if let Some(data_transfer) = event.data_transfer() {
                let new_files: Vec<web_sys::File> = data_transfer
                    .files()
                    .map(|f| js_sys::Array::from(&f).to_vec())
                    .unwrap_or_default()
                    .into_iter()
                    .map(web_sys::File::from)
                    .collect();

                if extend_files {
                    files.update(|files| files.extend(new_files));
                } else {
                    files.update(move |f| *f = new_files);
                }
            }
        };

        let _ = use_event_listener(target.clone(), dragenter, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter += 1);
            set_over_drop_zone.set(true);

            update_files(&event);

            #[cfg(debug_assertions)]
            let prev = SpecialNonReactiveZone::enter();

            on_enter(UseDropZoneEvent {
                files: files.get_untracked(),
                event,
            });

            #[cfg(debug_assertions)]
            SpecialNonReactiveZone::exit(prev);
        });

        let _ = use_event_listener(target.clone(), dragover, move |event| {
            event.prevent_default();
            update_files(&event);

            #[cfg(debug_assertions)]
            let prev = SpecialNonReactiveZone::enter();

            on_over(UseDropZoneEvent {
                files: files.get_untracked(),
                event,
            });

            #[cfg(debug_assertions)]
            SpecialNonReactiveZone::exit(prev);
        });

        let _ = use_event_listener(target.clone(), dragleave, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter -= 1);
            if counter.get_value() == 0 {
                set_over_drop_zone.set(false);
            }

            update_files(&event);

            #[cfg(debug_assertions)]
            let prev = SpecialNonReactiveZone::enter();

            on_leave(UseDropZoneEvent {
                files: files.get_untracked(),
                event,
            });

            #[cfg(debug_assertions)]
            SpecialNonReactiveZone::exit(prev);
        });

        let _ = use_event_listener(target, drop, move |event| {
            event.prevent_default();
            counter.update_value(|counter| *counter = 0);
            set_over_drop_zone.set(false);

            update_files(&event);

            #[cfg(debug_assertions)]
            let prev = SpecialNonReactiveZone::enter();

            on_drop(UseDropZoneEvent {
                files: files.get_untracked(),
                event,
            });

            #[cfg(debug_assertions)]
            SpecialNonReactiveZone::exit(prev);
        });
    }

    UseDropZoneReturn {
        files: files.into(),
        is_over_drop_zone: is_over_drop_zone.into(),
    }
}

/// Options for [`use_drop_zone_with_options`].
#[derive(DefaultBuilder, Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseDropZoneOptions {
    /// Event handler for the [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event) event
    on_drop: Rc<dyn Fn(UseDropZoneEvent)>,
    /// Event handler for the [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event) event
    on_enter: Rc<dyn Fn(UseDropZoneEvent)>,
    /// Event handler for the [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event) event
    on_leave: Rc<dyn Fn(UseDropZoneEvent)>,
    /// Event handler for the [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event) event
    on_over: Rc<dyn Fn(UseDropZoneEvent)>,
    /// Whether to allow the files to be extended or replaced
    extend_files: bool,
}

impl Default for UseDropZoneOptions {
    fn default() -> Self {
        Self {
            on_drop: Rc::new(|_| {}),
            on_enter: Rc::new(|_| {}),
            on_leave: Rc::new(|_| {}),
            on_over: Rc::new(|_| {}),
            extend_files: false,
        }
    }
}

impl Debug for UseDropZoneOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UseDropZoneOptions")
    }
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
    pub files: RwSignal<Vec<web_sys::File>>,
    /// Whether the files (dragged by the pointer) are over the drop zone
    pub is_over_drop_zone: Signal<bool>,
}
