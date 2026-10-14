use crate::MaybeRwSignal;
use default_struct_builder::DefaultBuilder;

/// Options for [`use_favicon_with_options`](crate::use_favicon_with_options).
#[derive(DefaultBuilder)]
pub struct UseFaviconOptions {
    /// New input favicon. Can be a `RwSignal` in which case updates will change the favicon. Defaults to None.
    #[builder(into)]
    pub(crate) new_icon: MaybeRwSignal<Option<String>>,

    /// Base URL of the favicon. Defaults to "".
    #[builder(into)]
    pub(crate) base_url: String,
    /// Rel attribute of the <link> tag. Defaults to "icon".
    #[builder(into)]
    pub(crate) rel: String,
}

impl Default for UseFaviconOptions {
    fn default() -> Self {
        Self {
            new_icon: Default::default(),
            base_url: "".to_string(),
            rel: "icon".to_string(),
        }
    }
}
