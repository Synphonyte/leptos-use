#![allow(unused_macros, unused_imports)]

macro_rules! signal_filtered {
    (
        $(#[$outer:meta])*
        $filter_name:ident
        #[$simple_func_doc:meta]
        #[$options_doc:meta]

    ) => {
        paste! {
            $(#[$outer])*
            #[track_caller]
            pub fn [<signal_ $filter_name d>]<S, T>(
                value: S,
                ms: impl Into<Signal<f64>> + 'static,
            ) -> Signal<T>
            where
                S: Into<Signal<T>>,
                T: Clone + Send + Sync + 'static,
            {
                [<signal_ $filter_name d_with_options>](value, ms, [<$filter_name:camel Options>]::default())
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
            pub fn [<signal_ $filter_name d_with_options>]<S, T>(
                value: S,
                ms: impl Into<Signal<f64>> + 'static,
                options: [<$filter_name:camel Options>],
            ) -> Signal<T>
            where
                S: Into<Signal<T>>,
                T: Clone + Send + Sync + 'static,
            {
                let value = value.into();
                let ms = ms.into();

                if ms.get_untracked() <= 0.0 {
                    return value;
                }

                let (filtered, set_filtered) = signal(value.get_untracked());

                let update = [<use_ $filter_name _fn_with_options>](
                    move || set_filtered.set(value.get_untracked()),
                    ms,
                    options,
                );

                Effect::watch(move || value.get(), move |_, _, _| update(), false);

                filtered.into()
            }
        }
    };
}

macro_rules! signal_filtered_local {
    (
        $(#[$outer:meta])*
        $filter_name:ident
        #[$simple_func_doc:meta]
        #[$options_doc:meta]

    ) => {
        paste! {
            $(#[$outer])*
            #[track_caller]
            pub fn [<signal_ $filter_name d_local>]<S, T>(
                value: S,
                ms: impl Into<Signal<f64>> + 'static,
            ) -> Signal<T, LocalStorage>
            where
                S: Into<Signal<T, LocalStorage>>,
                T: Clone + 'static,
            {
                [<signal_ $filter_name d_local_with_options>](value, ms, [<$filter_name:camel Options>]::default())
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
            pub fn [<signal_ $filter_name d_local_with_options>]<S, T>(
                value: S,
                ms: impl Into<Signal<f64>> + 'static,
                options: [<$filter_name:camel Options>],
            ) -> Signal<T, LocalStorage>
            where
                S: Into<Signal<T, LocalStorage>>,
                T: Clone + 'static,
            {
                let value = value.into();
                let ms = ms.into();

                if ms.get_untracked() <= 0.0 {
                    return value;
                }

                let (filtered, set_filtered) = signal_local(value.get_untracked());

                let update = [<use_ $filter_name _fn_with_options>](
                    move || set_filtered.set(value.get_untracked()),
                    ms,
                    options,
                );

                Effect::watch(move || value.get(), move |_, _, _| update(), false);

                filtered.into()
            }
        }
    };
}

pub(crate) use signal_filtered;
pub(crate) use signal_filtered_local;
