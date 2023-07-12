use std::fmt::Debug;

pub trait CloneableFnWithArg<Arg>: FnOnce(Arg) {
    fn clone_box(&self) -> Box<dyn CloneableFnWithArg<Arg>>;
}

impl<F, Arg> CloneableFnWithArg<Arg> for F
where
    F: FnOnce(Arg) + Clone + 'static,
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

impl<Arg> Debug for Box<dyn CloneableFnWithArg<Arg>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box<dyn CloneableFnWithArg<{}>>",
            std::any::type_name::<Arg>()
        )
    }
}
