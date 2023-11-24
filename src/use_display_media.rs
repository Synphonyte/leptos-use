use leptos::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{DisplayMediaStreamConstraints, MediaStream};
use crate::use_window::use_window;

async fn create_media(opts: Option<DisplayMediaStreamConstraints>) -> Result<MediaStream, JsValue> {
    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let promise = match opts {
        Some(o) => media.get_display_media_with_constraints(&o),
        None => media.get_display_media(),
    }?;
    let res = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok::<_, JsValue>(MediaStream::unchecked_from_js(res))
}

type UseDisplayReturn = Resource<Option<DisplayMediaStreamConstraints>, Result<MediaStream, JsValue>>;

pub fn use_display_media<S>(options: S) -> UseDisplayReturn
where
    S: Into<MaybeSignal<Option<DisplayMediaStreamConstraints>>>,
{
    let opts: MaybeSignal<Option<DisplayMediaStreamConstraints>> = options.into();
    create_local_resource(move || opts.with(|o| o.as_ref().cloned()), create_media)
}

