use std::env;

fn main() {
    let ssr = env::var("CARGO_FEATURE_SSR").is_ok();
    let wasm_ssr = env::var("CARGO_FEATURE_WASM_SSR").is_ok();
    let wasm32 = env::var("CARGO_CFG_TARGET_ARCH").expect("should be present in the build script")
        == "wasm32";
    if ssr && wasm32 && !wasm_ssr {
        println!(
            "cargo::warning=You have enabled the `ssr` feature for a wasm32 target. \
This is probably not what you want. Please check https://leptos-use.rs/server_side_rendering.html \
for how to use the `ssr` feature correctly.\n \
If you're building for wasm32 on the server you can enable the `wasm_ssr` feature to get rid of \
this warning."
        );
    }
}
