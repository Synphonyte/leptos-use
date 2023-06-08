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
