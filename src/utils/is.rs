use lazy_static::lazy_static;
use leptos::*;

lazy_static! {
    pub static ref IS_IOS: bool = if let Ok(user_agent) = window().navigator().user_agent() {
        user_agent.contains("iPhone") || user_agent.contains("iPad") || user_agent.contains("iPod")
    } else {
        false
    };
}
