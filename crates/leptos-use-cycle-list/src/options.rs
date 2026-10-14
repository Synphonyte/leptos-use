use crate::MaybeRwSignal;
use default_struct_builder::DefaultBuilder;

/// Options for [`use_cycle_list_with_options`](crate::use_cycle_list_with_options).
#[derive(DefaultBuilder)]
pub struct UseCycleListOptions<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    /// The initial value of the state. Can be a Signal. If none is provided the first entry
    /// of the list will be used.
    #[builder(keep_type)]
    pub(crate) initial_value: Option<MaybeRwSignal<T>>,

    /// The default index when the current value is not found in the list.
    /// For example when `get_index_of` returns `None`.
    pub(crate) fallback_index: usize,

    /// Custom function to get the index of the current value. Defaults to `Iterator::position()`
    #[builder(keep_type)]
    pub(crate) get_position: fn(&T, &Vec<T>) -> Option<usize>,
}

impl<T> Default for UseCycleListOptions<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            initial_value: None,
            fallback_index: 0,
            get_position: |value: &T, list: &Vec<T>| list.iter().position(|v| v == value),
        }
    }
}
