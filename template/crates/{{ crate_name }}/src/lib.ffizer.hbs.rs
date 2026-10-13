#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]

{{#if (eq unstable_apis "true")}}#[cfg(web_sys_unstable_apis)]
{{/if}}mod {{ function_name }};
{{#if (eq unstable_apis "true")}}#[cfg(web_sys_unstable_apis)]
{{/if}}pub use {{ function_name }}::*;
