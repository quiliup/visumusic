import * as wasm from "visumusic";

var dataArray;
var analyser;

function draw() {
  analyser.getFloatFrequencyData(dataArray);
  console.log("data: " + dataArray);
  //setTimeout(draw, 1000);
}

async function run() {
  analyser = await wasm.setup();
  dataArray = new Float32Array(analyser.frequencyBinCount);
  console.log("Setup is ready");
  setInterval(wasm.analyse_audio, 1000, analyser);
  draw();
}

run();
