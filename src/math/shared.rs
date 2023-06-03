macro_rules! use_partial_cmp {
    ($(#[$outer:meta])*
    $fn_name:ident,
    $ord:pat
    ) => {
        $(#[$outer])*
        pub fn $fn_name<C, S, N>(cx: Scope, container: S) -> Signal<Option<N>>
        where
            S: Into<MaybeSignal<C>>,
            C: 'static,
            for<'a> &'a C: IntoIterator<Item = &'a N>,
            N: PartialOrd + Clone,
        {
            let container = container.into();

            create_memo(cx, move |_| {
                container.with(|container| {
                    if container.into_iter().count() == 0 {
                        return None;
                    }

                    container
                        .into_iter()
                        .fold(None, |acc, e| match acc {
                            Some(acc) => match N::partial_cmp(acc, e) {
                                Some($ord) => Some(e),
                                _ => Some(acc),
                            },
                            None => match N::partial_cmp(e, e) {
                                None => None,
                                _ => Some(e),
                            },
                        })
                        .cloned()
                })
            })
            .into()
        }
    };
}

pub(crate) use use_partial_cmp;
