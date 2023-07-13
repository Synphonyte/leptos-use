use std::fmt::Debug;

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
