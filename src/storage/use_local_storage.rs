use crate::storage::shared::{use_specific_storage, UseSpecificStorageOptions};
use crate::storage::{use_storage_with_options, StorageType};
use leptos::*;
use paste::paste;
use serde::{Deserialize, Serialize};

use_specific_storage!(
    /// Reactive [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    ///
    /// ## Usage
    ///
    /// Please refer to [`use_storage`]
    ///
    /// ## See also
    ///
    /// * [`use_storage`]
    /// * [`use_session_storage`]
    local
    /// [`use_local_storage`]
);
