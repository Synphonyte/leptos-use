use crate::core::ElementMaybeSignal;
use crate::{UseDocument, UseWindow};
use cfg_if::cfg_if;
use leptos::html::ElementDescriptor;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
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

impl<T, E> DefinedAt for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
}

impl<T, E> With for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    type Value = Vec<Option<T>>;

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

impl<T, E> WithUntracked for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    type Value = Vec<Option<T>>;

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

macro_rules! impl_from_deref_option {
    ($ty:ty, $ty2:ty) => {
        impl<E> From<$ty> for ElementsMaybeSignal<$ty2, E>
        where
            E: From<$ty2> + 'static,
        {
            fn from(value: $ty) -> Self {
                Self::Static(vec![(*value).clone()])
            }
        }
    };
}

impl_from_deref_option!(UseWindow, web_sys::Window);
impl_from_deref_option!(UseDocument, web_sys::Document);

// From string (selector) ///////////////////////////////////////////////////////////////

impl<'a, E> From<&'a str> for ElementsMaybeSignal<web_sys::Node, E>
where
    E: From<web_sys::Node> + 'static,
{
    fn from(target: &'a str) -> Self {
        cfg_if! { if #[cfg(feature = "ssr")] {
            let _ = target;
            Self::Static(vec![])
        } else {
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
        }}
    }
}

impl<E> From<String> for ElementsMaybeSignal<web_sys::Node, E>
where
    E: From<web_sys::Node> + 'static,
{
    fn from(target: String) -> Self {
        Self::from(target.as_str())
    }
}

macro_rules! impl_from_signal_string {
    ($ty:ty) => {
        impl<E> From<$ty> for ElementsMaybeSignal<web_sys::Node, E>
        where
            E: From<web_sys::Node> + 'static,
        {
            fn from(signal: $ty) -> Self {
                cfg_if! { if #[cfg(feature = "ssr")] {
                    Self::Dynamic(
                        Signal::derive(move || {
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
                        }),
                    )
                } else {
                    let _ = signal;
                    Self::Dynamic(Signal::derive(Vec::new))
                }}
            }
        }
    };
}

impl_from_signal_string!(Signal<String>);
impl_from_signal_string!(ReadSignal<String>);
impl_from_signal_string!(RwSignal<String>);
impl_from_signal_string!(Memo<String>);

impl_from_signal_string!(Signal<&str>);
impl_from_signal_string!(ReadSignal<&str>);
impl_from_signal_string!(RwSignal<&str>);
impl_from_signal_string!(Memo<&str>);

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

// From single leptos::html::HTMLElement ///////////////////////////////////////////

macro_rules! impl_from_html_element {
    ($ty:ty) => {
        impl<HtmlEl> From<HtmlElement<HtmlEl>> for ElementsMaybeSignal<$ty, $ty>
        where
            HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty>,
        {
            fn from(value: HtmlElement<HtmlEl>) -> Self {
                let el: &$ty = value.deref();
                Self::Static(vec![Some(el.clone())])
            }
        }
    };
}

impl_from_html_element!(web_sys::EventTarget);
impl_from_html_element!(web_sys::Element);

// From multiple static elements //////////////////////////////////////////////////////

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

impl<T, E> From<Vec<T>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: Vec<T>) -> Self {
        Self::Static(target.iter().map(|t| Some(t.clone())).collect())
    }
}

impl<T, E> From<Vec<Option<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: Vec<Option<T>>) -> Self {
        Self::Static(target.to_vec())
    }
}

impl<T, E, const C: usize> From<[T; C]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: [T; C]) -> Self {
        Self::Static(target.iter().map(|t| Some(t.clone())).collect())
    }
}

impl<T, E, const C: usize> From<[Option<T>; C]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: [Option<T>; C]) -> Self {
        Self::Static(target.to_vec())
    }
}

// From multiple strings //////////////////////////////////////////////////////

macro_rules! impl_from_strings_inner {
    ($el_ty:ty, $str_ty:ty, $target:ident) => {
        Self::Static(
            $target
                .iter()
                .filter_map(|sel: &$str_ty| -> Option<Vec<Option<$el_ty>>> {
                    cfg_if! { if #[cfg(feature = "ssr")] {
                        let _ = sel;
                        None
                    } else {
                        use wasm_bindgen::JsCast;

                        if let Ok(node_list) = document().query_selector_all(sel) {
                            let mut list = Vec::with_capacity(node_list.length() as usize);
                            for i in 0..node_list.length() {
                                let node: $el_ty = node_list.get(i).expect("checked the range").unchecked_into();
                                list.push(Some(node));
                            }

                            Some(list)
                        } else {
                            None
                        }
                    }}
                })
                .flatten()
                .collect(),
        )
    };
}

macro_rules! impl_from_strings_with_container {
    ($el_ty:ty, $str_ty:ty, $container_ty:ty) => {
        impl From<$container_ty> for ElementsMaybeSignal<$el_ty, $el_ty> {
            fn from(target: $container_ty) -> Self {
                impl_from_strings_inner!($el_ty, $str_ty, target)
            }
        }
    };
}

macro_rules! impl_from_strings {
    ($el_ty:ty, $str_ty:ty) => {
        impl_from_strings_with_container!($el_ty, $str_ty, Vec<$str_ty>);
        impl_from_strings_with_container!($el_ty, $str_ty, &[$str_ty]);
        impl<const C: usize> From<[$str_ty; C]> for ElementsMaybeSignal<$el_ty, $el_ty> {
            fn from(target: [$str_ty; C]) -> Self {
                impl_from_strings_inner!($el_ty, $str_ty, target)
            }
        }
        impl<const C: usize> From<&[$str_ty; C]> for ElementsMaybeSignal<$el_ty, $el_ty> {
            fn from(target: &[$str_ty; C]) -> Self {
                impl_from_strings_inner!($el_ty, $str_ty, target)
            }
        }
    };
}

impl_from_strings!(web_sys::Element, &str);
impl_from_strings!(web_sys::Element, String);
impl_from_strings!(web_sys::EventTarget, &str);
impl_from_strings!(web_sys::EventTarget, String);

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

impl<T, E> From<Vec<Signal<T>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: Vec<Signal<T>>) -> Self {
        let list = list.clone();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| Some(t.get())).collect()
        }))
    }
}

impl<T, E> From<Vec<Signal<Option<T>>>> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: Vec<Signal<Option<T>>>) -> Self {
        let list = list.clone();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| t.get()).collect()
        }))
    }
}

impl<T, E, const C: usize> From<[Signal<T>; C]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: [Signal<T>; C]) -> Self {
        let list = list.to_vec();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| Some(t.get())).collect()
        }))
    }
}

impl<T, E, const C: usize> From<[Signal<Option<T>>; C]> for ElementsMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(list: [Signal<Option<T>>; C]) -> Self {
        let list = list.to_vec();

        Self::Dynamic(Signal::derive(move || {
            list.iter().map(|t| t.get()).collect()
        }))
    }
}

// From multiple NodeRefs //////////////////////////////////////////////////////////////

macro_rules! impl_from_multi_node_ref_inner {
    ($ty:ty, $node_refs:ident) => {
        Self::Dynamic(Signal::derive(move || {
            $node_refs
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
    };
}

macro_rules! impl_from_multi_node_ref {
    ($ty:ty) => {
        impl<R> From<&[NodeRef<R>]> for ElementsMaybeSignal<$ty, $ty>
        where
            R: ElementDescriptor + Clone + 'static,
        {
            fn from(node_refs: &[NodeRef<R>]) -> Self {
                let node_refs = node_refs.to_vec();
                impl_from_multi_node_ref_inner!($ty, node_refs)
            }
        }

        impl<R, const C: usize> From<[NodeRef<R>; C]> for ElementsMaybeSignal<$ty, $ty>
        where
            R: ElementDescriptor + Clone + 'static,
        {
            fn from(node_refs: [NodeRef<R>; C]) -> Self {
                let node_refs = node_refs.to_vec();
                impl_from_multi_node_ref_inner!($ty, node_refs)
            }
        }

        impl<R> From<Vec<NodeRef<R>>> for ElementsMaybeSignal<$ty, $ty>
        where
            R: ElementDescriptor + Clone + 'static,
        {
            fn from(node_refs: Vec<NodeRef<R>>) -> Self {
                let node_refs = node_refs.clone();
                impl_from_multi_node_ref_inner!($ty, node_refs)
            }
        }
    };
}

impl_from_multi_node_ref!(web_sys::EventTarget);
impl_from_multi_node_ref!(web_sys::Element);

// From multiple leptos::html::HTMLElement /////////////////////////////////////////

macro_rules! impl_from_multi_html_element {
    ($ty:ty) => {
        impl<HtmlEl> From<&[HtmlElement<HtmlEl>]> for ElementsMaybeSignal<$ty, $ty>
        where
            HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty>,
        {
            fn from(value: &[HtmlElement<HtmlEl>]) -> Self {
                Self::Static(
                    value
                        .iter()
                        .map(|el| {
                            let el: &$ty = el.deref();
                            Some(el.clone())
                        })
                        .collect(),
                )
            }
        }

        impl<HtmlEl, const C: usize> From<[HtmlElement<HtmlEl>; C]>
            for ElementsMaybeSignal<$ty, $ty>
        where
            HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty>,
        {
            fn from(value: [HtmlElement<HtmlEl>; C]) -> Self {
                Self::Static(
                    value
                        .iter()
                        .map(|el| {
                            let el: &$ty = el.deref();
                            Some(el.clone())
                        })
                        .collect(),
                )
            }
        }

        impl<HtmlEl> From<Vec<HtmlElement<HtmlEl>>> for ElementsMaybeSignal<$ty, $ty>
        where
            HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty>,
        {
            fn from(value: Vec<HtmlElement<HtmlEl>>) -> Self {
                Self::Static(
                    value
                        .iter()
                        .map(|el| {
                            let el: &$ty = el.deref();
                            Some(el.clone())
                        })
                        .collect(),
                )
            }
        }
    };
}

impl_from_multi_html_element!(web_sys::EventTarget);
impl_from_multi_html_element!(web_sys::Element);

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
