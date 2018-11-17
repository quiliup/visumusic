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
pub fn analyse_audio(analyser: &AnalyserNode) {
    //let mut data = vec![0f32; analyser.frequency_bin_count() as usize];
    //analyser.get_float_frequency_data(&mut data);
    let mut data = vec![0f32; analyser.fft_size() as usize];
    analyser.get_float_time_domain_data(&mut data);
    let plan = Plan::new(Operation::Forward, data.len());
    data.transform(&plan);
    let mut complex = dft::unpack(&data);
    let ft_res = complex.drain(..).map(|c| c.re).collect::<Vec<_>>();
    let data = ft_res;
    log(&format!("{:?}", data));

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
        let freq = get_freq(analyser, m.0);
        log(&format!("Maximum (level: {:.3}, freq: {}): {}", m.1, freq,
            note_for_frequency(freq)));
    }
}

fn get_freq(analyser: &AnalyserNode, bin: usize) -> f32 {
    let rate = analyser.context().sample_rate();
    //let bins = analyser.frequency_bin_count() as f32;
    let bins = analyser.fft_size() as f32;
    let bin_delta = rate / bins / 2f32;
    log(&format!("Sample rate: {} Hz\n#bins: {}\nper bin: {} Hz", rate, bins,
        bin_delta));
    bin as f32 * bin_delta
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

const NOTES: &[&str] = &[
    "C0","C#0","D0","D#0","E0","F0","F#0","G0","G#0","A0","A#0","B0",
    "C1","C#1","D1","D#1","E1","F1","F#1","G1","G#1","A1","A#1","B1",
    "C2","C#2","D2","D#2","E2","F2","F#2","G2","G#2","A2","A#2","B2",
    "C3","C#3","D3","D#3","E3","F3","F#3","G3","G#3","A3","A#3","B3",
    "C4","C#4","D4","D#4","E4","F4","F#4","G4","G#4","A4","A#4","B4",
    "C5","C#5","D5","D#5","E5","F5","F#5","G5","G#5","A5","A#5","B5",
    "C6","C#6","D6","D#6","E6","F6","F#6","G6","G#6","A6","A#6","B6",
    "C7","C#7","D7","D#7","E7","F7","F#7","G7","G#7","A7","A#7","B7",
    "C8","C#8","D8","D#8","E8","F8","F#8","G8","G#8","A8","A#8","B8",
    "C9","C#9","D9","D#9","E9","F9","F#9","G9","G#9","A9","A#9","B9" ];

#[wasm_bindgen]
pub fn note_for_frequency(frequency: f32) -> String {
    let a4index = 47;
    let note_diff = (12f32 * (frequency / 440f32).log2()) as usize;
    let index = a4index + note_diff;
    NOTES[index].to_string()
}
