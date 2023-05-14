use leptos::html::ElementDescriptor;
use leptos::*;
use std::ops::Deref;

/// Used as an argument type to make it easily possible to pass either
/// * a `web_sys` element that implements `EventTarget`,
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>` where `T` is the web_sys element,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
/// into a function. Used for example in [`use_event_listener`].
pub enum EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    Static(Option<T>),
    Dynamic(Signal<Option<T>>),
}

impl<T> Default for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn default() -> Self {
        Self::Static(None)
    }
}

impl<T> Clone for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::Dynamic(s) => Self::Dynamic(*s),
        }
    }
}

impl<T> SignalGet<Option<T>> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn get(&self) -> Option<T> {
        match self {
            Self::Static(t) => t.clone(),
            Self::Dynamic(s) => s.get(),
        }
    }

    fn try_get(&self) -> Option<Option<T>> {
        match self {
            Self::Static(t) => Some(t.clone()),
            Self::Dynamic(s) => s.try_get(),
        }
    }
}

impl<T> SignalWith<Option<T>> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            Self::Static(t) => f(t),
            Self::Dynamic(s) => s.with(f),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            Self::Static(t) => Some(f(t)),
            Self::Dynamic(s) => s.try_with(f),
        }
    }
}

impl<T> SignalWithUntracked<Option<T>> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            Self::Static(t) => f(t),
            Self::Dynamic(s) => s.with_untracked(f),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            Self::Static(t) => Some(f(t)),
            Self::Dynamic(s) => s.try_with_untracked(f),
        }
    }
}

impl<T> SignalGetUntracked<Option<T>> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn get_untracked(&self) -> Option<T> {
        match self {
            Self::Static(t) => t.clone(),
            Self::Dynamic(s) => s.get_untracked(),
        }
    }

    fn try_get_untracked(&self) -> Option<Option<T>> {
        match self {
            Self::Static(t) => Some(t.clone()),
            Self::Dynamic(s) => s.try_get_untracked(),
        }
    }
}

impl<T> From<(Scope, T)> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn from(value: (Scope, T)) -> Self {
        EventTargetMaybeSignal::Static(Some(value.1))
    }
}

impl<T> From<(Scope, Option<T>)> for EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    fn from(target: (Scope, Option<T>)) -> Self {
        EventTargetMaybeSignal::Static(target.1)
    }
}

macro_rules! impl_from_signal_option {
    ($ty:ty) => {
        impl<T> From<(Scope, $ty)> for EventTargetMaybeSignal<T>
        where
            T: Into<web_sys::EventTarget> + Clone + 'static,
        {
            fn from(target: (Scope, $ty)) -> Self {
                EventTargetMaybeSignal::Dynamic(target.1.into())
            }
        }
    };
}

impl_from_signal_option!(Signal<Option<T>>);
impl_from_signal_option!(ReadSignal<Option<T>>);
impl_from_signal_option!(RwSignal<Option<T>>);
impl_from_signal_option!(Memo<Option<T>>);

macro_rules! impl_from_signal {
    ($ty:ty) => {
        impl<T> From<(Scope, $ty)> for EventTargetMaybeSignal<T>
        where
            T: Into<web_sys::EventTarget> + Clone + 'static,
        {
            fn from(target: (Scope, $ty)) -> Self {
                let (cx, signal) = target;

                EventTargetMaybeSignal::Dynamic(Signal::derive(cx, move || Some(signal.get())))
            }
        }
    };
}

impl_from_signal!(Signal<T>);
impl_from_signal!(ReadSignal<T>);
impl_from_signal!(RwSignal<T>);
impl_from_signal!(Memo<T>);

impl<R> From<(Scope, NodeRef<R>)> for EventTargetMaybeSignal<web_sys::EventTarget>
where
    R: ElementDescriptor + Clone + 'static,
{
    fn from(target: (Scope, NodeRef<R>)) -> Self {
        let (cx, node_ref) = target;

        EventTargetMaybeSignal::Dynamic(Signal::derive(cx, move || {
            node_ref.get().map(move |el| {
                let el = el.into_any();
                let el: web_sys::EventTarget = el.deref().clone().into();
                el
            })
        }))
    }
}
