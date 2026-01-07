use leptos::prelude::*;
use std::ops::{Deref, DerefMut};

/// A signal of an optional send-wrapped type `T` that is always `None` on the server but behaves
/// on the client like a `Signal<Option<T>>`.
pub struct OptionLocalRwSignal<T> {
    #[cfg(feature = "ssr")]
    _data: std::marker::PhantomData<fn() -> T>,

    #[cfg(not(feature = "ssr"))]
    inner: RwSignal<Option<T>, LocalStorage>,

    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
}

impl<T> OptionLocalRwSignal<T>
where
    T: 'static,
{
    #[track_caller]
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "ssr")]
            _data: std::marker::PhantomData,

            #[cfg(not(feature = "ssr"))]
            inner: RwSignal::new_local(None),

            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    #[allow(unused_mut)]
    pub fn read_only(mut self) -> OptionLocalSignal<T> {
        #[cfg(not(feature = "ssr"))]
        {
            // Prolong the lifetime of the original RwSignal. This is basically equivalent to what
            // RwSignal::read_only() does under the hood.
            let new_inner = ArcRwSignal::from(self.inner);
            self.inner = RwSignal::from_local(new_inner);
        }

        OptionLocalSignal {
            inner: self,
            #[cfg(debug_assertions)]
            defined_at: std::panic::Location::caller(),
        }
    }
}

impl<T> Clone for OptionLocalRwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for OptionLocalRwSignal<T> {}

impl<T> DefinedAt for OptionLocalRwSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }

        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

// In order to allow `None::<T>` to be used like a Signal on the server, we have a special
// `NoneGuard` that always derefs to `None`.

#[cfg(feature = "ssr")]
pub type OptionLocalGuard<T> = NoneGuard<T>;

#[cfg(not(feature = "ssr"))]
pub type OptionLocalGuard<T> = guards::ReadGuard<Option<T>, guards::Plain<Option<T>>>;

impl<T> ReadUntracked for OptionLocalRwSignal<T>
where
    T: 'static,
{
    type Value = OptionLocalGuard<T>;

    #[inline(always)]
    fn try_read_untracked(&self) -> Option<Self::Value> {
        #[cfg(feature = "ssr")]
        {
            Some(NoneGuard::default())
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.try_read_untracked()
        }
    }
}

impl<T> Track for OptionLocalRwSignal<T>
where
    T: 'static,
{
    #[inline(always)]
    fn track(&self) {
        #[cfg(feature = "ssr")]
        {
            // No-op on server
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.track();
        }
    }
}

impl<T> Notify for OptionLocalRwSignal<T>
where
    T: 'static,
{
    #[inline(always)]
    fn notify(&self) {
        #[cfg(feature = "ssr")]
        {
            leptos::logging::warn!(
                "Attempted to notify an OptionLocalRwSignal on the server; this has no effect."
            );
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.notify();
        }
    }
}

impl<T> Write for OptionLocalRwSignal<T>
where
    T: 'static,
{
    type Value = Option<T>;

    #[inline(always)]
    fn try_write(&self) -> Option<impl UntrackableGuard<Target = Self::Value>> {
        #[cfg(feature = "ssr")]
        {
            leptos::logging::warn!(
                "Attempted to write to an OptionLocalRwSignal on the server; this has no effect."
            );
            None::<NoneGuard<T>>
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.try_write()
        }
    }

    #[inline(always)]
    fn try_write_untracked(&self) -> Option<impl DerefMut<Target = Self::Value>> {
        #[cfg(feature = "ssr")]
        {
            leptos::logging::warn!(
                "Attempted to write to an OptionLocalRwSignal on the server; this has no effect."
            );
            None::<NoneGuard<T>>
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.try_write_untracked()
        }
    }
}

impl<T> IsDisposed for OptionLocalRwSignal<T>
where
    T: 'static,
{
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        #[cfg(feature = "ssr")]
        {
            false
        }
        #[cfg(not(feature = "ssr"))]
        {
            self.inner.is_disposed()
        }
    }
}

/// A guard that only ever returns `None`. While this is implemented as both a read and a write guard,
/// actually using it as a write guard or dereferencing this mutably will panic.
/// The WriteGuard implementation only exists to allow returning `None` from `try_write` and `try_write_untracked`.
pub struct NoneGuard<T> {
    _data: std::marker::PhantomData<T>,
}

impl<T> Default for NoneGuard<T> {
    fn default() -> Self {
        Self {
            _data: std::marker::PhantomData,
        }
    }
}

impl<T> Deref for NoneGuard<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &None
    }
}

/// This implementation only exists so that we can create a None::<impl DerefMut>.
impl<T> DerefMut for NoneGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("Attempted to mutably dereference a NoneGuard");
    }
}

/// This implementation only exists so that we can create a None::<impl UntrackableGuard>.
impl<T> UntrackableGuard for NoneGuard<T> {
    fn untrack(&mut self) {
        panic!("Attempted to untrack a NoneGuard");
    }
}

/// Read-Only signal
pub struct OptionLocalSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,

    inner: OptionLocalRwSignal<T>,
}

impl<T> Clone for OptionLocalSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for OptionLocalSignal<T> {}

impl<T> DefinedAt for OptionLocalSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T> ReadUntracked for OptionLocalSignal<T>
where
    T: 'static,
{
    type Value = OptionLocalGuard<T>;

    #[inline(always)]
    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.inner.try_read_untracked()
    }
}

impl<T> Track for OptionLocalSignal<T>
where
    T: 'static,
{
    #[inline(always)]
    fn track(&self) {
        self.inner.track();
    }
}
