macro_rules! use_partial_cmp {
    ($(#[$outer:meta])*
    $fn_name:ident,
    $ord:pat
    ) => {
        $(#[$outer])*
        pub fn $fn_name<C, S, N>(container: S) -> Signal<Option<N>>
        where
            S: Into<Signal<C>>,
            C: Send + Sync + 'static,
            for<'a> &'a C: IntoIterator<Item = &'a N>,
            N: PartialOrd + Clone + Send + Sync + 'static,
        {
            let container = container.into();

            Signal::derive(move || {
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
        }
    };
}

macro_rules! use_simple_math {
    (
        $(#[$outer:meta])*
        $fn_name:ident
    ) => {
        paste! {
            $(#[$outer])*
            pub fn [<use_ $fn_name>]<S, N>(x: S) -> Signal<N>
            where
                S: Into<Signal<N>> + Send + Sync,
                N: Float + Send + Sync + 'static,
            {
                let x = x.into();
                Signal::derive(move || x.get().$fn_name())
            }
        }
    };
}

macro_rules! use_binary_logic {
    (
        $(#[$outer:meta])*
        $fn_name:ident
        $op:tt
    ) => {
        paste! {
            $(#[$outer])*
            pub fn [<use_ $fn_name>]<S1, S2>(a: S1, b: S2) -> Signal<bool>
            where
                S1: Into<Signal<bool>>,
                S2: Into<Signal<bool>>,
            {
                let a = a.into();
                let b = b.into();
                Signal::derive(move || a.get() $op b.get())
            }
        }
    };
}

pub(crate) use use_binary_logic;
pub(crate) use use_partial_cmp;
pub(crate) use use_simple_math;
