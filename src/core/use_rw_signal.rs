use leptos::prelude::*;

pub enum UseRwSignal<T: 'static> {
    Separate(Signal<T>, WriteSignal<T>),
    Combined(RwSignal<T>),
}

impl<T> From<RwSignal<T>> for UseRwSignal<T> {
    fn from(s: RwSignal<T>) -> Self {
        Self::Combined(s)
    }
}

impl<T, RS> From<(RS, WriteSignal<T>)> for UseRwSignal<T>
where
    RS: Into<Signal<T>>,
{
    fn from(s: (RS, WriteSignal<T>)) -> Self {
        Self::Separate(s.0.into(), s.1)
    }
}

impl<T> Default for UseRwSignal<T>
where
    T: Default + Send + Sync,
{
    fn default() -> Self {
        Self::Combined(Default::default())
    }
}

impl<T> Clone for UseRwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UseRwSignal<T> {}

impl<T> DefinedAt for UseRwSignal<T> {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        match self {
            Self::Combined(s) => s.defined_at(),
            // NOTE: is this sufficient communication?
            Self::Separate(_, s) => s.defined_at(),
        }
    }
}

impl<T> With for UseRwSignal<T>
where
    T: Send + Sync,
{
    type Value = T;

    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        match self {
            Self::Separate(s, _) => s.with(f),
            Self::Combined(s) => s.with(f),
        }
    }

    fn try_with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        match self {
            Self::Separate(s, _) => s.try_with(f),
            Self::Combined(s) => s.try_with(f),
        }
    }
}

impl<T> WithUntracked for UseRwSignal<T>
where
    T: Send + Sync,
{
    type Value = T;

    fn with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        match self {
            Self::Separate(s, _) => s.with_untracked(f),
            Self::Combined(s) => s.with_untracked(f),
        }
    }

    fn try_with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        match self {
            Self::Separate(s, _) => s.try_with_untracked(f),
            Self::Combined(s) => s.try_with_untracked(f),
        }
    }
}

impl<T> Set for UseRwSignal<T> {
    type Value = T;

    fn set(&self, new_value: T) {
        match *self {
            Self::Separate(_, s) => s.set(new_value),
            Self::Combined(s) => s.set(new_value),
        }
    }

    fn try_set(&self, new_value: T) -> Option<T> {
        match *self {
            Self::Separate(_, s) => s.try_set(new_value),
            Self::Combined(s) => s.try_set(new_value),
        }
    }
}

impl<T> Update for UseRwSignal<T> {
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut T)) {
        match self {
            Self::Separate(_, s) => s.update(f),
            Self::Combined(s) => s.update(f),
        }
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        match self {
            Self::Separate(_, s) => s.try_update(f),
            Self::Combined(s) => s.try_update(f),
        }
    }

    fn maybe_update(&self, fun: impl FnOnce(&mut Self::Value) -> bool) {
        match self {
            Self::Separate(_, s) => s.maybe_update(fun),
            Self::Combined(s) => s.maybe_update(fun),
        }
    }

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        match self {
            Self::Separate(_, s) => s.try_maybe_update(fun),
            Self::Combined(s) => s.try_maybe_update(fun),
        }
    }
}
