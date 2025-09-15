use crate::utils::get_header;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::sync::Arc;

/// Reactive [reduced motions preference](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_prefers_reduced_motion)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_prefers_reduced_motion;
/// # #[cfg(feature = "docs")]
/// # use leptos_use::docs::BooleanDisplay;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let is_reduced_motion_preferred = use_prefers_reduced_motion();
///
/// view! {
///     <div>
///         <p>Prefers reduced motions: <BooleanDisplay value=is_reduced_motion_preferred/></p>
///         <p>
///             Update reduce motion preference
///             <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion#user_preferences">
///                 documentation.
///             </a>
///         </p>
///     </div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this will try to read the
/// [`Sec-CH-Prefers-Reduced-Motion` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Reduced-Motion)
/// to indicate the preference for animations to be displayed with reduced motion.
/// Please have a look at the linked documentation above to see browser support
/// as well as potential serve requirements.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum nor Actix or the default implementation is not to your
/// liking, you can provide your own way of reading the reduced motion header value using the option
/// [`crate::UsePrefersReducedMotionOptions::ssr_motion_header_getter`].
///
/// ## See also
///
/// * [`fn@crate::use_media_query`]
/// * [`fn@crate::use_preferred_contrast`]
/// * [`fn@crate::use_preferred_dark`]
pub fn use_prefers_reduced_motion() -> Signal<bool> {
    use_prefers_reduced_motion_with_options(UsePrefersReducedMotionOptions::default())
}

/// Version of [`fn@crate::use_prefers_reduced_motion`] that takes a `UsePrefersReducedMotionOptions`. See [`fn@crate::use_prefers_reduced_motion`] for how to use.
pub fn use_prefers_reduced_motion_with_options(
    options: UsePrefersReducedMotionOptions,
) -> Signal<bool> {
    #[cfg(not(feature = "ssr"))]
    {
        let _ = options;
        crate::use_media_query("(prefers-reduced-motion: reduce)")
    }
    #[cfg(feature = "ssr")]
    {
        Signal::derive(move || (options.ssr_motion_header_getter)() == Some("reduce".to_string()))
    }
}

/// Options for [`fn@crate::use_prefers_reduced_motion_with_options`].
#[derive(DefaultBuilder)]
pub struct UsePrefersReducedMotionOptions {
    /// Getter function to return the string value of the
    /// [`Sec-CH-Prefers-Reduced-Motion`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Reduced-Motion)
    /// header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default
    /// implementation provided.
    #[allow(dead_code)]
    pub(crate) ssr_motion_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
}

impl Default for UsePrefersReducedMotionOptions {
    fn default() -> Self {
        Self {
            ssr_motion_header_getter: Arc::new(move || {
                get_header!(
                    HeaderName::from_static("sec-ch-prefers-reduced-motion"),
                    use_prefers_reduced_motion,
                    ssr_motion_header_getter
                )
            }),
        }
    }
}
