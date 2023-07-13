mod mut_;
mod mut_with_arg;
mod with_arg;
mod with_arg_and_return;
mod with_return;

pub use mut_::*;
pub use mut_with_arg::*;
use std::fmt::Debug;
pub use with_arg::*;
pub use with_arg_and_return::*;
pub use with_return::*;

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

impl Debug for Box<dyn CloneableFn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn CloneableFn>",)
    }
}
