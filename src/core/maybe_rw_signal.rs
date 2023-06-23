use leptos::*;
use std::pin::Pin;

pub enum MaybeRwSignal<T>
where
    T: 'static,
{
    Static(T),
    DynamicRw(ReadSignal<T>, WriteSignal<T>),
    DynamicRead(Signal<T>),
}

impl<T: Clone> Clone for MaybeRwSignal<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::DynamicRw(r, w) => Self::DynamicRw(r, w),
            Self::DynamicRead(s) => Self::DynamicRead(s),
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
        Self::DynamicRw(r, w)
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for MaybeRwSignal<T> {
    fn from(s: (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::DynamicRw(s.0, s.1)
    }
}

impl From<&str> for MaybeRwSignal<String> {
    fn from(s: &str) -> Self {
        Self::Static(s.to_string())
    }
}

impl<T: Clone> SignalGet<T> for MaybeRwSignal<T> {
    fn get(&self) -> T {
        match self {
            Self::Static(t) => t.clone(),
            Self::DynamicRw(r, _) => r.get(),
            Self::DynamicRead(s) => s.get(),
        }
    }

    fn try_get(&self) -> Option<T> {
        match self {
            Self::Static(t) => Some(t.clone()),
            Self::DynamicRw(r, _) => r.try_get(),
            Self::DynamicRead(s) => s.try_get(),
        }
    }
}

impl<T> SignalWith<T> for MaybeRwSignal<T> {
    fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        match self {
            Self::Static(t) => f(t),
            Self::DynamicRw(r, w) => r.with(f),
            Self::DynamicRead(s) => s.with(f),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        match self {
            Self::Static(t) => Some(f(t)),
            Self::DynamicRw(r, _) => r.try_with(f),
            Self::DynamicRead(s) => s.try_with(f),
        }
    }
}

impl<T> SignalWithUntracked<T> for MaybeRwSignal<T> {
    fn with_untracked<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        match self {
            Self::Static(t) => f(t),
            Self::DynamicRw(r, _) => r.with_untracked(f),
            Self::DynamicRead(s) => s.with_untracked(f),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        match self {
            Self::Static(t) => Some(f(t)),
            Self::DynamicRw(r, _) => r.try_with_untracked(f),
            Self::DynamicRead(s) => s.try_with_untracked(f),
        }
    }
}

impl<T: Clone> SignalGetUntracked<T> for MaybeRwSignal<T> {
    fn get_untracked(&self) -> T {
        match self {
            Self::Static(t) => t.clone(),
            Self::DynamicRw(r, _) => r.get_untracked(),
            Self::DynamicRead(s) => s.get_untracked(),
        }
    }

    fn try_get_untracked(&self) -> Option<T> {
        match self {
            Self::Static(t) => Some(t.clone()),
            Self::DynamicRw(r, _) => r.try_get_untracked(),
            Self::DynamicRead(s) => s.try_get_untracked(),
        }
    }
}

impl<T: Clone> SignalStream<T> for MaybeRwSignal<T> {
    fn to_stream(&self, cx: Scope) -> Pin<Box<dyn futures::stream::Stream<Item = T>>> {
        match self {
            Self::Static(t) => {
                let t = t.clone();

                let stream = futures::stream::once(async move { t });

                Box::bin(stream)
            }
            Self::DynamicRw(r, _) => r.to_stream(cx),
            Self::DynamicRead(s) => s.to_stream(cx),
        }
    }
}

impl<T> MaybeRwSignal<T> {
    pub fn derive(cx: Scope, derived_signal: impl Fn() -> T + 'static) -> Self {
        Self::DynamicRead(Signal::derive(cx, derived_signal))
    }
}

impl<T> SignalSetUntracked<T> for MaybeRwSignal<T> {
    fn set_untracked(&self, new_value: T) {
        match self {
            Self::DynamicRw(_, w) => w.set_untracked(new_value),
            _ => {
                // do nothing
            }
        }
    }

    fn try_set_untracked(&self, new_value: T) -> Option<T> {
        match self {
            Self::DynamicRw(_, w) => w.try_set_untracked(new_value),
            _ => Some(new_value),
        }
    }
}

impl<T> SignalUpdateUntracked<T> for MaybeRwSignal<T> {
    #[inline(always)]
    fn update_untracked(&self, f: impl FnOnce(&mut T)) {
        match self {
            Self::DynamicRw(_, w) => w.update_untracked(f),
            _ => {
                // do nothing
            }
        }
    }

    #[inline(always)]
    fn try_update_untracked<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        match self {
            Self::DynamicRw(_, w) => w.try_update_untracked(f),
            _ => Some(f()),
        }
    }
}

impl<T> SignalUpdate<T> for MaybeRwSignal<T> {
    #[inline(always)]
    fn update(&self, f: impl FnOnce(&mut T)) {
        match self {
            Self::DynamicRw(_, w) => w.update(f),
            _ => {
                // do nothing
            }
        }
    }

    #[inline(always)]
    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        match self {
            Self::DynamicRw(_, w) => w.try_update(f),
            _ => Some(f()),
        }
    }
}

impl<T> SignalSet<T> for MaybeRwSignal<T> {
    #[inline(always)]
    fn set(&self, new_value: T) {
        match self {
            Self::DynamicRw(_, w) => w.set(new_value),
            _ => {
                // do nothing
            }
        }
    }

    fn try_set(&self, new_value: T) -> Option<T> {
        match self {
            Self::DynamicRw(_, w) => w.try_set(new_value),
            _ => Some(new_value),
        }
    }
}

impl<T> SignalDispose for MaybeRwSignal<T> {
    fn dispose(self) {
        match self {
            Self::DynamicRw(r, w) => {
                r.dispose();
                w.dispose();
            }
            Self::DynamicRead(s) => s.dispose(),
            _ => {
                // do nothing
            }
        }
    }
}
