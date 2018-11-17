extern crate cfg_if;
extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod utils;

use std::cmp::Ordering;

use cfg_if::cfg_if;
use futures::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use web_sys::{AnalyserNode, AudioContext, MediaStream,
    MediaStreamConstraints};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn analyse_audio(analyser: AnalyserNode) {
    let mut data = vec![0f32; analyser.frequency_bin_count() as usize];
    analyser.get_float_frequency_data(&mut data);
    // Search maximum
    if let Some(m) = data.iter().cloned().enumerate().max_by(|(_, d1), (_, d2)| {
        if d1 > d2 {
            Ordering::Greater
        } else if d1 < d2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }) {
        log(&format!("Maximum: {:?}", m));
    }
}

#[wasm_bindgen]
pub fn setup() -> Result<Promise, JsValue> {
    let audio = AudioContext::new()?;
    let analyser = audio.create_analyser()?;
    // 0 means no time averaging, 1 means no change (old * (1-val) + new * val)
    analyser.set_smoothing_time_constant(0.1);

    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let devices = navigator.media_devices()?;
    let media = JsFuture::from(devices.get_user_media_with_constraints(
        MediaStreamConstraints::new().audio(&JsValue::TRUE))?);

    let res = media.then(move |val| {
        let stream: MediaStream = val?.into();
        let source = audio.create_media_stream_source(&stream)?;
        source.connect_with_audio_node(&analyser)?;

        Ok(analyser.into())
    });

    Ok(future_to_promise(res))
}
