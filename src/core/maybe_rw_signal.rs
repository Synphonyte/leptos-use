use leptos::prelude::*;
use std::fmt::Debug;

pub enum MaybeRwSignal<T>
where
    T: 'static,
{
    Static(T),
    DynamicRw(Signal<T>, WriteSignal<T>),
    DynamicRead(Signal<T>),
}

impl<T: Clone> Clone for MaybeRwSignal<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::DynamicRw(r, w) => Self::DynamicRw(*r, *w),
            Self::DynamicRead(s) => Self::DynamicRead(*s),
        }
    }
}

impl<T: Copy> Copy for MaybeRwSignal<T> {}

impl<T> From<T> for MaybeRwSignal<T> {
    fn from(t: T) -> Self {
        Self::Static(t)
    }
}

impl<T: Default> Default for MaybeRwSignal<T> {
    fn default() -> Self {
        Self::Static(T::default())
    }
}

impl<T: Debug> Debug for MaybeRwSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(t) => f.debug_tuple("Static").field(t).finish(),
            Self::DynamicRw(r, w) => f.debug_tuple("DynamicRw").field(r).field(w).finish(),
            Self::DynamicRead(s) => f.debug_tuple("DynamicRead").field(s).finish(),
        }
    }
}

impl<T> From<Signal<T>> for MaybeRwSignal<T> {
    fn from(s: Signal<T>) -> Self {
        Self::DynamicRead(s)
    }
}

impl<T> From<ReadSignal<T>> for MaybeRwSignal<T> {
    fn from(s: ReadSignal<T>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> From<Memo<T>> for MaybeRwSignal<T> {
    fn from(s: Memo<T>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> From<RwSignal<T>> for MaybeRwSignal<T> {
    fn from(s: RwSignal<T>) -> Self {
        let (r, w) = s.split();
        Self::DynamicRw(r.into(), w)
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for MaybeRwSignal<T> {
    fn from(s: (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::DynamicRw(s.0.into(), s.1)
    }
}

impl<T> From<(Signal<T>, WriteSignal<T>)> for MaybeRwSignal<T> {
    fn from(s: (Signal<T>, WriteSignal<T>)) -> Self {
        Self::DynamicRw(s.0, s.1)
    }
}

impl From<&str> for MaybeRwSignal<String> {
    fn from(s: &str) -> Self {
        Self::Static(s.to_string())
    }
}

impl<T: Clone> MaybeRwSignal<T> {
    pub fn into_signal(self) -> (Signal<T>, WriteSignal<T>) {
        match self {
            Self::DynamicRead(s) => {
                let (r, w) = signal(s.get_untracked());

                Effect::new(move |_| {
                    w.update(move |w| {
                        *w = s.get();
                    });
                });

                (r.into(), w)
            }
            Self::DynamicRw(r, w) => (r, w),
            Self::Static(v) => {
                let (r, w) = signal(v.clone());
                (Signal::from(r), w)
            }
        }
    }
}
