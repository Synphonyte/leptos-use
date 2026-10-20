#[doc(hidden)]
#[macro_export]
macro_rules! js_value_from_to_string {
    ($name:ident) => {
        impl From<$name> for $crate::__private::wasm_bindgen::JsValue {
            fn from(value: $name) -> Self {
                $crate::__private::wasm_bindgen::JsValue::from(&value.to_string())
            }
        }
    };
}
