#![allow(unused_macros, unused_imports)]

macro_rules! signal_filtered {
    (
        $(#[$outer:meta])*
        $filter_name:ident
        #[$simple_func_doc:meta]
        #[$options_doc:meta]

    ) => { ::paste::paste!{
        $crate::utils::signal_filtered_macro_impl!(
            $(#[$outer])*,
            #[$simple_func_doc],
            #[$options_doc],
            $filter_name,
            [<$filter_name d>],
            ::leptos::prelude::SyncStorage,
            Send;Sync
        );
    }};
}

macro_rules! signal_filtered_local {
    (
        $(#[$outer:meta])*
        $filter_name:ident
        #[$simple_func_doc:meta]
        #[$options_doc:meta]

    ) => { ::paste::paste! {
        $crate::utils::signal_filtered_macro_impl!(
            $(#[$outer])*,
            #[$simple_func_doc],
            #[$options_doc],
            $filter_name,
            [<$filter_name d_local>],
            ::leptos::prelude::LocalStorage,
        );
    }};
}

#[doc(hidden)]
macro_rules! signal_filtered_macro_impl{
    (
        $(#[$outer:meta])*
        ,#[$simple_func_doc:meta]
        ,#[$options_doc:meta]
        ,$filter_name:ident
        ,$fn_name:ident
        ,$storage:ty
        ,$($traits:ty);*
    )  => { ::paste::paste! {
        $(#[$outer])*
        #[track_caller]
        pub fn [<signal_ $fn_name>]<S, T>(
            value: S,
            ms: impl Into<Signal<f64>> + 'static,
        ) -> Signal<T, $storage>
        where
            S: Into<Signal<T, $storage>>,
            T: $($traits + )* Clone + 'static,
        {
            [<signal_ $fn_name _with_options>](value, ms, [<$filter_name:camel Options>]::default())
        }

        /// Version of
        #[$simple_func_doc]
        /// that accepts
        #[$options_doc]
        #[doc="."]
        /// See
        #[$simple_func_doc]
        /// for how to use.
        #[track_caller]
        pub fn [<signal_ $fn_name _with_options>]<S, T>(
            value: S,
            ms: impl Into<Signal<f64>> + 'static,
            options: [<$filter_name:camel Options>],
        ) -> Signal<T, $storage>
        where
            S: Into<Signal<T, $storage>>,
            T: $($traits + )* Clone + 'static,
        {
            let value = value.into();
            let ms = ms.into();

            if ms.get_untracked() <= 0.0 {
                return value;
            }

            let (filtered, set_filtered) = ::leptos::prelude::RwSignal::<T, $storage>::new_with_storage(value.get_untracked()).split();

            let update = [<use_ $filter_name _fn_with_options>](
                move || set_filtered.set(value.get_untracked()),
                ms,
                options,
            );

            Effect::watch(move || value.get(), move |_, _, _| update(), false);

            filtered.into()
        }
    }};
}

pub(crate) use {signal_filtered, signal_filtered_local, signal_filtered_macro_impl};
