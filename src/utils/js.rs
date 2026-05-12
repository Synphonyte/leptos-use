//! Holds JavaScript utilities and especially the [`js`] macro.

/// Expand to common JavaScript operations that are too long in Rust.
///
/// Holds three primary rules:
/// - `attribute in object`: Check if the attribute is in the object (see [`JsValue::js_in`]).
/// - `object[attribute]`: Get the value of the object attribute (see [`Reflect::get`]).
/// - `object[attribute] = val`: Assign to the attribute of the object (see [`Reflect::set`]).
///
/// [`JsValue`]: wasm_bindgen::JsValue
#[macro_export]
macro_rules! js {
    ($attr:literal in $($obj:tt)*) => {
        wasm_bindgen::JsValue::from($attr).js_in($($obj)*)
    };
    ($obj:ident[$attr:literal] = $($val:tt)*) => {
        {
            let _ = js_sys::Reflect::set(&$obj, &$attr.into(), &($($val)*).into());
        }
    };
    ($obj:ident[$attr:literal]) => {
        js_sys::Reflect::get(&$obj, &$attr.into())
    };
}

/// Create a [`wasm_bindgen_futures::JsFuture`] from the given tokens.
#[macro_export]
macro_rules! js_fut {
    ($($obj:tt)*) => {
        wasm_bindgen_futures::JsFuture::from($($obj)*)
    };
}
