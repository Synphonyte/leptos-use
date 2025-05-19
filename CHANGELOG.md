# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.16.0-beta] - 2025-03-20

### Breaking Change 🛠

- Updated dependency Leptos to version `0.8.0-beta` (thanks to @ifiokjr)

### Special thanks to our sponsor
- @spencewenski

## [0.16.0-alpha] - 2025-03-17

### Breaking Changes 🛠

- Removed the feature `spin` and it's backend integration dependency `leptos-spin` (thanks to @ifiokjr)
- Updated dependency Leptos to version `0.8.0-alpha` (thanks to @ifiokjr)
- Updated dependency getrandom to version `0.3`
- Updated dependency rand to version `0.9`

### Special thanks to our sponsor
- @spencewenski

## [0.15.7] - 2025-03-17

### Change

- Updated codee version to 0.3.0 in line with the latest 0.7 Leptos versions

### Special thanks to our sponsor
- @spencewenski


## [0.15.6] - 2025-02-08

### Fix 🍕

- Fixed `use_storage` overwriting stored values with SSR (thanks to @BakerNet).

### Special thanks to our sponsor
- @spencewenski


## [0.15.5] - 2025-01-15

### Fix 🍕

- `sync_signal` with `immediate = true` now syncs the signals on the server once initially. This fixes `use_color_mode`
  with cookies enabled to give wrong results during SSR.

### Special thanks to our sponsor
- @spencewenski


## [0.15.4] - 2025-01-15

### Fixes 🍕

- downgraded codee to 0.2.0 to be compatible with Leptos 0.7
- fixed `use_mutation_observer` (thanks to @bpwarner)

### Changes 🔥

- improxed DX: implemented `Clone + Copy` for `UseDropZoneReturn` (thanks to @mahmoud-eltahawy)

### Special thanks to our sponsor
- @spencewenski


## ~~[0.15.3] - 2025-01-08~~ (yanked)

### New Function 🚀

- `use_calendar`

### Fix 🍕

- added `Debug` to `Size` (thanks to @Ahlman)

### Special thanks to our sponsor
- @spencewenski

## [0.15.2] - 2025-01-03

### Fixes 🍕

- Fixed path of `use_color_mode` cookie
- `ElementMaybeSignal` and `ElementsMaybeSignal` are now properly `Clone` and `Copy`

### Special thanks to our sponsor
- @spencewenski

## [0.15.1] - 2024-12-31

### Fixes 🍕

- Fixed `use_element_hover` not properly cancelling it's timeout (thanks to @jcold).
- Fixed `use_storage` not writing default values.
- Fixed unidirectional `sync_signal` not syncing properly.

### Special thanks to our sponsors
- @spencewenski
- [LeftClick](https://www.leftclick.cloud/)


## [0.15.0] - 2024-12-17

### New Functions 🚀

- `signal_throttled` and `signal_debounced` now have `..._local` variants (thanks to @bicarlsen)

### Breaking Changes 🛠

- `use_storage` now accepts a Signal as it's `key` parameter (thanks to [LeftClick](https://www.leftclick.cloud/))
- `use_websocket` now supports sending heartbeats (thanks to [LeftClick](https://www.leftclick.cloud/))

### Fix 🍕

- Fixed `use_storage` to actually remove the key when `remove` is called (thanks to @flaviopezzini)

### Special thanks to our sponsors
- @spencewenski
- [LeftClick](https://www.leftclick.cloud/)

## [0.14.0] - 2024-12-01

### Highlights since 0.13

- Updated to Leptos 0.7
- Refactored `ElementMaybeSignal` and `ElementsMaybeSignal` to have a simpler implementation. For the vast majority
  of cases this should continue to work as before.
- Almost everything returned from functions is now `Send + Sync`.

### Changes since 0.14.0-rc5

- Updated Leptos to use stable 0.7 version
- Updated wasm-bindgen to 0.2.96
- Updated web-sys 0.3.73

Special thanks to our sponsor:
- @spencewenski

## [0.14.0-rc5] - 2024-11-27

- fixed error messages for get_header
- added Send + Sync to storage return closure

Special thanks to our sponsor:
- @spencewenski

## [0.14.0-rc4] - 2024-11-26

- Updated to Leptos 0.7.0-rc2
- Fixed WASM on the serverside (thanks to @GuillaumeDelorme)
- Fixed `use_storage`.
- Made all returned closures `Send + Sync`.

Special thanks to our sponsor:
- @spencewenski

## [0.14.0-rc3] - 2024-11-14

- Fixed potential SSR panic in use_locale(s) (thanks to @veigaribo)
- Make `use_locale` prioritize user preferred locales over app preferred ones

## [0.14.0-rc2] - 2024-11-10

- Updated to Leptos 0.7.0-rc1
- Updated to web-sys 0.3.72 and unpinned version (thanks to @sabify)
- Added dependabot (thanks to @sabify)
- Reverted use_user_media to have video enabled by default
- Fixed exponential increase on websocket reconnects

## [0.14.0-rc1] - 2024-11-06

- Fixed MediaTrackConstraints dependency (thanks to @mollymorphous)
- Fixed warnings and tests (thanks to @SleeplessOne1917 and @jheuel)
- Unpinned the wasm-bindgen version (thanks to @jheuel)

## [0.14.0-rc0] - 2024-11-03

- Latest changes up to version 0.13.7 ported
- Updated to Leptos 0.7.0-rc0

## [0.14.0-gamma2] - 2024-10-16

- Updated to Leptos 0.7.0-gamma3 by using `Signal` instead of `MaybeSignal`

## [0.14.0-gamma1] - 2024-10-10

- Adapted to the latest changes in Leptos (thanks to @BakerNet and @nikessel)
- Fixed all the examples
- `use_active_element` ported
- `use_drop_zone` now returns `Signal<Vec<SendSignal<web_sys::File>>>` instead of `Signal<Vec<web_sys::File>, LocalStorage>` 
  to make it easier to use with `<For>`

## [0.14.0-beta4] - 2024-09-15

- Latest changes from version 0.13.4 and 0.13.5 ported

## [0.14.0-beta3] - 2024-09-02

### Breaking Changes 🛠

- Refactored `ElementMaybeSignal` and `ElementsMaybeSignal` to have a simpler implementation. For the vast majority
  of cases this should continue to work as before.

## [0.14.0-beta2] - 2024-09-09

### Change 🔥

- Latest Leptos 0.7 beta changed the trigger trait method (thanks to @BakerNet)
- Latest changes from version 0.13.3 ported

## [0.14.0-beta1] - 2024-09-02

Ported everything to Leptos 0.7
Some example don't run yet.

## [0.13.12] - 2025-01-03

- Fixed path of `use_color_mode` cookie

Thanks to our generous sponsor:
- @spencewenski

## [0.13.11] - 2024-11-22

- Updated web-sys version to 0.3.72

Thanks to our generous sponsor:
- @spencewenski

## [0.13.10] - 2024-11-14

- Fixed potential SSR crash in `use_locale(s)` (thanks to @veigaribo)

## [0.13.9] - 2024-11-10

- Reverted use_user_media to have video enabled by default
- Fixed exponential increase on websocket reconnects

## [0.13.8] - 2024-11-06

- Backported fixes from 0.14.0-rc1

## [0.13.7] - 2024-10-20

- Added video and audio options to `use_user_media` (thanks to @sauloco).
- Fixed cookies in SSR (thanks to @jim-taylor-business).

## [0.13.6] - 2024-10-20

- Updated leptos-spin version to 0.2 (thanks to @tqq1994516).

## [0.13.5] - 2024-09-15

### New Function 🚀

- `use_textarea_autosize`

## [0.13.4] - 2024-09-05

### Fix 🍕

- `use_websocket` now returns a signal for the websocket instance so the user can actually use it. Before it always
  returned `None`.

## [0.13.3] - 2024-09-02

### Fix 🍕

- Fixed `use_color_mode` with cookies enabled

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
