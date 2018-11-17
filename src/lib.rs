extern crate cfg_if;
extern crate dft;
extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod utils;

use std::cmp::Ordering;

use cfg_if::cfg_if;
use dft::{Transform, Operation, Plan};
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
pub fn get_max_frequency(analyser: &AnalyserNode) -> f32 {
    //let mut data = vec![0f32; analyser.frequency_bin_count() as usize];
    //analyser.get_float_frequency_data(&mut data);
    let mut data = vec![0f32; analyser.fft_size() as usize];
    analyser.get_float_time_domain_data(&mut data);
    let plan = Plan::new(Operation::Forward, data.len());
    data.transform(&plan);
    let mut complex = dft::unpack(&data);
    let len = complex.len() / 4;
    let ft_res = complex.drain(..).take(len)
        .map(|c| c.re).collect::<Vec<_>>();
    let data = ft_res;
    // log(&format!("{:?}", data));

    // Search maximum
    if let Some(m) = data.iter().cloned().enumerate()
        .max_by(|(_, d1), (_, d2)| {
        if d1 > d2 {
            Ordering::Greater
        } else if d1 < d2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }) {
        let rate = analyser.context().sample_rate();
        get_freq(m.0, rate, data.len())
    } else {
        -1.0
    }
}

#[wasm_bindgen]
pub fn get_data(analyser: &AnalyserNode) -> Vec<f32> {
    let mut data = vec![0f32; analyser.fft_size() as usize];
    analyser.get_float_time_domain_data(&mut data);

    let plan = Plan::new(Operation::Forward, data.len());
    data.transform(&plan);
    let mut complex = dft::unpack(&data);
    let len = complex.len();
    let ft_res = complex.drain(..).take(len)
        .map(|c| c.norm()).collect::<Vec<_>>();
    ft_res
}

#[wasm_bindgen]
pub fn analyse_audio(analyser: &AnalyserNode) {
    let freq = get_max_frequency(analyser);
    if freq < 0f32 { return; }
    /* let rate = analyser.context().sample_rate();
    let freq = get_freq2(m.0, rate, data.len());
    log(&format!("Maximum (level: {:.3}, freq: {}): {}", m.1, freq,
        note_for_frequency(freq))); */
}

fn get_freq(bin: usize, rate: f32, bins: usize) -> f32 {
    let bin_delta = rate / (bins as f32) / 2f32;
    (bin as f32 + 0.5) * bin_delta
}

#[wasm_bindgen]
pub fn setup() -> Result<Promise, JsValue> {
    let audio = AudioContext::new()?;
    let analyser = audio.create_analyser()?;
    // 0 means no time averaging, 1 means no change (old * (1-val) + new * val)
    analyser.set_smoothing_time_constant(0.0);

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

const NOTES: &[&str] = &[
    "C,,,,,","^C,,,,,","D,,,,,","^D,,,,,","E,,,,,","F,,,,,","^F,,,,,","G,,,,,","^G,,,,,","A,,,,,","^A,,,,,","B,,,,,",
    "C,,,,","^C,,,,","D,,,,","^D,,,,","E,,,,","F,,,,","^F,,,,","G,,,,","^G,,,,","A,,,,","^A,,,,","B,,,,",
    "C,,,","^C,,,","D,,,","^D,,,","E,,,","F,,,","^F,,,","G,,,","^G,,,","A,,,","^A,,,","B,,,",
    "C,,","^C,,","D,,","^D,,","E,,","F,,","^F,,","G,,","^G,,","A,,","^A,,","B,,",
    "C,","^C,","D,","^D,","E,","F,","^F,","G,","^G,","A,","^A,","B,",
    "C","^C","D","^D","E","F","^F","G","^G","A","^A","B",
    "c","^c","d","^d","e","f","^f","g","^g","a","^a","b",
    "c'","^c'","d'","^d'","e'","f'","^f'","g'","^g'","a'","^a'","b'",
    "c''","^c''","d''","^d''","e''","f''","^f''","g''","^g''","a''","^a''","b''",
    "c'''","^c'''","d'''","^d'''","e'''","f'''","^f'''","g'''","^g'''","a'''","^a'''","b'''" ];

#[wasm_bindgen]
pub fn note_for_frequency(frequency: f32) -> String {
    let a4index = 47;
    let note_diff = (12f32 * (frequency / 440f32).log2()).round() as usize;
    let index = a4index + note_diff;
    NOTES[index].to_string()
}
