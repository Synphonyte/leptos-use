use leptos::prelude::*;
use std::fmt::Debug;

pub enum MaybeRwSignal<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    Static(T),
    DynamicRw(Signal<T, S>, WriteSignal<T, S>),
    DynamicRead(Signal<T, S>),
}

impl<T: Clone, S> Clone for MaybeRwSignal<T, S>
where
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::DynamicRw(r, w) => Self::DynamicRw(*r, *w),
            Self::DynamicRead(s) => Self::DynamicRead(*s),
        }
    }
}

impl<T: Copy, S> Copy for MaybeRwSignal<T, S> where S: Storage<T> {}

impl<T: Default, S> Default for MaybeRwSignal<T, S>
where
    S: Storage<T>,
{
    fn default() -> Self {
        Self::Static(T::default())
    }
}

impl<T: Debug, S> Debug for MaybeRwSignal<T, S>
where
    S: Storage<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(t) => f.debug_tuple("Static").field(t).finish(),
            Self::DynamicRw(r, w) => f.debug_tuple("DynamicRw").field(r).field(w).finish(),
            Self::DynamicRead(s) => f.debug_tuple("DynamicRead").field(s).finish(),
        }
    }
}

impl<T> From<T> for MaybeRwSignal<T, SyncStorage>
where
    SyncStorage: Storage<T>,
{
    fn from(t: T) -> Self {
        Self::Static(t)
    }
}

impl<T> FromLocal<T> for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(value: T) -> Self {
        Self::Static(value)
    }
}

impl<T> From<Signal<T>> for MaybeRwSignal<T>
where
    SyncStorage: Storage<T>,
{
    fn from(s: Signal<T>) -> Self {
        Self::DynamicRead(s)
    }
}

impl<T> FromLocal<Signal<T, LocalStorage>> for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: Signal<T, LocalStorage>) -> Self {
        Self::DynamicRead(s)
    }
}

impl<T> From<ReadSignal<T>> for MaybeRwSignal<T>
where
    T: Send + Sync,
{
    fn from(s: ReadSignal<T>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> FromLocal<ReadSignal<T, LocalStorage>> for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: ReadSignal<T, LocalStorage>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> From<Memo<T>> for MaybeRwSignal<T>
where
    T: Send + Sync,
{
    fn from(s: Memo<T>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> FromLocal<Memo<T, LocalStorage>> for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: Memo<T, LocalStorage>) -> Self {
        Self::DynamicRead(s.into())
    }
}

impl<T> From<RwSignal<T>> for MaybeRwSignal<T>
where
    T: Send + Sync,
{
    fn from(s: RwSignal<T>) -> Self {
        let (r, w) = s.split();
        Self::DynamicRw(r.into(), w)
    }
}

impl<T> FromLocal<RwSignal<T, LocalStorage>> for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: RwSignal<T, LocalStorage>) -> Self {
        let (r, w) = s.split();
        Self::DynamicRw(r.into(), w)
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for MaybeRwSignal<T>
where
    T: Send + Sync,
{
    fn from(s: (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::DynamicRw(s.0.into(), s.1)
    }
}

impl<T> FromLocal<(ReadSignal<T, LocalStorage>, WriteSignal<T, LocalStorage>)>
    for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: (ReadSignal<T, LocalStorage>, WriteSignal<T, LocalStorage>)) -> Self {
        Self::DynamicRw(s.0.into(), s.1)
    }
}

impl<T> From<(Signal<T>, WriteSignal<T>)> for MaybeRwSignal<T>
where
    T: Send + Sync,
{
    fn from(s: (Signal<T>, WriteSignal<T>)) -> Self {
        Self::DynamicRw(s.0, s.1)
    }
}

impl<T> FromLocal<(Signal<T, LocalStorage>, WriteSignal<T, LocalStorage>)>
    for MaybeRwSignal<T, LocalStorage>
where
    LocalStorage: Storage<T>,
{
    fn from_local(s: (Signal<T, LocalStorage>, WriteSignal<T, LocalStorage>)) -> Self {
        Self::DynamicRw(s.0, s.1)
    }
}

impl<S> From<&str> for MaybeRwSignal<String, S>
where
    S: Storage<String>,
{
    fn from(s: &str) -> Self {
        Self::Static(s.to_string())
    }
}

// TODO : From (Signal<T>, SignalSetter<T>) for slice! results

#[cfg(feature = "reactive_stores")]
impl<T, Inner, Prev> From<reactive_stores::Subfield<Inner, Prev, T>> for MaybeRwSignal<T>
where
    T: Clone + Send + Sync + 'static,
    reactive_stores::Subfield<Inner, Prev, T>: GetUntracked<Value = T> + Track + Copy,
    Inner: reactive_stores::StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(value: reactive_stores::Subfield<Inner, Prev, T>) -> Self {
        use crate::sync_signal;

        let internal_rw_signal = RwSignal::new(value.get_untracked());
        let _ = sync_signal(value, internal_rw_signal);

        Self::from(internal_rw_signal)
    }
}

#[cfg(feature = "reactive_stores")]
impl<T> From<reactive_stores::Field<T>> for MaybeRwSignal<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: reactive_stores::Field<T>) -> Self {
        use crate::sync_signal;

        let internal_rw_signal = RwSignal::new(value.get_untracked());
        let _ = sync_signal(value, internal_rw_signal);

        Self::from(internal_rw_signal)
    }
}

#[cfg(feature = "reactive_stores")]
impl<T> From<reactive_stores::Store<T>> for MaybeRwSignal<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: reactive_stores::Store<T>) -> Self {
        use crate::sync_signal;

        let internal_rw_signal = RwSignal::new(value.get_untracked());
        let _ = sync_signal(value, internal_rw_signal);

        Self::from(internal_rw_signal)
    }
}

impl<T: Clone> MaybeRwSignal<T, LocalStorage> {
    pub fn into_signal(self) -> (Signal<T, LocalStorage>, WriteSignal<T, LocalStorage>) {
        match self {
            Self::DynamicRead(s) => {
                let (r, w) = signal_local(s.get_untracked());

                Effect::<LocalStorage>::new(move |_| {
                    w.update(move |w| {
                        *w = s.get();
                    });
                });

                (r.into(), w.into())
            }
            Self::DynamicRw(r, w) => (r, w),
            Self::Static(v) => {
                let (r, w) = signal_local(v.clone());
                (Signal::from(r), w.into())
            }
        }
    }
}

impl<T: Clone> MaybeRwSignal<T>
where
    T: Send + Sync,
{
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
