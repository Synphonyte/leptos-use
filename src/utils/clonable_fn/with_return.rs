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

impl<R> Debug for Box<dyn CloneableFnWithReturn<R>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box<dyn CloneableFnWithReturn<{}>>",
            std::any::type_name::<R>()
        )
    }
}
