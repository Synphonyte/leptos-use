use leptos::prelude::{guards::ReadGuard, *};
use leptos::reactive::wrappers::read::Signal;
use send_wrapper::SendWrapper;
use std::{ops::Deref, rc::Rc, time::Duration};

use crate::{
    UseMutationObserverOptions, UseMutationObserverReturn, use_mutation_observer_with_options,
};

/// Used as an argument type to make it easily possible to pass either
///
/// * a `&str` for example "div > p.some-class",
/// * a `web_sys` element that implements `E` (for example `EventTarget`, `Element` or `HtmlElement`),
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>`, `RwSignal<T>`, `ReadSignal<T>` or `Memo<T>` where `T` is the web_sys element or a String,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `Signal<SendWrapper<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
///
/// into a function. Used for example in [`fn@crate::use_event_listener`].
///
/// ```
/// # use leptos::{html::Div, prelude::*};
/// # use leptos_use::{use_element_size};
/// # use send_wrapper::SendWrapper;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let test = "div > p.some-class";
/// use_element_size(&test); // &str
/// use_element_size(document().body()); // Option<web_sys::Element>
/// use_element_size(document().body().unwrap()); // web_sys::Element
///
/// let (string_signal, _set_string_signal) = signal("div > p.some-class".to_string());
/// use_element_size(string_signal); // Signal<String>
///
/// let (el_signal, _set_el_signal) = signal(
///     Some(SendWrapper::new(
///         document().query_selector("div > p.some-class").unwrap().unwrap()
///     ))
/// );
/// use_element_size(el_signal); // Signal<Option<SendWrapper<web_sys::Element>>>
///
/// let (el_signal_send_wrapper, _set_el_signal_send_wrapper) = signal(
///     SendWrapper::new(
///         document().query_selector("div > p.some-class").unwrap().unwrap()
///     )
/// );
/// use_element_size(el_signal_send_wrapper); // Signal<SendWrapper<web_sys::Element>>
///
/// let el = NodeRef::<Div>::new();
/// use_element_size(el); // NodeRef
///
///
/// # view! {
/// # }
/// # }
/// ```
#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct ElementMaybeSignal<T: 'static> {
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    inner: ElementMaybeSignalType<T>,
}

impl<T> Clone for ElementMaybeSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementMaybeSignal<T> {}

pub struct ElementMaybeSignalType<T: 'static>(Signal<Option<SendWrapper<T>>>);

impl<T> Clone for ElementMaybeSignalType<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementMaybeSignalType<T> {}

impl<T: 'static> Default for ElementMaybeSignalType<T> {
    fn default() -> Self {
        Self(Signal::stored(None))
    }
}

impl<T> Default for ElementMaybeSignal<T> {
    fn default() -> Self {
        Self {
            inner: ElementMaybeSignalType::default(),
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }
}

impl<T> DefinedAt for ElementMaybeSignal<T> {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T> ReadUntracked for ElementMaybeSignal<T> {
    type Value =
        ReadGuard<Option<SendWrapper<T>>, SignalReadGuard<Option<SendWrapper<T>>, SyncStorage>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.inner.0.try_read_untracked()
    }
}

impl<T> Track for ElementMaybeSignal<T> {
    fn track(&self) {
        self.inner.0.track();
    }
}

pub trait IntoElementMaybeSignal<T, Marker> {
    fn into_element_maybe_signal(self) -> ElementMaybeSignal<T>;
}

impl<El, T, Marker> IntoElementMaybeSignal<T, Marker> for El
where
    El: IntoElementMaybeSignalType<T, Marker>,
{
    fn into_element_maybe_signal(self) -> ElementMaybeSignal<T> {
        ElementMaybeSignal {
            inner: self.into_element_maybe_signal_type(),
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }
}

pub trait IntoElementMaybeSignalType<T, Marker> {
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T>;
}

// From static element //////////////////////////////////////////////////////////////

/// Handles `window()` or `document()`
impl<T, Js> IntoElementMaybeSignalType<T, web_sys::Element> for Js
where
    T: From<Js> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType(Signal::stored(Some(SendWrapper::new(
            T::from(self).clone(),
        ))))
    }
}

/// Handles `window().body()`
impl<T, Js> IntoElementMaybeSignalType<T, Option<web_sys::Element>> for Option<Js>
where
    T: From<Js> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType(Signal::stored(
            self.map(|el| SendWrapper::new(T::from(el).clone())),
        ))
    }
}

/// Handles `use_window()` or `use_document()`
impl<T, E, Js> IntoElementMaybeSignalType<T, Option<Option<web_sys::Element>>> for Js
where
    Js: Deref<Target = Option<E>>,
    E: Clone,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType(Signal::stored(
            self.as_ref().map(|e| SendWrapper::new(T::from(e.clone()))),
        ))
    }
}

// From string (selector) ///////////////////////////////////////////////////////////////

pub struct StrMarker;

/// Handles `"body"` or `"#app"`
impl<T, V> IntoElementMaybeSignalType<T, StrMarker> for V
where
    V: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementMaybeSignalType(Signal::stored(None))
        } else {
            ElementMaybeSignalType(el_signal_by_sel(self.as_ref()))
        }
    }
}

pub fn el_by_sel<T>(sel: &str) -> Option<T>
where
    T: From<web_sys::Element> + Clone,
{
    document()
        .query_selector(sel)
        .unwrap_or_default()
        .map(|el| T::from(el).clone())
}

pub fn el_signal_by_sel<T>(sel: &str) -> Signal<Option<SendWrapper<T>>>
where
    T: From<web_sys::Element> + Clone + 'static,
{
    let (el_signal, set_el_signal) = signal(None);

    let sel = sel.to_string();

    set_timeout(
        move || {
            if let Some(el) = el_by_sel(&sel) {
                set_el_signal.set(Some(SendWrapper::new(el)));
            } else {
                let stop_observer = StoredValue::new_local(Rc::new(|| {}) as Rc<dyn Fn()>);

                let UseMutationObserverReturn { stop, .. } = use_mutation_observer_with_options(
                    document().body().unwrap(),
                    move |_, _| {
                        if let Some(el) = el_by_sel(&sel) {
                            set_el_signal.set(Some(SendWrapper::new(el)));
                            stop_observer.get_value()();
                        } else {
                            set_el_signal.set(None)
                        }
                    },
                    UseMutationObserverOptions::default()
                        .child_list(true)
                        .subtree(true),
                );

                stop_observer.set_value(Rc::new(stop));
            }
        },
        Duration::ZERO,
    );

    el_signal.into()
}

pub struct SignalStrMarker;

/// Handles `Signal<&str>`
impl<T, V, I> IntoElementMaybeSignalType<T, SignalStrMarker> for V
where
    V: Get<Value = I> + Send + Sync + 'static,
    I: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementMaybeSignalType(Signal::stored(None))
        } else {
            ElementMaybeSignalType(Signal::derive(move || {
                document()
                    .query_selector(self.get().as_ref())
                    .unwrap_or_default()
                    .map(|el| SendWrapper::new(T::from(el).clone()))
            }))
        }
    }
}

// From signal ///////////////////////////////////////////////////////////////

pub struct SignalMarker;

pub struct SendWrapperSignalMarker;

/// Handles `Signal<SendWrapper<web_sys::*>>`
impl<T, V, E> IntoElementMaybeSignalType<T, SendWrapperSignalMarker> for V
where
    E: Clone,
    V: Get<Value = SendWrapper<E>> + Send + Sync + 'static,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementMaybeSignalType(Signal::stored(None))
        } else {
            ElementMaybeSignalType(Signal::derive(move || {
                Some(SendWrapper::new(T::from((self.get().take()).clone())))
            }))
        }
    }
}

pub struct OptionSignalMarker;

/// Handles `Signal<Option<web_sys::*>>` and `NodeRef`
impl<T, V, E> IntoElementMaybeSignalType<T, OptionSignalMarker> for V
where
    V: Get<Value = Option<E>> + Send + Sync + 'static,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType(Signal::derive(move || {
            self.get().map(|v| SendWrapper::new(T::from(v)))
        }))
    }
}

pub struct OptionSendWrapperSignalMarker;

/// Handles `Signal<Option<SendWrapper<web_sys::*>>>` and `ElementMaybeSignal`
impl<T, V, E> IntoElementMaybeSignalType<T, OptionSendWrapperSignalMarker> for V
where
    V: Get<Value = Option<SendWrapper<E>>> + Send + Sync + 'static,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType(Signal::derive(move || {
            self.get().map(|v| SendWrapper::new(T::from(v.take())))
        }))
    }
}
