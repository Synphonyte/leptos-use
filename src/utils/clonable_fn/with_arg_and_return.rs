use std::fmt::Debug;

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

impl<R, Arg> Debug for Box<dyn CloneableFnWithArgAndReturn<Arg, R>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box<dyn CloneableFnWithArgAndReturn<{}, {}>>",
            std::any::type_name::<Arg>(),
            std::any::type_name::<R>()
        )
    }
}
