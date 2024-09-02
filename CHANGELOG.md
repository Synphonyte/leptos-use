# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.2] - 2024-09-02

### Fix 🍕

- Fixed web-sys `unstable_apis` flag for `use_web_lock`

## [0.13.1] - 2024-09-01 

### New Functions 🚀

- `use_web_lock`
- `use_window_size`

### Change 🔥

- `UseWebsocket::protocols` now supports a signal. It is read right before `open` is called. (thanks to @zakstucke) 

## [0.13.0] - 2024-08-28

### New Functions 🚀

- `use_toggle`
- `use_prefers_reduced_motion` (thanks to @hcandelaria)

### Breaking Changes 🛠

- `use_websocket` now supports different types for sending and receiving messages
- `SyncSignalOptions` now can take now either transformations or assignment functions but not both.
- updated to `codee` version 0.2.0

### Fixes 🍕

- `use_websocket` fixed error with cleanup and reconnect (thanks to @BakerNet).

### New Features 🚀

- There is now a feature for almost every function to get better compile and rust-analyzer times.
- `use_web_notification` now supports the `vibrate` option (thanks to @hcandelaria).
- `UseDocument` now supports a whole bunch of methods more from `document` (thanks to @luckynumberke7in).

## [0.12.0] - 2024-08-14

> Make sure you also update `cargo-leptos` to the latest version if you use that.

### Breaking Changes 🛠

- Updated to web_sys 0.3.70 which unfortunately is breaking some things.
- `use_clipboard` doesn't need the unstable flags anymore.
- `use_locale` now uses `unic_langid::LanguageIdentifier` and proper locale matching (thanks to @mondeja).
- Removed `UseMouseEventExtractorDefault` and reworked `UseMouseCoordType` (thanks to @carloskiki)
- `use_preferred_dark` and `use_color_mode` now try to read the `Sec-CH-Prefers-Color-Scheme` header in SSR. This brings
  the necessity to enable an additional feature for them (`axum` / `actix` / `spin`).

### Fixes 🍕

- Fixed the codec chapter in the book to refer to crate `codee`.

## [0.11.4] - 2024-08-12

### New Features 🚀

- `use_web_notification` now supports the options `renotify`, `silent` and `image` (thanks to @hcandelaria).
- `sync_signal` no supports the options `assign_ltr` and `assign_rtl`.

## [0.11.3] - 2024-07-31

### Fix 🍕

- Made `use_timeout_fn` SSR-safe

## [0.11.2] - 2024-07-30

### Change 🔥

- `use_locale` has now a supported locale list.

## (yanked) [0.11.1] - 2024-07-28 

### New Functions 🚀

- `use_locale` (thanks to @BrandonDyer64)
- `use_locales` (thanks to @BrandonDyer64)
- `header` – Standard implementations for reading a header on the server.

## [0.11.0] - 2024-07-27

### New Functions 🚀

- `use_user_media`

### New Features 🚀

- Codecs:
    - All codecs now live in their own crate `codee`
    - There are now binary codecs in addition to string codecs.
        - `FromToBytesCodec`
        - `WebpackSerdeCodec`
        - `BincodeSerdeCodec`
        - `ProstCodec` (see also the section "Breaking Changes 🛠" below)
    - Every binary codec can be used as a string codec with the `Base64` wrapper which encodes the binary data as a
      base64
      string.
        - This required feature `base64`
        - It can be wrapped for example like this: `Base64<WebpackSerdeCodec>`.
    - There is now an `OptionCodec` wrapper that allows to wrap any string codec that encodes `T` to encode `Option<T>`.
        - Use it like this: `OptionCodec<FromToStringCodec<f64>>`.

- `ElementMaybeSignal` is now implemented for `websys::HtmlElement` (thanks to @blorbb).
- `UseStorageOptions` now has `delay_during_hydration` which has to be used when you conditionally show parts of
  the DOM controlled by a value from storage. This leads to hydration errors which can be fixed by setting this new
  option to `true`.
- `cookie::SameSite` is now re-exported
- Changing the signal returned by `use_cookie` now tries and changes the headers during SSR.
- New book chapter about codecs
- The macro `use_derive_signal!` is now exported (thanks to @mscofield0).

### Breaking Changes 🛠

- `UseStorageOptions` and `UseEventSourceOptions` no longer accept a `codec` value because this is already provided as a
  generic parameter to the respective function calls.
- Codecs have been refactored. There are now two traits that codecs implement: `Encoder` and `Decoder`. The
  trait `StringCodec` is gone. The methods are now associated methods and their params now always take references.
    - `JsonCodec` has been renamed to `JsonSerdeCodec`.
    - The feature to enable this codec is now called `json_serde` instead of just `serde`.
    - `ProstCodec` now encodes as binary data. If you want to keep using it with string data you can wrap it like
      this: `Base64<ProstCodec>`.
    - All of these structs, traits and features now live in their own crate called `codee`
    - A bunch of new codecs are available. Have a look at the docs for crate `codee`.
- `use_websocket`:
    - `UseWebsocketOptions` has been renamed to `UseWebSocketOptions` (uppercase S) to be consistent with the return
      type.
    - `UseWebSocketOptions::reconnect_limit` and `UseEventSourceOptions::reconnect_limit` is now `ReconnectLimit`
      instead
      of `u64`. Use `ReconnectLimit::Infinite` for infinite retries or `ReconnectLimit::Limited(...)` for limited
      retries.
    - `use_websocket` now uses codecs to send typed messages over the network.
        - When calling you have give type parameters for the message type and the
          codec: `use_websocket::<String, WebpackSerdeCodec>`
        - You can use binary or string codecs.
        - The `UseWebSocketReturn::send` closure now takes a `&T` which is encoded using the codec.
        - The `UseWebSocketReturn::message` signal now returns an `Option<T>` which is decoded using the codec.
        - `UseWebSocketReturn::send_bytes` and `UseWebSocketReturn::message_bytes` are gone.
        - `UseWebSocketOptions::on_message` and `UseWebSocketOptions::on_message_bytes` have been renamed
          to `on_message_raw` and `on_message_raw_bytes`.
        - The new `UseWebSocketOptions::on_message` takes a `&T`.
        - `UseWebSocketOptions::on_error` now takes a `UseWebSocketError` instead of a `web_sys::Event`.
- `use_storage` now always saves the default value to storage if the key doesn't exist yet.
- Renamed `BreakpointsSematic` to `BreakpointsSemantic` and `breakpoints_sematic` to `breakpoints_semantic`
  (note the `n`) (thanks to @mondeja).

### Fixes 🍕

- Fixed auto-reconnect in `use_websocket`
- Fixed typo in compiler error messages in `use_cookie` (thanks to @SleeplessOne1917).
- Fixed potential signal out of scope issue with `use_raf_fn`

### Other Changes 🔥

- Better links in docs that work both in the book and in rustdoc (thanks to @mondeja).
- Better CI/CD (thanks to @EstebanBorai).

## [0.10.10] - 2024-05-10

### Change 🔥

- Added compile-time warning when you use `ssr` feature with `wasm32`. You can enable `wasm_ssr` to remove the warning.

## [0.10.9] - 2024-04-27

### Fixes 🍕

- Fixed `use_color_mode` without cookies and make cookies sync properly with local storage
- Fixed `use_infinite_scroll` edge case bug with disposed signals

## [0.10.8] - 2024-04-19

### Change 🔥

- `use_cookie` now supports Spin out of the box (thanks to @javierEd).

## [0.10.7] - 2024-04-10

### New Function 🚀

- `sync_signal`

### Change 🔥

- `use_color_mode` now supports cookies.

## [0.10.6] - 2024-04-02

### Fixes 🍕

- Corrected docs of `use_cookie`'s `max-age` unit to milliseconds (thanks to @sify21).
- Fixed setting multiple cookies in the browser (thanks to @sbking).

## [0.10.5] - 2024-03-12

### Fix 🍕

- Fixed SSR detection from an url query parameter for `use_color_mode` (thanks to @mondeja).

## [0.10.4] - 2024-03-05

### New Functions 🚀

- `use_event_source`

### Changes 🔥

- Wrapped callbacks in a non-reactive zone to remove potential warnings.
- Updated SSR chapter in the book to make it more clear and beginner-friendly (thanks to @flupke).

## [0.10.3] - 2024-02-23

### New Functions 🚀

- `use_or`
- `use_and`
- `use_not`

### Fix 🍕

- Removed signal warnings from `use_websocket`'s `send...` methods.

### Changes 🔥

- `use_color_mode` now supports detection from an url query parameter. (thanks to @mondeja)

## [0.10.2] - 2024-02-09

### New Functions 🚀

- `use_permission`
- `use_clipboard`
- `use_timeout_fn`

## [0.10.1] - 2024-01-31

### Fix 🍕

- Fixed docs.rs build

## [0.10.0] - 2024-01-31

### New Functions 🚀

- `use_broadcast_channel`
- `use_cookie` (thanks to @rakshith-ravi)
- `use_mouse_in_element`
- `use_device_orientation` (thanks to @mondeja)
- `use_device_pixel_ratio` (thanks to @mondeja)
- `use_element_bounding`

### Breaking Changes 🛠

- The `leptos` version is now 0.6
- The trait `Codec` has been renamed to `StringCodec` and has been moved to `util::StringCodec`.
    - The struct `StringCodec` has been renamed to `FromToStringCodec` and has been moved to `util::FromToStringCodec`.
    - The structs `JsonCodec` and `ProstCodec` have been moved to `util` as well.
- The function `use_storage` now requires type parameters for the stored type and the codec like all the other
  `...storage...` functions.

### Fixes 🍕

- Fixed `use_geolocation` SSR compile issue
- Fixed `use_intl_number_format` maximum fraction digits option

### Changes 🔥

- The `UseMouseReturn` signals `x`, `y`, and `source_type` are now of type `Signal<f64>` instead of `ReadSignal<f64>`.
- You can now convert `leptos::html::HtmlElement<T>` into `Element(s)MaybeSignal`. This should make functions a lot
  easier to use in directives.
- There's now a chapter in the book especially for `Element(s)MaybeSignal`.
- Throttled or debounced callbacks (in watch\__ or _\_fn) no longer are called after the containing scope was cleaned
  up.
- The document returned from `use_document` now supports the methods `query_selector` and `query_selector_all`.

## [0.9.0] - 2023-12-06

### New Functions 🚀

- `use_display_media` (thanks to @seanaye)

### Breaking Changes 🛠

- (@feral-dot-io) The use `use_<type>_storage` functions have been rewritten to use `Codec`s instead of always
  requiring `serde`.
    - This also removes the feature `storage`
    - By default the `StringCodec` is used which relies on types implementing `FromString + ToString`
    - If you want to use `JsonCodec` you have to enable the feature `serde`
    - If you want to use `ProstCodec` (new!) you have to enable the feature `prost`.
- (@feral-dot-io) The Rust flag `--cfg=web_sys_unstable_apis` is not needed anymore since relevant `web_sys` APIs are
  now stable.
  This affects in particular
    - `use_element_size`
    - `use_resize_observer`

### Fixes 🍕

- `use_raf_fn` and `use_timestamp` no longer spam warnings because of `get`ting signals outside of reactive contexts.
- `use_infinite_scroll` no longer calls the callback twice for the same event
- `use_scroll` now uses `try_get_untracked` in the debounced callback to avoid panics if the context has been destroyed
  while the callback was waiting to be called.
- `use_idle` works properly now (no more idles too early).
- `use_web_notification` doesn't panic on the server anymore.

## [0.8.2] - 2023-11-09

### Fixes 🍕

- Fixed SSR for
    - use_timestamp
    - use_raf_fn
    - use_idle

## [0.8.1] - 2023-10-28

### Fixes 🍕

- Using strings for `ElementMaybeSignal` and `ElementsMaybeSignal` is now SSR safe.
    - This fixes specifically `use_color_mode` to work on the server.

## [0.8.0] - 2023-10-24

### New Functions 🚀

- `use_web_notification` (thanks to @centershocks44)
- `use_infinite_scroll`
- `use_service_worker` (thanks to @lpotthast)

### Breaking Changes 🛠

- `use_scroll` returns `impl Fn(T) + Clone` instead of `Box<dyn Fn(T)>`.

### Other Changes 🔥

- `UseScrollReturn` is now documented

## [0.7.2] - 2023-10-21

### Fixes 🍕

- Some functions still used `window()` which could lead to panics in SSR. This is now fixed.
  Specifically for `use_draggable`.

## [0.7.1] - 2023-10-02

### New Function 🚀

- `use_sorted`

## [0.7.0] - 2023-09-30

### New Functions 🚀

- `use_timestamp`
- `use_idle`
- `use_document`
- `use_window`
- `use_geolocation`
- `signal_debounced`
- `signal_throttled`

### Breaking Changes 🛠

- Leptos version is now 0.5
- No `cx: Scope` params are supported/needed anymore because of the changes in Leptos.
  Please check the release notes of Leptos 0.5 for how to upgrade.
- `watch` is now deprecated in favor of `leptos::watch` and will be removed in a future release.
  `watch_with_options` will continue to exist.
- `use_event_listener_with_options` now takes a `UseEventListenerOptions` instead of
  a `web_sys::AddEventListenerOptions`.
- `use_mutation_observer_with_options` now takes a `UseMutationObserverOptions` instead of
  a `web_sys::MutationObserverInit`.
- `use_websocket`:
    - takes now a `&str` instead of a `String` as its `url` parameter.
    - same for the returned `send` method.
    - The `ready_state` return type is now renamed to `ConnectionReadyState` instead of `UseWebSocketReadyState`.
    - The returned signals `ready_state`, `message`, `message_bytes` have now the type
      `Signal<...>` instead of `ReadSignal<...>` to make them more consistent with other functions.
    - The options `reconnect_limit` and `reconnect_interval` now take a `u64` instead of `Option<u64>` to improve DX.
    - The option `manual` has been renamed to `immediate` to make it more consistent with other functions.
      To port please note that `immediate` is the inverse of `manual` (`immediate` = `!manual`).
    - Added documentation how pass it ergonomically as context.
- `use_color_mode`:
    - The optional `on_changed` handler parameters have changed slightly. Please refer to the docs for more details.
- Throttled or debounced functions cannot be `FnOnce` anymore.
- All traits `ClonableFn...` have been removed.

### Other Changes 🔥

- `use_websocket` can use relative urls now
- Callbacks in options don't require to be cloneable anymore
- Callback in `use_raf_fn` doesn't require to be cloneable anymore
- All (!) functions can now be safely called on the server. Specifically this includes the following that before
  panicked on the server:
    - `use_scroll`
    - `use_event_listener`
    - `use_element_hover`
    - `on_click_outside`
    - `use_drop_zone`
    - `use_element_size`
    - `use_element_visibility`
    - `use_resize_observer`
    - `use_intersection_observer`
    - `use_mutation_observer`

### Fixes 🍕

- `use_element_visibility` didn't work in some cases on Chrome properly. This has been fixed.

## [0.6.3] - 2023-08-12

### Fixes 🍕

- `use_websocket` panicked after unmount

## [0.6.2] - 2023-08-03

### Fixes 🍕

- `use_event_listener_with_options` removes the handlers now correctly.

## [0.6.1] - 2023-08-03

### Fixes 🍕

- `use_storage` now uses `.get_untracked()` to avoid warnings.

## [0.6.0] - 2023-07-17

### New Functions 🚀

- `use_draggable`
- `use_to_string`
- `is_err`
- `is_ok`
- `is_none`
- `is_some`
- `use_raf_fn`

### Breaking Changes 🛠

- The following functions now accept a `MaybeRwSignal` as their initial/default value which means
  you can use a synchronized `RwSignal` in those places.
    - `use_color_mode`
    - `use_cycle_list`
    - `use_favicon`
    - `use_storage`
    - `use_local_storage`
    - `use_session_storage`
- Instead of returning `ReadSignal`, the following functions now return `Signal`.
    - `use_color_mode`
    - `use_favicon`
    - `use_storage`
    - `use_local_storage`
    - `use_session_storage`

### Fixes 🍕

- `use_drop_zone` now uses `.get_untracked()` in event handlers

## [0.5.0] - 2023-07-15

### New Functions 🚀

- `use_drop_zone`
- `use_websocket` (thanks @sectore)
- `use_intl_number_format`

### Changes 🔥

- Crate is ready for Server-Side Rendering. Enable feature `ssr` like you do for `leptos`.

## [0.4.1] - 2023-07-07

### New Functions 🚀

- `use_window_focus`
- `use_window_scroll`
- `use_document_visibility`

## [0.4.0] - 2023-07-03

### Breaking Changes 🛠

- Required `leptos` version is now 0.4
- Following the changes in `leptos` there is no longer a `stable` crate feature required in order to use this library
  with a stable toolchain.
  If you want to use it with a nightly toolchain you have to enable the `nightly` feature only on `leptos` directly.
  No change is required for `leptos-use` itself.

## [0.3.3] - 2023-06-24

### New Functions 🚀

- `use_color_mode`
- `use_cycle_list`
- `use_active_element`

### Changes 🔥

- You can now use this crate with the `stable` toolchain (thanks @lpotthast)
- Set leptos dependency to `default-features = false` in order to enable SSR.

## [0.3.2] - 2023-06-17

### New Functions 🚀

- `use_css_var`
- `use_element_hover`

## [0.3.1] - 2023-06-15

### New Functions 🚀

- `use_interval_fn`
- `use_interval`

## [0.3.0] - 2023-06-13

### Breaking Changes 🛠

- `use_event_listener` no longer returns a `Box<dyn Fn()>` but a `impl Fn() + Clone`

### Changes 🔥

- You can now specify a `&str` or `Signal<String>` with CSS selectors wherever a node ref is accepted
- Callbacks of the following functions no longer require `Clone`
    - `use_resize_observer`
    - `use_intersection_observer`
- These functions now also accept multiple target elements in addition to a single one:
    - `use_resize_observer`
    - `use_intersection_observer`

### New Functions 🚀

- `whenever`
- `use_mutation_observer`
- `use_abs`
- `on_click_outside`

## [0.2.1] - 2023-06-11

### New Functions

- `use_intersection_observer`
- `use_element_visibility`

## [0.2.0] - 2023-06-11

### Breaking Changes

- `watch` doesn't accept `immediate` as a direct argument anymore. This is only provided by the option variant.
- `watch` has now variant `watch_with_options` which allows for debouncing and throttling.

### New Functions

- `use_storage`
- `use_local_storage`
- `use_session_storage`
- `watch_debounced`
- `watch_throttled`
- `watch_pausable`
- `use_ceil`
- `use_round`
- `use_media_query`
- `use_preferred_dark`
- `use_preferred_contrast`
- `use_favicon`
- `use_breakpoints`

### Other Changes

- Function count badge in readme

## [0.1.8/9] - 2023-06-05

- Fixed documentation and doc tests running for functions behind `#[cfg(web_sys_unstable_apis)]`

## [0.1.7] - 2023-06-05

### New Function

- `use_element_size`

## [0.1.6] - 2023-06-03

### Changes

- Fixed documentation so all feature are documented

## [0.1.5] - 2023-06-03

### New Functions

- `use_floor`
- `use_max`
- `use_min`

### Changes

- New feature: `math` that has to be activated in order to use the math functions.

## [0.1.4] - 2023-06-02

### New Functions

- `use_supported`
- `use_resize_observer`
- `watch`
- `use_mouse`

### Changes

- Use the crate `default-struct-builder` to provide ergonimic function options.

## [0.1.3] - 2023-05-28

### New Functions

- `use_scroll`
- `use_debounce_fn`

### Other Changes

- Better and more beautiful demo integration into the guide.
