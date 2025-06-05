use std::{marker::PhantomData, ops::Deref};

use leptos::prelude::*;

pub struct UseRwSignal<R, W, T>(R, W, PhantomData<T>)
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static;

impl<RW, T> From<RW> for UseRwSignal<RW, RW, T>
where
    RW: Read + Copy,
    <RW as Read>::Value: Deref<Target = T>,
    RW: Write<Value = T> + Copy,
    T: 'static,
{
    fn from(s: RW) -> Self {
        Self(s, s, PhantomData)
    }
}

impl<T, R, W> From<(R, W)> for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    fn from((r, w): (R, W)) -> Self {
        Self(r, w, PhantomData)
    }
}

impl<T> Default for UseRwSignal<RwSignal<T>, RwSignal<T>, T>
where
    T: Default + Send + Sync,
{
    fn default() -> Self {
        let signal = Default::default();
        Self(signal, signal, PhantomData)
    }
}

impl<T> Default for UseRwSignal<RwSignal<T, LocalStorage>, RwSignal<T, LocalStorage>, T>
where
    T: Default,
{
    fn default() -> Self {
        let signal = Default::default();
        Self(signal, signal, PhantomData)
    }
}

impl<R, W, T> Clone for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<R, W, T> Copy for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
}

impl<R, W, T> DefinedAt for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        self.0.defined_at()
    }
}

impl<R, W, T> ReadUntracked for UseRwSignal<R, W, T>
where
    R: Read + Copy + ReadUntracked,
    <R as ReadUntracked>::Value: Deref<Target = T>,
    <R as Read>::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    type Value = <R as ReadUntracked>::Value;

    #[inline(always)]
    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.0.try_read_untracked()
    }
}

impl<R, W, T> Notify for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    #[inline(always)]
    fn notify(&self) {
        self.1.notify()
    }
}

impl<R, W, T> Write for UseRwSignal<R, W, T>
where
    R: Read + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    type Value = T;

    #[inline(always)]
    fn try_write(&self) -> Option<impl UntrackableGuard<Target = Self::Value>> {
        self.1.try_write()
    }

    #[inline(always)]
    fn try_write_untracked(&self) -> Option<impl std::ops::DerefMut<Target = Self::Value>> {
        self.1.try_write_untracked()
    }
}

impl<R, W, T> Track for UseRwSignal<R, W, T>
where
    R: Read + Track + Copy,
    R::Value: Deref<Target = T>,
    W: Write<Value = T> + Copy,
    T: 'static,
{
    fn track(&self) {
        self.0.track();
    }
}
