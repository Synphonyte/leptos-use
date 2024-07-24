use crate::{UseDocument, UseWindow};
use cfg_if::cfg_if;
use leptos::html::{CreateElement, ElementType};
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::marker::PhantomData;
use wasm_bindgen::JsCast;

/// Used as an argument type to make it easily possible to pass either
/// * a `web_sys` element that implements `E` (for example `EventTarget`, `Element` or `HtmlElement`),
/// * an `Option<T>` where `T` is the web_sys element,
/// * a `Signal<T>` where `T` is the web_sys element,
/// * a `Signal<Option<T>>` where `T` is the web_sys element,
/// * a `NodeRef`
/// into a function. Used for example in [`use_event_listener`].
pub enum ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    Static(Option<SendWrapper<T>>),
    Dynamic(Signal<Option<SendWrapper<T>>>),
    _Phantom(PhantomData<fn() -> E>),
}

impl<T, E> Default for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn default() -> Self {
        Self::Static(None)
    }
}

impl<T, E> Clone for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::Dynamic(s) => Self::Dynamic(*s),
            _ => unreachable!(),
        }
    }
}

impl<T, E> DefinedAt for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
}

impl<T, E> With for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    type Value = Option<T>;

    fn with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            Self::Static(t) => {
                let value = t.as_ref().map(|t| t.clone().take());
                f(&value)
            }
            Self::Dynamic(s) => s.with(|s| {
                let value = s.as_ref().map(|s| s.clone().take());
                f(&value)
            }),
            _ => unreachable!(),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            Self::Static(t) => {
                let value = t.as_ref().map(|t| t.clone().take());
                Some(f(&value))
            }
            Self::Dynamic(s) => s.try_with(|s| {
                let value = s.as_ref().map(|s| s.clone().take());
                f(&value)
            }),
            _ => unreachable!(),
        }
    }
}

impl<T, E> WithUntracked for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    type Value = Option<T>;

    fn with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            Self::Static(t) => {
                let value = t.as_ref().map(|t| t.clone().take());
                f(&value)
            }
            Self::Dynamic(s) => s.with_untracked(|s| {
                let value = s.as_ref().map(|s| s.clone().take());
                f(&value)
            }),
            _ => unreachable!(),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            Self::Static(t) => {
                let value = t.as_ref().map(|t| t.clone().take());
                Some(f(&value))
            }
            Self::Dynamic(s) => s.try_with_untracked(|s| {
                let value = s.as_ref().map(|s| s.clone().take());
                f(&value)
            }),
            _ => unreachable!(),
        }
    }
}

// From static element //////////////////////////////////////////////////////////////

impl<T, E> From<T> for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(value: T) -> Self {
        ElementMaybeSignal::Static(Some(SendWrapper::new(value)))
    }
}

impl<T, E> From<Option<T>> for ElementMaybeSignal<T, E>
where
    T: Into<E> + Clone + 'static,
{
    fn from(target: Option<T>) -> Self {
        ElementMaybeSignal::Static(target.map(SendWrapper::new))
    }
}

macro_rules! impl_from_deref_option {
    ($ty:ty, $ty2:ty) => {
        impl<E> From<$ty> for ElementMaybeSignal<$ty2, E>
        where
            E: From<$ty2> + 'static,
        {
            fn from(value: $ty) -> Self {
                Self::Static((*value).clone().map(SendWrapper::new))
            }
        }
    };
}

impl_from_deref_option!(UseWindow, web_sys::Window);
impl_from_deref_option!(UseDocument, web_sys::Document);

// From string (selector) ///////////////////////////////////////////////////////////////

impl<'a, E> From<&'a str> for ElementMaybeSignal<web_sys::Element, E>
where
    E: From<web_sys::Element> + 'static,
{
    fn from(target: &'a str) -> Self {
        cfg_if! { if #[cfg(feature = "ssr")] {
            let _ = target;
            Self::Static(None)
        } else {
            Self::Static(document().query_selector(target).unwrap_or_default().map(SendWrapper::new))
        }}
    }
}

impl<E> From<String> for ElementMaybeSignal<web_sys::Element, E>
where
    E: From<web_sys::Element> + 'static,
{
    fn from(target: String) -> Self {
        Self::from(target.as_str())
    }
}

macro_rules! impl_from_signal_string {
    ($ty:ty) => {
        impl<E> From<$ty> for ElementMaybeSignal<web_sys::Element, E>
        where
            E: From<web_sys::Element> + 'static,
        {
            fn from(signal: $ty) -> Self {
                cfg_if! { if #[cfg(feature = "ssr")] {
                    let _ = signal;
                    Self::Dynamic(Signal::derive(|| None))
                } else {
                    Self::Dynamic(
                        Signal::derive(move || document().query_selector(&signal.get()).unwrap_or_default().map(SendWrapper::new)),
                    )
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
impl_from_signal_string!(RwSignal<&'static str>);
impl_from_signal_string!(Memo<&'static str>);

// From signal ///////////////////////////////////////////////////////////////

macro_rules! impl_from_signal_option {
    ($ty:ty) => {
        impl<T, E> From<$ty> for ElementMaybeSignal<T, E>
        where
            T: Into<E> + Clone + 'static,
        {
            fn from(target: $ty) -> Self {
                Self::Dynamic(target.into())
            }
        }
    };
}

impl_from_signal_option!(Signal<Option<SendWrapper<T>>>);
impl_from_signal_option!(ReadSignal<Option<SendWrapper<T>>>);
impl_from_signal_option!(RwSignal<Option<SendWrapper<T>>>);
impl_from_signal_option!(Memo<Option<SendWrapper<T>>>);

macro_rules! impl_from_signal {
    ($ty:ty) => {
        impl<T, E> From<$ty> for ElementMaybeSignal<T, E>
        where
            T: Into<E> + Clone + 'static,
        {
            fn from(signal: $ty) -> Self {
                Self::Dynamic(Signal::derive(move || Some(signal.get())))
            }
        }
    };
}

impl_from_signal!(Signal<SendWrapper<T>>);
impl_from_signal!(ReadSignal<SendWrapper<T>>);
impl_from_signal!(RwSignal<SendWrapper<T>>);
impl_from_signal!(Memo<SendWrapper<T>>);

// From NodeRef //////////////////////////////////////////////////////////////

macro_rules! impl_from_node_ref {
    ($ty:ty) => {
        impl<R> From<NodeRef<R>> for ElementMaybeSignal<$ty, $ty>
        where
            R: ElementType + CreateElement<Dom> + Clone + Send + Sync + 'static,
            R::Output: JsCast + Into<$ty> + Clone + 'static,
        {
            fn from(node_ref: NodeRef<R>) -> Self {
                Self::Dynamic(Signal::derive(move || {
                    node_ref.get().map(move |el| {
                        let el: $ty = el.clone().into();
                        SendWrapper::new(el)
                    })
                }))
            }
        }
    };
}

impl_from_node_ref!(web_sys::EventTarget);
impl_from_node_ref!(web_sys::Element);
impl_from_node_ref!(web_sys::HtmlElement);

// // From leptos::html::HTMLElement ///////////////////////////////////////////////
//
// macro_rules! impl_from_html_element {
//     ($ty:ty) => {
//         impl<HtmlEl> From<HtmlElement<HtmlEl>> for ElementMaybeSignal<$ty, $ty>
//         where
//             HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty>,
//         {
//             fn from(value: HtmlElement<HtmlEl>) -> Self {
//                 let el: &$ty = value.deref();
//                 Self::Static(Some(el.clone()))
//             }
//         }
//     };
// }
//
// impl_from_html_element!(web_sys::EventTarget);
// impl_from_html_element!(web_sys::Element);
// impl_from_html_element!(web_sys::HtmlElement);
//
// // From Signal<leptos::html::HTMLElement> /////////////////////////////////////////
//
// macro_rules! impl_from_signal_html_element {
//     ($signal:ty, $ty:ty) => {
//         impl<HtmlEl> From<$signal> for ElementMaybeSignal<$ty, $ty>
//         where
//             HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty> + Clone,
//         {
//             fn from(value: $signal) -> Self {
//                 Self::Dynamic(Signal::derive(move || {
//                     let value = value.get();
//                     let el: &$ty = value.deref();
//                     Some(el.clone())
//                 }))
//             }
//         }
//     };
// }
//
// impl_from_signal_html_element!(Signal<HtmlElement<HtmlEl>>, web_sys::EventTarget);
// impl_from_signal_html_element!(ReadSignal<HtmlElement<HtmlEl>>, web_sys::EventTarget);
// impl_from_signal_html_element!(RwSignal<HtmlElement<HtmlEl>>, web_sys::EventTarget);
// impl_from_signal_html_element!(Memo<HtmlElement<HtmlEl>>, web_sys::EventTarget);
//
// impl_from_signal_html_element!(Signal<HtmlElement<HtmlEl>>, web_sys::Element);
// impl_from_signal_html_element!(ReadSignal<HtmlElement<HtmlEl>>, web_sys::Element);
// impl_from_signal_html_element!(RwSignal<HtmlElement<HtmlEl>>, web_sys::Element);
// impl_from_signal_html_element!(Memo<HtmlElement<HtmlEl>>, web_sys::Element);
//
// // From Signal<Option<leptos::html::HTMLElement>> /////////////////////////////////////////
//
// macro_rules! impl_from_signal_html_element {
//     ($signal:ty, $ty:ty) => {
//         impl<HtmlEl> From<$signal> for ElementMaybeSignal<$ty, $ty>
//         where
//             HtmlEl: ElementDescriptor + std::ops::Deref<Target = $ty> + Clone,
//         {
//             fn from(value: $signal) -> Self {
//                 Self::Dynamic(Signal::derive(move || {
//                     let el: Option<$ty> = value.get().map(|el| el.deref().clone());
//                     el
//                 }))
//             }
//         }
//     };
// }
//
// impl_from_signal_html_element!(Signal<Option<HtmlElement<HtmlEl>>>, web_sys::EventTarget);
// impl_from_signal_html_element!(
//     ReadSignal<Option<HtmlElement<HtmlEl>>>,
//     web_sys::EventTarget
// );
// impl_from_signal_html_element!(RwSignal<Option<HtmlElement<HtmlEl>>>, web_sys::EventTarget);
// impl_from_signal_html_element!(Memo<Option<HtmlElement<HtmlEl>>>, web_sys::EventTarget);
//
// impl_from_signal_html_element!(Signal<Option<HtmlElement<HtmlEl>>>, web_sys::Element);
// impl_from_signal_html_element!(ReadSignal<Option<HtmlElement<HtmlEl>>>, web_sys::Element);
// impl_from_signal_html_element!(RwSignal<Option<HtmlElement<HtmlEl>>>, web_sys::Element);
// impl_from_signal_html_element!(Memo<Option<HtmlElement<HtmlEl>>>, web_sys::Element);
//
// impl_from_signal_html_element!(Signal<Option<HtmlElement<HtmlEl>>>, web_sys::HtmlElement);
// impl_from_signal_html_element!(
//     ReadSignal<Option<HtmlElement<HtmlEl>>>,
//     web_sys::HtmlElement
// );
// impl_from_signal_html_element!(RwSignal<Option<HtmlElement<HtmlEl>>>, web_sys::HtmlElement);
// impl_from_signal_html_element!(Memo<Option<HtmlElement<HtmlEl>>>, web_sys::HtmlElement);
