use crate::core::ElementMaybeSignal;
use leptos::html::ElementDescriptor;
use leptos::*;
use std::marker::PhantomData;
use std::ops::Deref;

/// Used as an argument type to make it easily possible to pass either
/// * a `web_sys` element that implements `E` (for example `EventTarget` or `Element`),
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>` where `T` is the web_sys element,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
/// into a function. Used for example in [`use_event_listener`].
pub enum ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    Static(Vec<Option<T>>),
    Dynamic(Signal<Vec<Option<T>>>),
    _Phantom(PhantomData<E>),
}

impl<T, E> Default for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn default() -> Self {
        Self::Static(vec![])
    }
}

impl<T, E> Clone for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Static(v) => Self::Static(v.clone()),
            Self::Dynamic(s) => Self::Dynamic(*s),
            Self::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T, E> SignalGet<Vec<Option<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn get(&self) -> Vec<Option<T>> {
        match self {
            Self::Static(v) => v.clone(),
            Self::Dynamic(s) => s.get(),
            Self::_Phantom(_) => unreachable!(),
        }
    }

    fn try_get(&self) -> Option<Vec<Option<T>>> {
        match self {
            Self::Static(v) => Some(v.clone()),
            Self::Dynamic(s) => s.try_get(),
            Self::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T, E> SignalWith<Vec<Option<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn with<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> O {
        match self {
            Self::Static(v) => f(v),
            Self::Dynamic(s) => s.with(f),
            Self::_Phantom(_) => unreachable!(),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> Option<O> {
        match self {
            Self::Static(v) => Some(f(v)),
            Self::Dynamic(s) => s.try_with(f),
            Self::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T, E> SignalWithUntracked<Vec<Option<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn with_untracked<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> O {
        match self {
            Self::Static(t) => f(t),
            Self::Dynamic(s) => s.with_untracked(f),
            Self::_Phantom(_) => unreachable!(),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Vec<Option<T>>) -> O) -> Option<O> {
        match self {
            Self::Static(t) => Some(f(t)),
            Self::Dynamic(s) => s.try_with_untracked(f),
            Self::_Phantom(_) => unreachable!(),
        }
    }
}

impl<T, E> SignalGetUntracked<Vec<Option<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn get_untracked(&self) -> Vec<Option<T>> {
        match self {
            Self::Static(t) => t.clone(),
            Self::Dynamic(s) => s.get_untracked(),
            Self::_Phantom(_) => unreachable!(),
        }
    }

    fn try_get_untracked(&self) -> Option<Vec<Option<T>>> {
        match self {
            Self::Static(t) => Some(t.clone()),
            Self::Dynamic(s) => s.try_get_untracked(),
            Self::_Phantom(_) => unreachable!(),
        }
    }
}

// From single static element //////////////////////////////////////////////////////////////

impl<T, E> From<T> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(value: T) -> Self {
        ElementsMaybeSignal::Static(vec![Some(value)])
    }
}

impl<T, E> From<Option<T>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: Option<T>) -> Self {
        ElementsMaybeSignal::Static(vec![target])
    }
}

// From string (selector) ///////////////////////////////////////////////////////////////

impl<'a, E> From<&'a str> for ElementsMaybeSignal<web_sys::Node, E>
where
    E: From<web_sys::Node> + 'static,
{
    fn from(target: &'a str) -> Self {
        if let Ok(node_list) = document().query_selector_all(target) {
            let mut list = Vec::with_capacity(node_list.length() as usize);
            for i in 0..node_list.length() {
                let node = node_list.get(i).expect("checked the range");
                list.push(Some(node));
            }

            Self::Static(list)
        } else {
            Self::Static(vec![])
        }
    }
}

impl<E> From<Signal<String>> for ElementsMaybeSignal<web_sys::Node, E>
where
    E: From<web_sys::Node> + 'static,
{
    fn from(signal: Signal<String>) -> Self {
        Self::Dynamic(
            create_memo(move |_| {
                if let Ok(node_list) = document().query_selector_all(&signal.get()) {
                    let mut list = Vec::with_capacity(node_list.length() as usize);
                    for i in 0..node_list.length() {
                        let node = node_list.get(i).expect("checked the range");
                        list.push(Some(node));
                    }
                    list
                } else {
                    vec![]
                }
            })
            .into(),
        )
    }
}

// From single signal ///////////////////////////////////////////////////////////////

macro_rules! impl_from_signal_option {
    ($ty:ty) => {
        impl<T, E> From<$ty> for ElementsMaybeSignal<T, E>
        where
            T: Into<E> + Clone + 'static,
        {
            fn from(signal: $ty) -> Self {
                Self::Dynamic(Signal::derive(move || vec![signal.get()]))
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
        impl<T, E> From<$ty> for ElementsMaybeSignal<T, E>
        where
            T: Into<E> + Clone + 'static,
        {
            fn from(signal: $ty) -> Self {
                Self::Dynamic(Signal::derive(move || vec![Some(signal.get())]))
            }
        }
    };
}

impl_from_signal!(Signal<T>);
impl_from_signal!(ReadSignal<T>);
impl_from_signal!(RwSignal<T>);
impl_from_signal!(Memo<T>);

// From single NodeRef //////////////////////////////////////////////////////////////

macro_rules! impl_from_node_ref {
    ($ty:ty) => {
        impl<R> From<NodeRef<R>> for ElementsMaybeSignal<$ty, $ty>
        where
            R: ElementDescriptor + Clone + 'static,
        {
            fn from(node_ref: NodeRef<R>) -> Self {
                Self::Dynamic(Signal::derive(move || {
                    vec![node_ref.get().map(move |el| {
                        let el = el.into_any();
                        let el: $ty = el.deref().clone().into();
                        el
                    })]
                }))
            }
        }
    };
}

impl_from_node_ref!(web_sys::EventTarget);
impl_from_node_ref!(web_sys::Element);

// From multiple static elements //////////////////////////////////////////////////////////////

impl<T, E> From<&[T]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: &[T]) -> Self {
        Self::Static(target.iter().map(|t| Some(t.clone())).collect())
    }
}

impl<T, E> From<&[Option<T>]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: &[Option<T>]) -> Self {
        Self::Static(target.to_vec())
    }
}

// From signal of vec ////////////////////////////////////////////////////////////////

impl<T, E> From<Signal<Vec<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(signal: Signal<Vec<T>>) -> Self {
        Self::Dynamic(Signal::derive(move || {
            signal.get().into_iter().map(|t| Some(t)).collect()
        }))
    }
}

impl<T, E> From<Signal<Vec<Option<T>>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: Signal<Vec<Option<T>>>) -> Self {
        Self::Dynamic(target)
    }
}

// From multiple signals //////////////////////////////////////////////////////////////

impl<T, E> From<&[Signal<T>]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: &[Signal<T>]) -> Self {
        let list = list.to_vec();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| Some(t.get())).collect()
        }))
    }
}

impl<T, E> From<&[Signal<Option<T>>]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: &[Signal<Option<T>>]) -> Self {
        let list = list.to_vec();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| t.get()).collect()
        }))
    }
}

// From multiple NodeRefs //////////////////////////////////////////////////////////////

macro_rules! impl_from_multi_node_ref {
    ($ty:ty) => {
        impl<R> From<&[NodeRef<R>]> for ElementsMaybeSignal<$ty, $ty>
        where
            R: ElementDescriptor + Clone + 'static,
        {
            fn from(node_refs: &[NodeRef<R>]) -> Self {
                let node_refs = node_refs.to_vec();

                Self::Dynamic(Signal::derive(move || {
                    node_refs
                        .iter()
                        .map(|node_ref| {
                            node_ref.get().map(move |el| {
                                let el = el.into_any();
                                let el: $ty = el.deref().clone().into();
                                el
                            })
                        })
                        .collect()
                }))
            }
        }
    };
}

impl_from_multi_node_ref!(web_sys::EventTarget);
impl_from_multi_node_ref!(web_sys::Element);

// From ElementMaybeSignal //////////////////////////////////////////////////////////////

impl<T, E> From<ElementMaybeSignal<T, E>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(signal: ElementMaybeSignal<T, E>) -> Self {
        match signal {
            ElementMaybeSignal::Dynamic(signal) => {
                Self::Dynamic(Signal::derive(move || vec![signal.get()]))
            }
            ElementMaybeSignal::Static(el) => Self::Static(vec![el]),
            ElementMaybeSignal::_Phantom(_) => unreachable!(),
        }
    }
}
