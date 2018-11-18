extern crate cfg_if;
extern crate dft;
extern crate futures;
extern crate js_sys;
extern crate serde;
#[macro_use]
extern crate serde_derive;
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

const MIN_FREQ: f32 = 00.0;
const MAX_FREQ: f32 = 20_000.0;

#[wasm_bindgen]
pub struct MaxFreq {
    /// The maximum frequency.
    pub freq: f32,
    /// The volume.
    pub val: f32,
}

#[derive(Serialize)]
struct DataEntry {
    /// The frequency
    x: f32,
    /// The volume.
    y: f32,
}

#[derive(Serialize)]
struct PeakEntry {
    /// The frequency
    x: f32,
    /// The volume.
    y: f32,
    /// The index into the data array.
    index: usize,
}

#[derive(Serialize)]
struct PeakResult {
    /// The volume of the highest peak.
    max: f32,
    peaks: Vec<PeakEntry>,
}

#[wasm_bindgen]
pub fn get_max_frequency(analyser: &AnalyserNode) -> MaxFreq {
    let data = get_data_intern(analyser);
    // Search maximum
    let d = data.iter().max_by(|d1, d2| {
        if d1.y > d2.y {
            Ordering::Greater
        } else if d1.y < d2.y {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }).unwrap();
    //log(&format!("i: {} -> {}     (Del: {})", i, freq, bin_delta));
    MaxFreq { freq: d.x, val: d.y }
}

fn get_data_intern(analyser: &AnalyserNode) -> Vec<DataEntry> {
    let mut data = vec![0f32; analyser.fft_size() as usize];
    analyser.get_float_time_domain_data(&mut data);

    let plan = Plan::new(Operation::Forward, data.len());
    data.transform(&plan);
    let mut complex = dft::unpack(&data);
    let ft_res = complex.drain(..)
        .map(|c| c.norm()).collect::<Vec<_>>();

    let rate = analyser.context().sample_rate();
    let bins = ft_res.len();
    let bin_delta = rate / (bins as f32);
    // Cut off left and right
    let start = (MIN_FREQ / bin_delta) as usize;
    let end = ft_res.len() - (((rate - MAX_FREQ) / bin_delta) as usize);
    let data = &ft_res[start..end];

    let mut res = Vec::with_capacity(data.len());
    let mut x = (start as f32 + 0.5) * bin_delta;
    for &y in data {
        res.push(DataEntry { x, y });
        x += bin_delta;
    }
    res
}

#[wasm_bindgen]
pub fn get_data(analyser: &AnalyserNode) -> JsValue {
    JsValue::from_serde(&get_data_intern(analyser)).unwrap()
}

fn get_peaks_intern(analyser: &AnalyserNode) -> PeakResult {
    let data = get_data_intern(analyser);
    //let avg: f32 = data.iter().map(|v| v.y).sum::<f32>() / data.len() as f32;
    let mut max = 0f32;

    // Collect all peaks higher than `avg`
    let mut peaks = Vec::new();
    let mut last_x = 0.0;
    let mut last_y = std::f32::INFINITY;
    // If we are currently ascending
    let mut ascending = false;
    for (index, &DataEntry { x, y }) in data.iter().enumerate() {
        if y > max / 3.0 {
            if y > max {
                max = y;
            }

            // The last point is a peak
            if y < last_y && ascending {
                peaks.push(PeakEntry {
                    x: last_x,
                    y: last_y,
                    index: index - 1,
                });
            }
            ascending = y >= last_y;
            last_x = x;
            last_y = y;
        }
    }

    peaks.retain(|PeakEntry { y, .. }| *y > max / 3.0);

    PeakResult { max, peaks }
}

#[wasm_bindgen]
pub fn get_peaks(analyser: &AnalyserNode) -> JsValue {
    JsValue::from_serde(&get_peaks_intern(analyser)).unwrap()
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
    let a4index = 58;
    let note_diff = (12f32 * (frequency / 440f32).log2()).round() as i32;
    if note_diff < -a4index || (a4index + note_diff) as usize >= NOTES.len() {
        return "^A,,,,,,,,,".to_string();
    }
    let index = (a4index + note_diff) as usize;
    //log(&format!("{} -> {}", frequency, NOTES[index]));
    NOTES[index].to_string()
}
