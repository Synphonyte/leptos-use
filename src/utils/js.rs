#[macro_export]
macro_rules! js {
    ($attr:literal in $($obj:tt)*) => {
        wasm_bindgen::JsValue::from($attr).js_in($($obj)*)
    };
    ($obj:ident[$attr:literal] = $($val:tt)*) => {
        let _ = js_sys::Reflect::set(&$obj, &$attr.into(), &($($val)*).into());
    };
    ($obj:ident[$attr:literal]) => {
        js_sys::Reflect::get(&$obj, &$attr.into())
    };
}

#[macro_export]
macro_rules! js_fut {
    ($($obj:tt)*) => {
        wasm_bindgen_futures::JsFuture::from($($obj)*)
    };

}
