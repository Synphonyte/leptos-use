use leptos::prelude::*;

pub enum UseRwSignal<T: 'static, S = SyncStorage>
where
    S: Storage<T>,
{
    Separate(Signal<T, S>, WriteSignal<T, S>),
    Combined(RwSignal<T, S>),
}

impl<T, S> From<RwSignal<T, S>> for UseRwSignal<T, S>
where
    S: Storage<T>,
{
    fn from(s: RwSignal<T, S>) -> Self {
        Self::Combined(s)
    }
}

impl<T, S, RS> From<(RS, WriteSignal<T, S>)> for UseRwSignal<T, S>
where
    RS: Into<Signal<T, S>>,
    S: Storage<T>,
{
    fn from(s: (RS, WriteSignal<T, S>)) -> Self {
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

impl<T> Default for UseRwSignal<T, LocalStorage>
where
    T: Default,
{
    fn default() -> Self {
        Self::Combined(Default::default())
    }
}

impl<T, S> Clone for UseRwSignal<T, S>
where
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for UseRwSignal<T, S> where S: Storage<T> {}

impl<T, S> DefinedAt for UseRwSignal<T, S>
where
    S: Storage<T>,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        match self {
            Self::Combined(s) => s.defined_at(),
            // NOTE: is this sufficient communication?
            Self::Separate(_, s) => s.defined_at(),
        }
    }
}

impl<T, S> With for UseRwSignal<T, S>
where
    RwSignal<T, S>: With<Value = T>,
    Signal<T, S>: With<Value = T>,
    ReadSignal<T, S>: With<Value = T>,
    S: Storage<T>,
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

impl<T, S> WithUntracked for UseRwSignal<T, S>
where
    RwSignal<T, S>: WithUntracked<Value = T>,
    Signal<T, S>: WithUntracked<Value = T>,
    ReadSignal<T, S>: WithUntracked<Value = T>,
    S: Storage<T>,
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

impl<T, S> Set for UseRwSignal<T, S>
where
    RwSignal<T, S>: Set<Value = T>,
    WriteSignal<T, S>: Set<Value = T>,
    S: Storage<T>,
{
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

impl<T, S> Update for UseRwSignal<T, S>
where
    RwSignal<T, S>: Update<Value = T>,
    WriteSignal<T, S>: Update<Value = T>,
    S: Storage<T>,
{
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
