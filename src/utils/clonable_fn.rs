use std::fmt::Debug;

pub trait CloneableFnWithReturn<R>: FnOnce() -> R {
    fn clone_box(&self) -> Box<dyn CloneableFnWithReturn<R>>;
}

impl<F, R> CloneableFnWithReturn<R> for F
where
    F: FnOnce() -> R + Clone + 'static,
    R: 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFnWithReturn<R>> {
        Box::new(self.clone())
    }
}

impl<R> Clone for Box<dyn CloneableFnWithReturn<R>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl<R: Default + 'static> Default for Box<dyn CloneableFnWithReturn<R>> {
    fn default() -> Self {
        Box::new(|| Default::default())
    }
}

pub trait CloneableFnWithArgAndReturn<Arg, R>: FnOnce(Arg) -> R {
    fn clone_box(&self) -> Box<dyn CloneableFnWithArgAndReturn<Arg, R>>;
}

impl<F, R, Arg> CloneableFnWithArgAndReturn<Arg, R> for F
where
    F: FnMut(Arg) -> R + Clone + 'static,
    R: 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFnWithArgAndReturn<Arg, R>> {
        Box::new(self.clone())
    }
}

impl<R, Arg> Clone for Box<dyn CloneableFnWithArgAndReturn<Arg, R>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl<R: Default + 'static, Arg> Default for Box<dyn CloneableFnWithArgAndReturn<Arg, R>> {
    fn default() -> Self {
        Box::new(|_| Default::default())
    }
}

pub trait CloneableFn: FnOnce() {
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

impl<F> CloneableFn for F
where
    F: FnOnce() + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableFn> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl Default for Box<dyn CloneableFn> {
    fn default() -> Self {
        Box::new(|| {})
    }
}

pub trait CloneableFnWithArg<Arg>: FnOnce(Arg) {
    fn clone_box(&self) -> Box<dyn CloneableFnWithArg<Arg>>;
}

impl<F, Arg> CloneableFnWithArg<Arg> for F
where
    F: FnMut(Arg) + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFnWithArg<Arg>> {
        Box::new(self.clone())
    }
}

impl<Arg> Clone for Box<dyn CloneableFnWithArg<Arg>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl<Arg> Default for Box<dyn CloneableFnWithArg<Arg>> {
    fn default() -> Self {
        Box::new(|_| {})
    }
}

pub trait CloneableFnMutWithArg<Arg>: FnMut(Arg) {
    fn clone_box(&self) -> Box<dyn CloneableFnMutWithArg<Arg>>;
}

impl<F, Arg> CloneableFnMutWithArg<Arg> for F
where
    F: FnMut(Arg) + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFnMutWithArg<Arg>> {
        Box::new(self.clone())
    }
}

impl<Arg> Clone for Box<dyn CloneableFnMutWithArg<Arg>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl<Arg> Default for Box<dyn CloneableFnMutWithArg<Arg>> {
    fn default() -> Self {
        Box::new(|_| {})
    }
}

impl<Arg> Debug for Box<dyn CloneableFnMutWithArg<Arg>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box<dyn CloneableFnMutWithArg<{}>>",
            std::any::type_name::<Arg>()
        )
    }
}

pub trait CloneableFnMut: FnMut() {
    fn clone_box(&self) -> Box<dyn CloneableFnMut>;
}

impl<F> CloneableFnMut for F
where
    F: FnMut() + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFnMut> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableFnMut> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

impl Default for Box<dyn CloneableFnMut> {
    fn default() -> Self {
        Box::new(|| {})
    }
}

impl Debug for Box<dyn CloneableFnMut> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn CloneableFnMut>",)
    }
}
