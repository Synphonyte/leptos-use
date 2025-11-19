use std::ops::Deref;

use leptos::prelude::{guards::ReadGuard, *};
use send_wrapper::SendWrapper;

/// A signal of an optional send-wrapped type `T` that is always `None` on the server but behaves
/// on the client like a `Signal<Option<T>>`.
pub struct OptionLocalSignal<T>(Signal<Option<SendWrapper<T>>>)
where
    T: 'static;

impl<T> OptionLocalSignal<T>
where
    T: 'static,
{
    pub fn new(signal: Signal<Option<SendWrapper<T>>>) -> Self {
        Self(signal)
    }
}

impl<T> Clone for OptionLocalSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for OptionLocalSignal<T> {}

impl<T> DefinedAt for OptionLocalSignal<T> {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        self.0.defined_at()
    }
}

impl<T: Clone> ReadUntracked for OptionLocalSignal<T> {
    type Value = ReadGuard<Option<T>, ClientLocalReadGuard<T>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.0
            .try_read_untracked()
            .map(|value| ReadGuard::new(ClientLocalReadGuard(value.clone().map(|v| v.take()))))
    }
}

/*impl<T: Clone> Read for OptionLocalSignal<T> {
    
}*/

impl<T> Track for OptionLocalSignal<T>
where
    T: Clone + 'static,
{
    fn track(&self) {
        self.0.track();
    }
}

impl<T, S> From<S> for OptionLocalSignal<T>
where
    S: Into<Signal<Option<SendWrapper<T>>>>,
{
    fn from(signal: S) -> Self {
        Self::new(signal.into())
    }
}

pub struct ClientLocalReadGuard<T>(Option<T>);

impl<T> Deref for ClientLocalReadGuard<T>
where
    T: Clone + 'static,
{
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
