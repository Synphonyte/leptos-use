---
name: new-hook
description: Scaffold and implement a new leptos-use hook/function as its own micro crate using the ffizer template in `template/` (crate, browser test, example, book page, workspace and changelog entries). Use when asked to add a new `use_*` function to leptos-use.
---

# Add a new hook to leptos-use

Every function lives in its own micro crate `crates/leptos-use-<name>` (library `leptos_use_<name>`).
The ffizer template in `template/` scaffolds everything; `template/modify_files.py` registers it.
Don't add anything to the deprecated `leptos-use` facade crate (`src/`).

## 1. Decide the inputs

| Variable        | Example                  | Notes                                                                                 |
|-----------------|--------------------------|---------------------------------------------------------------------------------------|
| `function_name` | `use_battery`            | snake_case, usually starting with `use_`                                              |
| `category`      | `sensors`                | lower-case heading of `docs/book/src/SUMMARY.md` (storage, elements, browser, sensors, network, animation, component, watch, reactivity, iterable, utilities, intl, math) |
| `unstable_apis` | `"false"`                | `"true"` if the hook needs `--cfg=web_sys_unstable_apis` (gates the whole crate)      |
| `crate_name`    | `leptos-use-battery`     | optional; defaults to `leptos-use-` + kebab-case name without `use_`                  |

Check that `crates/<crate_name>` and `examples/<function_name>` don't exist yet.

## 2. Scaffold

Requires the [`ffizer`](https://ffizer.github.io/) CLI and Python 3. Run from the repo root:

```bash
ffizer apply -y -s template -d . \
  -v function_name=use_battery \
  -v category=sensors \
  -v 'unstable_apis="false"'
```

- The quotes around `"false"`/`"true"` are required: an unquoted `true` is parsed as a boolean and the
  template's `eq unstable_apis "true"` check silently falls back to the stable variant.
- Add `-v crate_name=...` only to override the default crate name.
- With `-y` ffizer prints the post-script but does **not** run it. Run it yourself:

```bash
python3 template/modify_files.py use_battery sensors leptos-use-battery
```

It adds the crate to the workspace `members` and `[workspace.dependencies]`, copies the licenses, adds a
"New Functions" entry to `CHANGELOG.md`, links the book page in `SUMMARY.md` (if the category is missing
it prints a warning: add the `# Category` heading and link manually) and registers the example in
`examples/Cargo.toml`.

Created files:

- `crates/<crate_name>/`: `Cargo.toml`, `README.md`, `src/lib.rs`, `src/<function_name>.rs`,
  `src/options.rs` (`<Name>Options`), `src/return_type.rs` (`<Name>Return`), `tests/<function_name>.rs`
- `examples/<function_name>/` (Trunk demo app, `src/main.rs`)
- `docs/book/src/<category>/<function_name>.md` (pulls the function's doc comment and the doc comments of
  its options struct fields from the source automatically)

## 3. Implement the hook

Follow the conventions of the existing crates (look at a similar hook first):

- **Layout**: the function(s) in `src/<function_name>.rs`, `...Options` (with `#[derive(DefaultBuilder)]`,
  fields `pub(crate)`, a manual `Default` impl and a doc comment with the default on every field, which
  the book renders as the "Options" section) in `options.rs`, `...Return` in
  `return_type.rs`. Provide `use_x()` and `use_x_with_options(options)`. Drop options/return files if the
  hook doesn't need them (and their `mod`/`pub use` lines in `lib.rs`).
- **Depending on other micro crates**: add `leptos-use-foo = { workspace = true }` to `[dependencies]`,
  forward SSR with `ssr = ["leptos-use-foo/ssr", ...]`, and re-export it in `lib.rs` with
  `pub(crate) use leptos_use_foo::*;` so sources can use `crate::use_foo`. Shared building blocks:
  `leptos-use-core` (plain types), `leptos-use-element` (`IntoElementMaybeSignal` for element params),
  `leptos-use-maybe-rw-signal`, `leptos-use-event-listener`, `leptos-use-filters`, `leptos-use-supported`,
  `wasm-js-macros` (`js!`), `sendwrap-fn` (`sendwrap_fn!`).
- **web-sys**: list every needed feature in the crate's own `web-sys` dependency.
- **SSR**: the hook must compile and not panic with the `ssr` feature. Guard browser-only code with
  `#[cfg(not(feature = "ssr"))]` / `cfg_if!` and return sensible static values on the server.
  Never call `window()`/`document()` on the server.
- **`use_supported` pitfall**: its signal is only set in an effect, so `is_supported.get_untracked()` is
  always `false` while the hook is being set up. Check support directly (e.g.
  `js!("Foo" in &window())`) when deciding whether to register listeners.
- **Docs**: keep the template's doc sections (Demo link, Usage doctest importing from the micro crate,
  Server-Side Rendering). Doctests must compile: `use leptos_use_battery::...`.
- Unstable web APIs: everything in `lib.rs` is gated by `#[cfg(web_sys_unstable_apis)]` already when
  scaffolded with `unstable_apis="true"`.

## 4. Write the browser tests

`tests/<function_name>.rs` runs in headless Chrome with `wasm-bindgen-test`. Replace the placeholder with
real tests that drive the hook and assert on what it returns. See the other crates' `tests/` for patterns:

- Start async tests with `let _ = Executor::init_wasm_bindgen(); let owner = Owner::new(); owner.set();`
  (needed for effects), then `Executor::tick().await` to let effects run, or
  `gloo_timers::future::TimeoutFuture::new(ms).await` for timers/observers.
- Create DOM with `document().create_element(..)` and append it to the body; trigger behaviour with
  `dispatch_event` (use `*EventInit` dicts for coordinates, `bubbles`, …).
- Browser APIs that need permissions or user gestures (clipboard, geolocation, notifications, media) are
  replaced by small stubs via `js_sys::eval("Object.defineProperty(navigator, ...)")`.
- Files in the crate directory are served by the test runner (e.g. a service worker script).
- Add extra web-sys features the test needs to the crate's
  `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` (`web-sys = { workspace = true, features = [...] }`).

## 5. Write the example

Fill `examples/<function_name>/src/main.rs` with a small demo using `leptos_use_docs` helpers
(`BooleanDisplay`, `Note`). Other micro crates it uses go into `examples/<function_name>/Cargo.toml`
as `leptos-use-foo = { workspace = true }`; add them to `[workspace.dependencies]` of `examples/Cargo.toml`
(`leptos-use-foo = { path = "../crates/leptos-use-foo" }`) if they aren't listed there yet.

## 6. Verify

```bash
cargo fmt --all
cargo clippy -p <crate_name> --tests -- -D warnings
cargo clippy -p <crate_name> --features ssr --tests -- -D warnings
RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo test -p <crate_name> --doc
RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo clippy --target wasm32-unknown-unknown -p <crate_name> --tests -- -D warnings
RUSTFLAGS="--cfg=web_sys_unstable_apis" CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  CHROMEDRIVER=/path/to/chromedriver \
  cargo test --target wasm32-unknown-unknown -p <crate_name> --tests
(cd examples && cargo build -p <function_name> --target wasm32-unknown-unknown)
```

The browser tests need `wasm-bindgen-test-runner` matching the `wasm-bindgen` version in `Cargo.lock`
(`cargo install wasm-bindgen-cli --version <version>`) and a chromedriver matching the installed Chrome
(`wasm-pack` caches one in `~/Library/Caches/.wasm-pack/chromedriver-*` on macOS). The "Browser Tests"
job in `.github/workflows/tests.yml` runs the same tests in CI.

Finally review the `CHANGELOG.md` entry and the generated book page.
