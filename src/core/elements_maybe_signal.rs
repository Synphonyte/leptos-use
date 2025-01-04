use crate::core::{SignalMarker, SignalStrMarker};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::ops::Deref;

/// Used as an argument type to make it easily possible to pass either
///
/// * a `web_sys` element that implements `E` (for example `EventTarget` or `Element`),
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>` where `T` is the web_sys element,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
///
/// into a function. Used for example in [`fn@crate::use_event_listener`].
#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct ElementsMaybeSignal<T: 'static> {
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    inner: ElementsMaybeSignalType<T>,
}

impl<T> Clone for ElementsMaybeSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementsMaybeSignal<T> {}

pub enum ElementsMaybeSignalType<T: 'static> {
    Static(StoredValue<Vec<Option<T>>, LocalStorage>),
    Dynamic(Signal<Vec<Option<T>>, LocalStorage>),
}

impl<T> Clone for ElementsMaybeSignalType<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementsMaybeSignalType<T> {}

impl<T: 'static> Default for ElementsMaybeSignalType<T> {
    fn default() -> Self {
        Self::Static(StoredValue::new_local(vec![]))
    }
}

impl<T> Default for ElementsMaybeSignal<T> {
    fn default() -> Self {
        Self {
            inner: ElementsMaybeSignalType::default(),
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }
}

impl<T> DefinedAt for ElementsMaybeSignal<T> {
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

impl<T> With for ElementsMaybeSignal<T> {
    type Value = Vec<Option<T>>;

    fn try_with<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> Option<O> {
        match &self.inner {
            ElementsMaybeSignalType::Static(v) => v.try_with_value(f),
            ElementsMaybeSignalType::Dynamic(s) => s.try_with(f),
        }
    }
}

impl<T> WithUntracked for ElementsMaybeSignal<T> {
    type Value = Vec<Option<T>>;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> Option<O> {
        match self.inner {
            ElementsMaybeSignalType::Static(t) => t.try_with_value(f),
            ElementsMaybeSignalType::Dynamic(s) => s.try_with_untracked(f),
        }
    }
}

pub trait IntoElementsMaybeSignal<T, Marker: ?Sized> {
    fn into_elements_maybe_signal(self) -> ElementsMaybeSignal<T>;
}

impl<El, T, Marker: ?Sized> IntoElementsMaybeSignal<T, Marker> for El
where
    El: IntoElementsMaybeSignalType<T, Marker>,
{
    fn into_elements_maybe_signal(self) -> ElementsMaybeSignal<T> {
        ElementsMaybeSignal {
            inner: self.into_elements_maybe_signal_type(),
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }
}

pub trait IntoElementsMaybeSignalType<T, Marker: ?Sized> {
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T>;
}

// From single static element //////////////////////////////////////////////////////////////

/// Handles `window()` or `document()`
impl<T, Js> IntoElementsMaybeSignalType<T, web_sys::Element> for Js
where
    T: From<Js> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(vec![Some(T::from(self).clone())]))
    }
}

/// Handles `window().body()`
impl<T, Js> IntoElementsMaybeSignalType<T, Option<web_sys::Element>> for Option<Js>
where
    T: From<Js> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(vec![
            self.map(|el| T::from(el).clone())
        ]))
    }
}

/// Handles `use_window()` or `use_document()`
impl<T, E, Js> IntoElementsMaybeSignalType<T, Option<Option<web_sys::Element>>> for Js
where
    Js: Deref<Target = Option<E>>,
    E: Clone,
    T: From<E> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(vec![self
            .as_ref()
            .map(|e| T::from(e.clone()))]))
    }
}

// From string (selector) ///////////////////////////////////////////////////////////////

/// Handles `"body"` or `"#app"`
impl<T, V> IntoElementsMaybeSignalType<T, str> for V
where
    V: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementsMaybeSignalType::Static(StoredValue::new_local(vec![]))
        } else {
            ElementsMaybeSignalType::Static(StoredValue::new_local(vec![document()
                .query_selector(self.as_ref())
                .unwrap_or_default()
                .map(|el| T::from(el).clone())]))
        }
    }
}

/// Handles `Signal<&str>`
impl<T, V, I> IntoElementsMaybeSignalType<T, SignalStrMarker> for V
where
    V: Get<Value = I> + 'static,
    I: AsRef<str>,
    T: From<web_sys::Element> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementsMaybeSignalType::Static(StoredValue::new_local(vec![]))
        } else {
            ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
                vec![document()
                    .query_selector(self.get().as_ref())
                    .unwrap_or_default()
                    .map(|el| T::from(el).clone())]
            }))
        }
    }
}

// From single signal ///////////////////////////////////////////////////////////////

/// Handles `Signal<web_sys::*>`
impl<T, V, E> IntoElementsMaybeSignalType<T, SignalMarker> for V
where
    V: Get<Value = E> + 'static,
    T: From<E> + Clone,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
            vec![Some(T::from(self.get()))]
        }))
    }
}

// From multiple static elements //////////////////////////////////////////////////////

/// Handles `&[web_sys::*]`
impl<'a, T, Js, C> IntoElementsMaybeSignalType<T, &'a [web_sys::Element]> for C
where
    Js: Clone + 'a,
    T: From<Js>,
    C: IntoIterator<Item = &'a Js>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(
            self.into_iter().map(|t| Some(T::from(t.clone()))).collect(),
        ))
    }
}

/// Handles `&[Option<web_sys::*>]`
impl<'a, T, Js, C> IntoElementsMaybeSignalType<T, &'a [Option<web_sys::Element>]> for C
where
    Js: Clone + 'a,
    T: From<Js>,
    C: IntoIterator<Item = &'a Option<Js>>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(
            self.into_iter().map(|t| t.clone().map(T::from)).collect(),
        ))
    }
}

/// Handles `Vec<web_sys::*>`
impl<T, Js, C> IntoElementsMaybeSignalType<T, Vec<web_sys::Element>> for C
where
    T: From<Js> + Clone,
    C: IntoIterator<Item = Js>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(
            self.into_iter().map(|t| Some(T::from(t))).collect(),
        ))
    }
}

/// Handles `Vec<Option<web_sys::*>>`
impl<T, Js, C> IntoElementsMaybeSignalType<T, Vec<Option<web_sys::Element>>> for C
where
    T: From<Js> + Clone,
    C: IntoIterator<Item = Option<Js>>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Static(StoredValue::new_local(
            self.into_iter().map(|t| t.map(T::from)).collect(),
        ))
    }
}

// From multiple strings //////////////////////////////////////////////////////

/// Handles `["body", "#app"]`
impl<T, V, C> IntoElementsMaybeSignalType<T, &[&str]> for C
where
    V: AsRef<str>,
    T: From<web_sys::Element> + Clone,
    C: IntoIterator<Item = V>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        if cfg!(feature = "ssr") {
            ElementsMaybeSignalType::Static(StoredValue::new_local(vec![]))
        } else {
            ElementsMaybeSignalType::Static(StoredValue::new_local(
                self.into_iter()
                    .map(|sel| {
                        document()
                            .query_selector(sel.as_ref())
                            .unwrap_or_default()
                            .map(|el| T::from(el).clone())
                    })
                    .collect(),
            ))
        }
    }
}

// From signal of multiple ////////////////////////////////////////////////////////////////

pub struct SignalVecMarker;

/// Handles `Signal<Vec<web_sys::*>>` and `Signal<Option<web_sys::*>>` and `NodeRef` and `ElementMaybeSignal`
impl<T, Js, C, G> IntoElementsMaybeSignalType<T, SignalVecMarker> for G
where
    T: From<Js> + Clone,
    G: Get<Value = C> + 'static,
    C: IntoIterator<Item = Js>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
            self.get().into_iter().map(|t| Some(T::from(t))).collect()
        }))
    }
}

pub struct SignalVecOptionMarker;

/// Handles `Signal<Vec<Option<web_sys::*>>>`
impl<T, Js, C, G> IntoElementsMaybeSignalType<T, SignalVecOptionMarker> for G
where
    T: From<Js> + Clone,
    G: Get<Value = C> + 'static,
    C: IntoIterator<Item = Option<Js>>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
            self.get().into_iter().map(|t| t.map(T::from)).collect()
        }))
    }
}

// From multiple signals //////////////////////////////////////////////////////////////

pub struct VecSignalMarker;

/// Handles `Vec<Signal<web_sys::*>>`
impl<T, Js, C, G> IntoElementsMaybeSignalType<T, VecSignalMarker> for C
where
    T: From<Js> + Clone,
    C: IntoIterator<Item = G> + Clone + 'static,
    G: Get<Value = Js>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        let signals = self.clone();

        ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
            signals
                .clone()
                .into_iter()
                .map(|t| Some(T::from(t.get())))
                .collect()
        }))
    }
}

pub struct VecSignalOptionMarker;

/// Handles `Vec<Signal<Option<web_sys::*>>>`, `Vec<NodeRef>`
impl<T, Js, C, G> IntoElementsMaybeSignalType<T, VecSignalOptionMarker> for C
where
    T: From<Js> + Clone,
    C: IntoIterator<Item = G> + Clone + 'static,
    G: Get<Value = Option<Js>>,
{
    fn into_elements_maybe_signal_type(self) -> ElementsMaybeSignalType<T> {
        let signals = self.clone();

        ElementsMaybeSignalType::Dynamic(Signal::derive_local(move || {
            signals
                .clone()
                .into_iter()
                .map(|t| t.get().map(T::from))
                .collect()
        }))
    }
}
