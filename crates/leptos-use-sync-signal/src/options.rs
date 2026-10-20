use default_struct_builder::DefaultBuilder;
use std::rc::Rc;

/// Direction of syncing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SyncDirection {
    LeftToRight,
    RightToLeft,
    #[default]
    Both,
}

pub type AssignFn<T, S> = Rc<dyn Fn(&mut T, &S)>;

/// Transforms or assigns for syncing.
pub enum SyncTransforms<L, R> {
    /// Transform the signal into each other by calling the transform functions.
    /// The values are then simply assigned.
    Transforms {
        /// Transforms the left signal into the right signal.
        ltr: Rc<dyn Fn(&L) -> R>,
        /// Transforms the right signal into the left signal.
        rtl: Rc<dyn Fn(&R) -> L>,
    },

    /// Assign the signals to each other. Instead of using `=` to assign the signals,
    /// these functions are called.
    Assigns {
        /// Assigns the left signal to the right signal.
        ltr: AssignFn<R, L>,
        /// Assigns the right signal to the left signal.
        rtl: AssignFn<L, R>,
    },
}

impl<T> Default for SyncTransforms<T, T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::Assigns {
            ltr: Rc::new(|right, left| *right = left.clone()),
            rtl: Rc::new(|left, right| *left = right.clone()),
        }
    }
}

impl<L, R> SyncTransforms<L, R>
where
    L: 'static,
    R: 'static,
{
    /// Returns assign functions for both directions that respect the value of this enum.
    pub fn assigns(&self) -> (AssignFn<R, L>, AssignFn<L, R>) {
        match self {
            SyncTransforms::Transforms { ltr, rtl } => {
                let ltr = Rc::clone(ltr);
                let rtl = Rc::clone(rtl);
                (
                    Rc::new(move |right, left| *right = ltr(left)),
                    Rc::new(move |left, right| *left = rtl(right)),
                )
            }
            SyncTransforms::Assigns { ltr, rtl } => (Rc::clone(ltr), Rc::clone(rtl)),
        }
    }
}

/// Options for [`sync_signal_with_options`](crate::sync_signal_with_options).
#[derive(DefaultBuilder)]
pub struct SyncSignalOptions<L, R> {
    /// If `true`, the signals will be immediately synced when this function is called.
    /// If `false`, a signal is only updated when the other signal's value changes.
    /// Defaults to `true`.
    pub(crate) immediate: bool,

    /// Direction of syncing. Defaults to `SyncDirection::Both`.
    pub(crate) direction: SyncDirection,

    /// How to transform or assign the values to each other
    /// If `L` and `R` are identical this defaults to the simple `=` operator. If the types are
    /// not the same, then you have to choose to either use [`SyncSignalOptions::with_transforms`]
    /// or [`SyncSignalOptions::with_assigns`].
    #[builder(skip)]
    pub(crate) transforms: SyncTransforms<L, R>,
}

impl<L, R> SyncSignalOptions<L, R> {
    /// Initializes options with transforms functions that convert the signals into each other.
    pub fn with_transforms(
        transform_ltr: impl Fn(&L) -> R + 'static,
        transform_rtl: impl Fn(&R) -> L + 'static,
    ) -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transforms: SyncTransforms::Transforms {
                ltr: Rc::new(transform_ltr),
                rtl: Rc::new(transform_rtl),
            },
        }
    }

    /// Initializes options with assign functions that replace the default `=` operator.
    pub fn with_assigns(
        assign_ltr: impl Fn(&mut R, &L) + 'static,
        assign_rtl: impl Fn(&mut L, &R) + 'static,
    ) -> Self {
        Self {
            immediate: true,
            direction: SyncDirection::Both,
            transforms: SyncTransforms::Assigns {
                ltr: Rc::new(assign_ltr),
                rtl: Rc::new(assign_rtl),
            },
        }
    }
}

impl<T> Default for SyncSignalOptions<T, T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            immediate: true,
            direction: Default::default(),
            transforms: Default::default(),
        }
    }
}
