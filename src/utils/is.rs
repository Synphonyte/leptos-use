use crate::use_window;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IS_IOS: bool = if let Some(Ok(user_agent)) =
        use_window().navigator().map(|n| n.user_agent())
    {
        user_agent.contains("iPhone") || user_agent.contains("iPad") || user_agent.contains("iPod")
    } else {
        false
    };
}
