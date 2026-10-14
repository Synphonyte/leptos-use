use crate::{OptionLocalRwSignal, UseDisplayMediaOptions, UseDisplayMediaReturn, sendwrap_fn};
use cfg_if::cfg_if;
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

/// Reactive [`mediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia) streaming.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_display_media)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::{log, error};
/// # use leptos_use_display_media::{use_display_media, UseDisplayMediaReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let video_ref = NodeRef::<leptos::html::Video>::new();
///
/// let UseDisplayMediaReturn { stream, start, .. } = use_display_media();
///
/// start();
///
/// Effect::new(move |_|
///     video_ref.get().map(|v| {
///         match stream.get() {
///             Some(Ok(s)) => v.set_src_object(Some(&s)),
///             Some(Err(e)) => error!("Failed to get media stream: {:?}", e),
///             None => log!("No stream yet"),
///         }
///     })
/// );
///
/// view! { <video node_ref=video_ref controls=false autoplay=true muted=true></video> }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `start` and `stop` are sendwrapped functions. They can
/// only be called from the same thread that called `use_display_media`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server calls to `start` or any other way to enable the stream will be ignored
/// and the stream will always be `None`.
pub fn use_display_media()
-> UseDisplayMediaReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    use_display_media_with_options(UseDisplayMediaOptions::default())
}

/// Version of [`use_display_media`] that accepts a [`UseDisplayMediaOptions`].
pub fn use_display_media_with_options(
    options: UseDisplayMediaOptions,
) -> UseDisplayMediaReturn<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync> {
    let UseDisplayMediaOptions { enabled, audio } = options;

    let (enabled, set_enabled) = enabled.into_signal();

    let stream = OptionLocalRwSignal::<Result<web_sys::MediaStream, JsValue>>::new();

    let _start = move || async move {
        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            if stream.get_untracked().is_some() {
                return;
            }

            let new_stream = create_media(audio).await;

            stream.update(|s| *s = Some(new_stream));
        } else {
            let _ = audio;
        }}
    };

    let _stop = move || {
        if let Some(sendwrapped_stream) = stream.get_untracked()
            && let Ok(stream) = sendwrapped_stream.as_ref()
        {
            for track in stream.get_tracks() {
                track.unchecked_ref::<web_sys::MediaStreamTrack>().stop();
            }
        }

        stream.set(None);
    };

    // Without this the tracks keep running (and the screen sharing indicator
    // stays on) after the owner is disposed, with no handle left to stop them.
    #[cfg(not(feature = "ssr"))]
    on_cleanup(_stop);

    let start = sendwrap_fn!(move || {
        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            leptos::task::spawn_local(async move {
                _start().await;
                stream.with_untracked(move |stream| {
                    if let Some(sendwrapped_stream) = stream && sendwrapped_stream.as_ref().is_ok() {
                        set_enabled.set(true);
                    }
                });
            });
        }}
    });

    let stop = sendwrap_fn!(move || {
        _stop();
        set_enabled.set(false);
    });

    Effect::watch(
        move || enabled.get(),
        move |enabled, _, _| {
            if *enabled {
                leptos::task::spawn_local(async move {
                    _start().await;
                });
            } else {
                _stop();
            }
        },
        true,
    );

    UseDisplayMediaReturn {
        stream: stream.read_only(),
        start,
        stop,
        enabled,
        set_enabled,
    }
}

#[cfg(not(feature = "ssr"))]
async fn create_media(audio: bool) -> Result<web_sys::MediaStream, JsValue> {
    use crate::js_fut;
    use crate::use_window;

    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let constraints = web_sys::DisplayMediaStreamConstraints::new();
    if audio {
        constraints.set_audio(&JsValue::from(true));
    }

    let promise = media.get_display_media_with_constraints(&constraints)?;
    let res = js_fut!(promise).await?;

    Ok::<_, JsValue>(web_sys::MediaStream::unchecked_from_js(res))
}
