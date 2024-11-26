/// Macro to wrap closures with `send_wrapper::SendWrapper`.
///
/// ## Usage
///
/// ```
/// # use leptos_use::sendwrap_fn;
///
/// let wrapped = sendwrap_fn!(move |a: i32, b: i32| { /* do stuff */ });
/// ```
///
/// For closures that implement only `FnOnce`:
///
/// ```
/// # use leptos_use::sendwrap_fn;
///
/// let wrapped = sendwrap_fn!(once move || { /* do stuff */ });
/// ```
#[macro_export]
macro_rules! sendwrap_fn {
    (move |$($param:ident : $ty:ty),*| $($content:tt)*) => {
        {
            let wrapped = send_wrapper::SendWrapper::new(move |$($param : $ty),*| $($content)*);

            move |$($param : $ty),*| wrapped($($param),*)
        }
    };

    (once move |$($param:ident : $ty:ty),*| $($content:tt)*) => {
        {
            let wrapped = send_wrapper::SendWrapper::new(move |$($param : $ty),*| $($content)*);

            move |$($param : $ty),*| {
                let inner = wrapped.take();
                inner($($param),*)
            }
        }
    };

    (move |$($param:ident),*| $($content:tt)*) => {
        {
            let wrapped = send_wrapper::SendWrapper::new(move |$($param),*| $($content)*);

            move |$($param),*| wrapped($($param),*)
        }
    };

    (once move |$($param:ident),*| $($content:tt)*) => {
        {
            let wrapped = send_wrapper::SendWrapper::new(move |$($param),*| $($content)*);

            move |$($param),*| {
                let inner = wrapped.take();
                inner($($param),*)
            }
        }
    };

    (once move || $($content:tt)*) => {
        sendwrap_fn!(once move | | $($content)*)
    };

    (move || $($content:tt)*) => {
        sendwrap_fn!(move | | $($content)*)
    };
}
