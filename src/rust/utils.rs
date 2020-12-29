use crate::AppRoute;
use js_sys::ArrayBuffer;
use kira::Frame;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, RequestMode, Response};
use yew::{html, Html};
use yew_router::prelude::*;

pub fn load_audio_data(url: &'static str, callback: impl FnOnce(u32, Vec<Frame>) + 'static) {
    std::mem::drop(wasm_bindgen_futures::future_to_promise(
        load_audio_data_async(url, callback),
    ));
}

pub async fn load_audio_data_async(
    url: &'static str,
    callback: impl FnOnce(u32, Vec<Frame>),
) -> Result<JsValue, JsValue> {
    let audio_ctx = AudioContext::new()?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "video/ogg")?;

    let window = web_sys::window().ok_or_else(|| JsValue::from("could not get window handle"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into()?;

    let encoded = JsFuture::from(resp.array_buffer()?).await?;
    let encoded: ArrayBuffer = encoded.dyn_into()?;

    let decoded = JsFuture::from(audio_ctx.decode_audio_data(&encoded)?).await?;
    let decoded: AudioBuffer = decoded.dyn_into()?;

    let left = decoded.get_channel_data(0)?;
    let right = decoded.get_channel_data(1)?;

    let frames = left
        .iter()
        .zip(right.iter())
        .map(|(&left, &right)| Frame { left, right })
        .collect();

    callback(decoded.sample_rate() as u32, frames);

    Ok(JsValue::undefined())
}

pub fn loading(content: &str) -> Html {
    html! {
        <>
            <div class="container title">
                {("Loading ").to_string() + content + "..."}
            </div>
            <RouterButton<AppRoute> classes="centered" route=AppRoute::Index>
                { "Cancel" }
            </RouterButton<AppRoute>>
        </>
    }
}
