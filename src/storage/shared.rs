use crate::filter_builder_methods;
use crate::storage::{StorageType, UseStorageError, UseStorageOptions};
use crate::utils::{CloneableFnWithArg, DebounceOptions, FilterOptions, ThrottleOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;

macro_rules! use_specific_storage {
    ($(#[$outer:meta])*
    $storage_name:ident
    #[$simple_func:meta]
    ) => {
        paste! {
            $(#[$outer])*
            pub fn [<use_ $storage_name _storage>]<T, D>(
                cx: Scope,
                key: &str,
                defaults: D,
            ) -> (ReadSignal<T>, WriteSignal<T>, impl Fn() + Clone)
            where
                for<'de> T: Serialize + Deserialize<'de> + Clone + 'static,
                D: Into<MaybeSignal<T>>,
                T: Clone,
            {
                [<use_ $storage_name _storage_with_options>](cx, key, defaults, UseSpecificStorageOptions::default())
            }

            /// Version of
            #[$simple_func]
            /// that accepts [`UseSpecificStorageOptions`]. See
            #[$simple_func]
            /// for how to use.
            pub fn [<use_ $storage_name _storage_with_options>]<T, D>(
                cx: Scope,
                key: &str,
                defaults: D,
                options: UseSpecificStorageOptions<T>,
            ) -> (ReadSignal<T>, WriteSignal<T>, impl Fn() + Clone)
            where
                for<'de> T: Serialize + Deserialize<'de> + Clone + 'static,
                D: Into<MaybeSignal<T>>,
                T: Clone,
            {
                use_storage_with_options(cx, key, defaults, options.into_storage_options(StorageType::[<$storage_name:camel>]))
            }
        }
    };
}

pub(crate) use use_specific_storage;

/// Options for [`use_local_storage_with_options`].
#[doc(cfg(feature = "storage"))]
#[derive(DefaultBuilder)]
pub struct UseSpecificStorageOptions<T> {
    /// Listen to changes to this storage key from somewhere else. Defaults to true.
    listen_to_storage_changes: bool,
    /// If no value for the give key is found in the storage, write it. Defaults to true.
    write_defaults: bool,
    /// Takes the serialized (json) stored value and the default value and returns a merged version.
    /// Defaults to simply returning the stored value.
    merge_defaults: fn(&str, &T) -> String,
    /// Optional callback whenever an error occurs. The callback takes an argument of type [`UseStorageError`].
    on_error: Box<dyn CloneableFnWithArg<UseStorageError>>,

    /// Debounce or throttle the writing to storage whenever the value changes.
    filter: FilterOptions,
}

impl<T> Default for UseSpecificStorageOptions<T> {
    fn default() -> Self {
        Self {
            listen_to_storage_changes: true,
            write_defaults: true,
            merge_defaults: |stored_value, _default_value| stored_value.to_string(),
            on_error: Box::new(|_| ()),
            filter: Default::default(),
        }
    }
}

impl<T> UseSpecificStorageOptions<T> {
    pub fn into_storage_options(self, storage_type: StorageType) -> UseStorageOptions<T> {
        UseStorageOptions {
            storage_type,
            listen_to_storage_changes: self.listen_to_storage_changes,
            write_defaults: self.write_defaults,
            merge_defaults: self.merge_defaults,
            on_error: self.on_error,
            filter: self.filter,
        }
    }

    filter_builder_methods!(
        /// the serializing and storing into storage
        filter
    );
}
