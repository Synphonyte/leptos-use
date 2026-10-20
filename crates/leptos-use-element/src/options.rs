use default_struct_builder::DefaultBuilder;
use wasm_bindgen::prelude::*;

/// Options for [`use_mutation_observer_with_options`](crate::use_mutation_observer_with_options).
#[derive(DefaultBuilder, Clone, Default)]
pub struct UseMutationObserverOptions {
    /// Set to `true` to extend monitoring to the entire subtree of nodes rooted at `target`.
    /// All of the other properties are then extended to all of the nodes in the subtree
    /// instead of applying solely to the `target` node. The default value is `false`.
    subtree: bool,

    /// Set to `true` to monitor the target node (and, if `subtree` is `true`, its descendants)
    /// for the addition of new child nodes or removal of existing child nodes.
    /// The default value is `false`.
    child_list: bool,

    /// Set to `true` to watch for changes to the value of attributes on the node or nodes being
    /// monitored. The default value is `true` if either of `attribute_filter` or
    /// `attribute_old_value` is specified, otherwise the default value is `false`.
    attributes: bool,

    /// An array of specific attribute names to be monitored. If this property isn't included,
    /// changes to all attributes cause mutation notifications.
    #[builder(into)]
    attribute_filter: Option<Vec<String>>,

    /// Set to `true` to record the previous value of any attribute that changes when monitoring
    /// the node or nodes for attribute changes; See
    /// [Monitoring attribute values](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe#monitoring_attribute_values)
    /// for an example of watching for attribute changes and recording values.
    /// The default value is `false`.
    attribute_old_value: bool,

    /// Set to `true` to monitor the specified target node
    /// (and, if `subtree` is `true`, its descendants)
    /// for changes to the character data contained within the node or nodes.
    /// The default value is `true` if `character_data_old_value` is specified,
    /// otherwise the default value is `false`.
    #[builder(into)]
    character_data: Option<bool>,

    /// Set to `true` to record the previous value of a node's text whenever the text changes on
    /// nodes being monitored. The default value is `false`.
    character_data_old_value: bool,
}

impl From<UseMutationObserverOptions> for web_sys::MutationObserverInit {
    fn from(val: UseMutationObserverOptions) -> Self {
        let UseMutationObserverOptions {
            subtree,
            child_list,
            attributes,
            attribute_filter,
            attribute_old_value,
            character_data,
            character_data_old_value,
        } = val;

        let init = Self::new();

        init.set_subtree(subtree);
        init.set_child_list(child_list);
        // `attributes` is always written, so it can never be absent and cannot
        // pick up the browser's implicit default. The DOM rejects `observe()`
        // when `attribute_filter` or `attribute_old_value` is present while
        // `attributes` is false, so apply the documented default here.
        init.set_attributes(attributes || attribute_filter.is_some() || attribute_old_value);
        init.set_attribute_old_value(attribute_old_value);
        init.set_character_data_old_value(character_data_old_value);

        if let Some(attribute_filter) = attribute_filter {
            let array = js_sys::Array::from_iter(attribute_filter.into_iter().map(JsValue::from));
            init.set_attribute_filter(array.unchecked_ref());
        }
        if let Some(character_data) = character_data {
            init.set_character_data(character_data);
        }

        init
    }
}
