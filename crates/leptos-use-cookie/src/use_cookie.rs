#![allow(clippy::too_many_arguments)]

use crate::{UseCookieOptions, now};
use codee::{CodecError, Decoder, Encoder};
pub use cookie::SameSite;
use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, CookieJar};
use leptos::logging::debug_warn;
use leptos::prelude::*;
use std::sync::Arc;

/// SSR-friendly and reactive cookie access.
///
/// You can use this function multiple times for the same cookie and their signals will synchronize
/// (even across windows/tabs). But there is no way to listen to changes to `document.cookie` directly so in case
/// something outside of this function changes the cookie, the signal will **not** be updated.
///
/// When the options `max_age` or `expire` is given then the returned signal will
/// automatically turn to `None` after that time.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_cookie)
///
/// ## Usage
///
/// The example below creates a cookie called `counter`. If the cookie doesn't exist, it is initially set to a random value.
/// Whenever we update the `counter` variable, the cookie will be updated accordingly.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_cookie::use_cookie;
/// # use codee::string::FromToStringCodec;
/// # use rand::random;
///
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");
///
/// let reset = move || set_counter.set(Some(random()));
///
/// if counter.get().is_none() {
///     reset();
/// }
///
/// let increase = move || {
///     set_counter.set(counter.get().map(|c| c + 1));
/// };
///
/// view! {
///     <p>Counter: {move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
///     <button on:click=move |_| reset()>Reset</button>
///     <button on:click=move |_| increase()>+</button>
/// }
/// # }
/// ```
///
/// Values are (en)decoded via the given codec. You can use any of the string codecs or a
/// binary codec wrapped in `Base64`.
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
/// > available and what feature flags they require.
///
/// ## Cookie attributes
///
/// As part of the options when you use `use_cookie_with_options` you can specify cookie attributes.
///
/// ```
/// # use cookie::SameSite;
/// # use leptos::prelude::*;
/// # use leptos_use_cookie::{use_cookie_with_options, UseCookieOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (cookie, set_cookie) = use_cookie_with_options::<bool, FromToStringCodec>(
///     "user_info",
///     UseCookieOptions::default()
///         .max_age(3600_000) // one hour
///         .same_site(SameSite::Lax)
/// );
/// #
/// # view! {}
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// This works equally well on the server or the client.
/// On the server this function reads the cookie from the HTTP request header and writes it back into
/// the HTTP response header according to options (if provided).
/// The returned `WriteSignal` may not affect the cookie headers on the server! It will try and write
/// the headers but if this happens after the headers have already been streamed to the client then
/// this will have no effect.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`..
///
/// ### Bring your own header
///
/// In case you're neither using Axum nor Actix or the default implementation is not to your liking,
/// you can provide your own way of reading and writing the cookie header value.
///
/// ```
/// # use cookie::Cookie;
/// # use leptos::prelude::*;
/// # use serde::{Deserialize, Serialize};
/// # use leptos_use_cookie::{use_cookie_with_options, UseCookieOptions};
/// # use codee::string::JsonSerdeCodec;
/// #
/// # #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// # pub struct Auth {
/// #     pub username: String,
/// #     pub token: String,
/// # }
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_cookie_with_options::<Auth, JsonSerdeCodec>(
///     "auth",
///     UseCookieOptions::default()
///         .ssr_cookies_header_getter(|| {
///             #[cfg(feature = "ssr")]
///             {
///                 Some("Somehow get the value of the cookie header as a string".to_owned())
///             }
///             #[cfg(not(feature = "ssr"))]
///             None
///         })
///         .ssr_set_cookie(|cookie: &Cookie| {
///             #[cfg(feature = "ssr")]
///             {
///                 // somehow insert the Set-Cookie header for this cookie
///             }
///         }),
/// );
/// # view! {}
/// # }
/// ```
pub fn use_cookie<T, C>(cookie_name: &str) -> (Signal<Option<T>>, WriteSignal<Option<T>>)
where
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
    T: Clone + PartialEq + Send + Sync + 'static,
    T: std::fmt::Debug,
    <C as Encoder<T>>::Error: std::fmt::Debug,
{
    use_cookie_with_options::<T, C>(cookie_name, UseCookieOptions::default())
}

/// Version of [`use_cookie`] that takes [`UseCookieOptions`].
pub fn use_cookie_with_options<T, C>(
    cookie_name: &str,
    options: UseCookieOptions<T, <C as Encoder<T>>::Error, <C as Decoder<T>>::Error>,
) -> (Signal<Option<T>>, WriteSignal<Option<T>>)
where
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
    T: Clone + PartialEq + Send + Sync + 'static,
    T: std::fmt::Debug,
    <C as Encoder<T>>::Error: std::fmt::Debug,
{
    let UseCookieOptions {
        max_age,
        expires,
        http_only,
        secure,
        domain,
        path,
        same_site,
        ssr_cookies_header_getter,
        ssr_set_cookie,
        default_value,
        readonly,
        on_error,
    } = options;

    let delay = if let Some(max_age) = max_age {
        Some(max_age)
    } else {
        expires.map(|expires| expires * 1000 - now() as i64)
    };

    let has_expired = if let Some(delay) = delay {
        delay <= 0
    } else {
        false
    };

    let jar = StoredValue::new(CookieJar::new());

    let restart_expiration;

    let (cookie, set_cookie) = if !has_expired {
        let ssr_cookies_header_getter = Arc::clone(&ssr_cookies_header_getter);

        let new_cookie = jar.try_update_value(|jar| {
            *jar = load_and_parse_cookie_jar(ssr_cookies_header_getter)?;
            jar.get(cookie_name)
                .and_then(|c| {
                    C::decode(c.value())
                        .map_err(|err| on_error(CodecError::Decode(err)))
                        .ok()
                })
                .or(default_value)
        });

        let out = signal(new_cookie.flatten());
        restart_expiration = handle_expiration(delay, out.1);
        out
    } else {
        debug_warn!(
            "not setting cookie '{}' because it has already expired",
            cookie_name
        );

        restart_expiration = None;

        signal(None::<T>)
    };

    // only used to restart the expiration timer on the client
    #[cfg(feature = "ssr")]
    let _ = restart_expiration;

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{
            UseBroadcastChannelReturn, WatchOptions, use_broadcast_channel, watch_with_options,
        };
        use codee::string::{FromToStringCodec, OptionCodec};

        let UseBroadcastChannelReturn { message, post, .. } =
            use_broadcast_channel::<Option<String>, OptionCodec<FromToStringCodec>>(&format!(
                "leptos-use:cookies:{cookie_name}"
            ));

        let on_cookie_change = {
            let cookie_name = cookie_name.to_owned();
            let ssr_cookies_header_getter = Arc::clone(&ssr_cookies_header_getter);
            let on_error = Arc::clone(&on_error);
            let domain = domain.clone();
            let path = path.clone();
            let restart_expiration = restart_expiration.clone();

            move || {
                if readonly {
                    return;
                }

                let value = cookie.try_with_untracked(|cookie| {
                    cookie.as_ref().and_then(|cookie| {
                        C::encode(cookie)
                            .map_err(|err| on_error(CodecError::Encode(err)))
                            .ok()
                    })
                });

                if let Some(value) = value {
                    let prev_value = jar
                        .read_value()
                        .get(&cookie_name)
                        .map(|c| c.value().to_string());

                    if prev_value == value {
                        return;
                    }

                    if let Some(prev_value) = prev_value
                        && let Ok(prev_parsed_value) = C::decode(&prev_value)
                        && let Some(cookie) = cookie.try_read_untracked()
                        && Some(prev_parsed_value) == *cookie
                    {
                        return;
                    }

                    jar.update_value(|jar| {
                        write_client_cookie(
                            &cookie_name,
                            &value,
                            jar,
                            max_age,
                            expires,
                            &domain,
                            &path,
                            same_site,
                            secure,
                            http_only,
                            Arc::clone(&ssr_cookies_header_getter),
                        );
                    });

                    // The cookie was just written with a fresh `Max-Age`, so
                    // the countdown that clears the signal starts over too.
                    if let Some(restart_expiration) = &restart_expiration {
                        restart_expiration();
                    }

                    post(&value);
                }
            }
        };

        // Marks the next watcher invocation as originating from the broadcast
        // channel. The watcher runs deferred, i.e. after the message handler has
        // returned, so the flag has to survive until then.
        let from_broadcast = StoredValue::new(false);

        let stop = watch_with_options(
            move || cookie.track(),
            {
                let on_cookie_change = on_cookie_change.clone();

                move |_, _, _| {
                    if from_broadcast.get_value() {
                        from_broadcast.set_value(false);
                        return;
                    }

                    on_cookie_change();
                }
            },
            WatchOptions::default(),
        );

        // listen to cookie changes from the broadcast channel
        Effect::new({
            let ssr_cookies_header_getter = Arc::clone(&ssr_cookies_header_getter);
            let cookie_name = cookie_name.to_owned();

            move |_| {
                if let Some(message) = message.get() {
                    if let Some(message) = message {
                        match C::decode(&message) {
                            Ok(value) => {
                                let ssr_cookies_header_getter =
                                    Arc::clone(&ssr_cookies_header_getter);

                                jar.update_value(|jar| {
                                    update_client_cookie_jar(
                                        &cookie_name,
                                        &Some(message),
                                        jar,
                                        max_age,
                                        expires,
                                        &domain,
                                        &path,
                                        same_site,
                                        secure,
                                        http_only,
                                        ssr_cookies_header_getter,
                                    );
                                });

                                from_broadcast.set_value(true);
                                set_cookie.set(Some(value));
                            }
                            Err(err) => {
                                on_error(CodecError::Decode(err));
                            }
                        }
                    } else {
                        let cookie_name = cookie_name.clone();
                        let ssr_cookies_header_getter = Arc::clone(&ssr_cookies_header_getter);

                        jar.update_value(|jar| {
                            update_client_cookie_jar(
                                &cookie_name,
                                &None,
                                jar,
                                max_age,
                                expires,
                                &domain,
                                &path,
                                same_site,
                                secure,
                                http_only,
                                ssr_cookies_header_getter,
                            );
                            jar.force_remove(cookie_name);
                        });

                        from_broadcast.set_value(true);
                        set_cookie.set(None);
                    }
                }
            }
        });

        on_cleanup(move || {
            stop();
            on_cookie_change();
        });

        let _ = ssr_set_cookie;
    }

    #[cfg(feature = "ssr")]
    {
        if !readonly {
            let _ = ImmediateEffect::new_isomorphic({
                let cookie_name = cookie_name.to_owned();
                let ssr_set_cookie = Arc::clone(&ssr_set_cookie);

                let lock = Arc::new(std::sync::Mutex::new(()));

                move || {
                    if let Ok(_lock_guard) = lock.clone().lock() {
                        let domain = domain.clone();
                        let path = path.clone();

                        if let Some(value) = cookie.try_with(|cookie| {
                            cookie.as_ref().map(|cookie| {
                                C::encode(cookie)
                                    .map_err(|err| on_error(CodecError::Encode(err)))
                                    .ok()
                            })
                        }) {
                            jar.update_value({
                                let domain = domain.clone();
                                let path = path.clone();
                                let ssr_set_cookie = Arc::clone(&ssr_set_cookie);

                                |jar| {
                                    write_server_cookie(
                                        &cookie_name,
                                        value.flatten(),
                                        jar,
                                        max_age,
                                        expires,
                                        domain,
                                        path,
                                        same_site,
                                        secure,
                                        http_only,
                                        ssr_set_cookie,
                                    )
                                }
                            });
                        }
                    }
                }
            });
        }
    }

    (cookie.into(), set_cookie)
}

fn read_cookies_string(
    ssr_cookies_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
) -> Option<String> {
    let cookies;

    #[cfg(feature = "ssr")]
    {
        cookies = ssr_cookies_header_getter();
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        let _ = ssr_cookies_header_getter;

        let js_value: wasm_bindgen::JsValue = document().into();
        let document: web_sys::HtmlDocument = js_value.unchecked_into();
        cookies = Some(document.cookie().unwrap_or_default());
    }

    cookies
}

/// Clears the cookie signal once `delay` has elapsed. Returns a closure that
/// restarts the countdown, which has to be called whenever the cookie is
/// rewritten, because writing renews the cookie's own `Max-Age`.
fn handle_expiration<T>(
    delay: Option<i64>,
    set_cookie: WriteSignal<Option<T>>,
) -> Option<Arc<dyn Fn() + Send + Sync>>
where
    T: Send + Sync + 'static,
{
    if let Some(delay) = delay {
        #[cfg(not(feature = "ssr"))]
        {
            use leptos::leptos_dom::helpers::TimeoutHandle;
            use std::sync::{Mutex, atomic::AtomicI32};

            // The maximum value allowed on a timeout delay.
            // Reference: https://developer.mozilla.org/en-US/docs/Web/API/setTimeout#maximum_delay_value
            const MAX_TIMEOUT_DELAY: i64 = 2_147_483_647;

            let timeout = Arc::new(Mutex::new(None::<TimeoutHandle>));
            let elapsed = Arc::new(AtomicI32::new(0));

            on_cleanup({
                let timeout = Arc::clone(&timeout);
                move || {
                    if let Some(timeout) = timeout.lock().unwrap().take() {
                        timeout.clear();
                    }
                }
            });

            let create_expiration_timeout =
                Arc::new(Mutex::new(None::<Box<dyn Fn() + Send + Sync>>));

            *create_expiration_timeout.lock().unwrap() = Some(Box::new({
                let timeout = Arc::clone(&timeout);
                let elapsed = Arc::clone(&elapsed);
                let create_expiration_timeout = Arc::clone(&create_expiration_timeout);

                move || {
                    if let Some(timeout) = timeout.lock().unwrap().take() {
                        timeout.clear();
                    }

                    let time_remaining =
                        delay - elapsed.load(std::sync::atomic::Ordering::Relaxed) as i64;
                    let timeout_length = time_remaining.min(MAX_TIMEOUT_DELAY);

                    let elapsed = Arc::clone(&elapsed);
                    let create_expiration_timeout = Arc::clone(&create_expiration_timeout);

                    *timeout.lock().unwrap() = set_timeout(
                        move || {
                            let elapsed = elapsed.fetch_add(
                                timeout_length as i32,
                                std::sync::atomic::Ordering::Relaxed,
                            ) as i64
                                + timeout_length;

                            if elapsed < delay {
                                if let Some(create_expiration_timeout) =
                                    create_expiration_timeout.lock().unwrap().as_ref()
                                {
                                    create_expiration_timeout();
                                }
                                return;
                            }

                            set_cookie.set(None);
                        },
                        std::time::Duration::from_millis(timeout_length as u64),
                    )
                    .ok();
                }
            }));

            if let Some(create_expiration_timeout) =
                create_expiration_timeout.lock().unwrap().as_ref()
            {
                create_expiration_timeout();
            };

            return Some(Arc::new({
                let elapsed = Arc::clone(&elapsed);
                let create_expiration_timeout = Arc::clone(&create_expiration_timeout);

                move || {
                    elapsed.store(0, std::sync::atomic::Ordering::Relaxed);

                    if let Some(create_expiration_timeout) =
                        create_expiration_timeout.lock().unwrap().as_ref()
                    {
                        create_expiration_timeout();
                    }
                }
            }));
        }

        #[cfg(feature = "ssr")]
        {
            let _ = set_cookie;
            let _ = delay;
        }
    }

    None
}

#[cfg(not(feature = "ssr"))]
fn write_client_cookie(
    name: &str,
    value: &Option<String>,
    jar: &mut CookieJar,
    max_age: Option<i64>,
    expires: Option<i64>,
    domain: &Option<String>,
    path: &Option<String>,
    same_site: Option<SameSite>,
    secure: bool,
    http_only: bool,
    ssr_cookies_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
) {
    use wasm_bindgen::JsCast;

    update_client_cookie_jar(
        name,
        value,
        jar,
        max_age,
        expires,
        domain,
        path,
        same_site,
        secure,
        http_only,
        ssr_cookies_header_getter,
    );

    let document = document();
    let document: &web_sys::HtmlDocument = document.unchecked_ref();

    document.set_cookie(&cookie_jar_to_string(jar, name)).ok();
}

#[cfg(not(feature = "ssr"))]
fn update_client_cookie_jar(
    name: &str,
    value: &Option<String>,
    jar: &mut CookieJar,
    max_age: Option<i64>,
    expires: Option<i64>,
    domain: &Option<String>,
    path: &Option<String>,
    same_site: Option<SameSite>,
    secure: bool,
    http_only: bool,
    ssr_cookies_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
) {
    if let Some(new_jar) = load_and_parse_cookie_jar(ssr_cookies_header_getter) {
        *jar = new_jar;
        if let Some(value) = value {
            let cookie = build_cookie_from_options(
                name, max_age, expires, http_only, secure, path, same_site, domain, value,
            );

            jar.add_original(cookie);
        } else {
            let max_age = Some(0);
            let expires = Some(0);
            let value = "";
            let cookie = build_cookie_from_options(
                name, max_age, expires, http_only, secure, path, same_site, domain, value,
            );

            jar.add(cookie);
        }
    }
}

#[cfg(not(feature = "ssr"))]
fn cookie_jar_to_string(jar: &CookieJar, name: &str) -> String {
    match jar.get(name) {
        Some(c) => c.encoded().to_string(),
        None => "".to_string(),
    }
}

fn build_cookie_from_options(
    name: &str,
    max_age: Option<i64>,
    expires: Option<i64>,
    http_only: bool,
    secure: bool,
    path: &Option<String>,
    same_site: Option<SameSite>,
    domain: &Option<String>,
    value: &str,
) -> Cookie<'static> {
    let mut cookie = Cookie::build((name, value));
    if let Some(max_age) = max_age {
        cookie = cookie.max_age(Duration::milliseconds(max_age));
    }
    if let Some(expires) = expires {
        match OffsetDateTime::from_unix_timestamp(expires) {
            Ok(expires) => {
                cookie = cookie.expires(expires);
            }
            Err(err) => {
                debug_warn!("failed to set cookie expiration: {:?}", err);
            }
        }
    }
    if http_only {
        cookie = cookie.http_only(true);
    }
    if secure {
        cookie = cookie.secure(true);
    }
    if let Some(domain) = domain {
        cookie = cookie.domain(domain);
    }
    if let Some(path) = path {
        cookie = cookie.path(path);
    }
    if let Some(same_site) = same_site {
        cookie = cookie.same_site(same_site);
    }

    let cookie: Cookie = cookie.into();
    cookie.into_owned()
}

#[cfg(feature = "ssr")]
fn write_server_cookie(
    name: &str,
    value: Option<String>,
    jar: &mut CookieJar,
    max_age: Option<i64>,
    expires: Option<i64>,
    domain: Option<String>,
    path: Option<String>,
    same_site: Option<SameSite>,
    secure: bool,
    http_only: bool,
    ssr_set_cookie: Arc<dyn Fn(&Cookie) + Send + Sync>,
) {
    if let Some(value) = value {
        let cookie: Cookie = build_cookie_from_options(
            name, max_age, expires, http_only, secure, &path, same_site, &domain, &value,
        );

        jar.add(cookie.into_owned());
    } else {
        jar.remove(name.to_owned());
    }

    for cookie in jar.delta() {
        ssr_set_cookie(cookie);
    }
}

fn load_and_parse_cookie_jar(
    ssr_cookies_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
) -> Option<CookieJar> {
    read_cookies_string(ssr_cookies_header_getter).map(|cookies| {
        let mut jar = CookieJar::new();
        for cookie in Cookie::split_parse_encoded(cookies).flatten() {
            jar.add_original(cookie);
        }

        jar
    })
}
