import * as wasm from "visumusic";
import abcjs from "abcjs";

var dataArray;
var analyser;
var notes;

function init_notes() {
  notes = "X: 1\nT:visumusic\nM:4/4\nL:1/8\nK:Emin\n|D2|EB|";
  console.log('init notes');
}

function update_notes() {
  var freq = wasm.get_max_frequency(analyser);
  var note = wasm.note_for_frequency(freq);
  notes += note;
}

function draw() {
  analyser.getFloatFrequencyData(dataArray);
  console.log("data: " + dataArray);
  update_notes();
  abcjs.renderAbc("notation", notes, { scale: 2.0 });
  setTimeout(draw, 1000);
}

async function run() {
  init_notes();
  analyser = await wasm.setup();
  dataArray = new Float32Array(analyser.frequencyBinCount);
  console.log("Setup is ready");
  setInterval(wasm.analyse_audio, 1000, analyser);
  draw();
}

run();
