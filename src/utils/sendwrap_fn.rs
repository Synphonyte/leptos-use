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
    
    (once move || $($content:tt)*) => {
        sendwrap_fn!(once move | | $($content)*)  
    };
    
    (move || $($content:tt)*) => {
        sendwrap_fn!(move | | $($content)*)  
    };
}

pub(crate) use sendwrap_fn;
