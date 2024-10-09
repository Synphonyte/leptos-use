use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::ops::Deref;

/// Used as an argument type to make it easily possible to pass either
///
/// * a `web_sys` element that implements `E` (for example `EventTarget`, `Element` or `HtmlElement`),
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>` where `T` is the web_sys element,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
///
/// into a function. Used for example in [`fn@crate::use_event_listener`].
#[derive(Copy, Clone)]
#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct ElementMaybeSignal<T: 'static> {
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    inner: ElementMaybeSignalType<T>,
}

#[derive(Clone, Copy)]
pub enum ElementMaybeSignalType<T: 'static> {
    Static(StoredValue<Option<T>, LocalStorage>),
    Dynamic(Signal<Option<T>, LocalStorage>),
}

impl<T: 'static> Default for ElementMaybeSignalType<T> {
    fn default() -> Self {
        Self::Static(StoredValue::new_local(None))
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

impl<T> With for ElementMaybeSignal<T> {
    type Value = Option<T>;

    fn try_with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match &self.inner {
            ElementMaybeSignalType::Static(v) => v.try_with_value(f),
            ElementMaybeSignalType::Dynamic(s) => s.try_with(f),
        }
    }
}

impl<T> WithUntracked for ElementMaybeSignal<T> {
    type Value = Option<T>;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match &self.inner {
            ElementMaybeSignalType::Static(t) => t.try_with_value(f),
            ElementMaybeSignalType::Dynamic(s) => s.try_with_untracked(f),
        }
    }
}

pub trait IntoElementMaybeSignal<T, Marker: ?Sized> {
    fn into_element_maybe_signal(self) -> ElementMaybeSignal<T>;
}

impl<El, T, Marker: ?Sized> IntoElementMaybeSignal<T, Marker> for El
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

pub trait IntoElementMaybeSignalType<T, Marker: ?Sized> {
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T>;
}

// From static element //////////////////////////////////////////////////////////////

/// Handles `window()` or `document()`
impl<T, Js> IntoElementMaybeSignalType<T, web_sys::Element> for Js
where
    T: From<Js> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType::Static(StoredValue::new_local(Some(T::from(self).clone())))
    }
}

/// Handles `window().body()`
impl<T, Js> IntoElementMaybeSignalType<T, Option<web_sys::Element>> for Option<Js>
where
    T: From<Js> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType::Static(StoredValue::new_local(self.map(|el| T::from(el).clone())))
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
        ElementMaybeSignalType::Static(StoredValue::new_local(
            self.as_ref().map(|e| T::from(e.clone())),
        ))
    }
}

// From string (selector) ///////////////////////////////////////////////////////////////

/// Handles `"body"` or `"#app"`
impl<T, V> IntoElementMaybeSignalType<T, str> for V
where
    V: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementMaybeSignalType::Static(StoredValue::new_local(None))
        } else {
            ElementMaybeSignalType::Static(StoredValue::new_local(
                document()
                    .query_selector(self.as_ref())
                    .unwrap_or_default()
                    .map(|el| T::from(el).clone()),
            ))
        }
    }
}

pub struct SignalStrMarker;

/// Handles `Signal<&str>`
impl<T, V, I> IntoElementMaybeSignalType<T, SignalStrMarker> for V
where
    V: Get<Value = I> + 'static,
    I: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementMaybeSignalType::Static(StoredValue::new_local(None))
        } else {
            ElementMaybeSignalType::Dynamic(Signal::derive_local(move || {
                document()
                    .query_selector(self.get().as_ref())
                    .unwrap_or_default()
                    .map(|el| T::from(el).clone())
            }))
        }
    }
}

// From signal ///////////////////////////////////////////////////////////////

pub struct SignalMarker;

/// Handles `Signal<web_sys::*>`
impl<T, V, E> IntoElementMaybeSignalType<T, SignalMarker> for V
where
    V: Get<Value = E> + 'static,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType::Dynamic(Signal::derive_local(move || Some(T::from(self.get()))))
    }
}

pub struct OptionSignalMarker;

/// Handles `Signal<Option<web_sys::*>>` and `NodeRef` and `ElementMaybeSignal`
impl<T, V, E> IntoElementMaybeSignalType<T, OptionSignalMarker> for V
where
    V: Get<Value = Option<E>> + 'static,
    T: From<E> + Clone,
{
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<T> {
        ElementMaybeSignalType::Dynamic(Signal::derive_local(move || self.get().map(T::from)))
    }
}
