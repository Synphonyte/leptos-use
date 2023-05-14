use leptos::html::ElementDescriptor;
use leptos::*;
use std::ops::Deref;

pub enum EventTargetMaybeSignal<T>
where
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    Static(Option<T>),
    Dynamic(Signal<Option<T>>),
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
