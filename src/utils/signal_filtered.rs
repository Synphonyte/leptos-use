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
                ms: impl Into<MaybeSignal<f64>> + 'static,
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
                ms: impl Into<MaybeSignal<f64>> + 'static,
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

                let _ = watch(move || value.get(), move |_, _, _| update(), false);

                filtered.into()
            }
        }
    };
}

pub(crate) use signal_filtered;
