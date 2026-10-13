#[doc(hidden)]
#[macro_export]
macro_rules! js_value_from_to_string {
    ($name:ident) => {
        impl From<$name> for JsValue {
            fn from(value: $name) -> Self {
                JsValue::from(&value.to_string())
            }
        }
    };
}
