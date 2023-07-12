use std::fmt::Debug;

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
