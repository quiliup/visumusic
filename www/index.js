import * as wasm from "visumusic";
import abcjs from "abcjs";

var dataArray;
var analyser;
var notes;
var note_counter;
var performanceChart;
var first_note = true;


var dps = 5000;

function init_notes() {
  notes = "X: 1\nT:visumusic\nM:4/4\nL:1/4\nK:none\n|";
  console.log('init notes');
  note_counter = 0;
}

function update_notes() {
  if (first_note) {
    first_note = false;
    return;
  }
  var freq = wasm.get_max_frequency(analyser);
  var note = wasm.note_for_frequency(freq);

  if (note.endsWith("''''") || note.endsWith(",,")) {
    note = "z"
  }

  notes += note;
  note_counter += 1;
  if (note_counter % 4 == 0) {
    notes += "|"
  } else {
    notes += " "
  }
  if (note_counter % 20 == 0) {
    notes += "\n|"
  }
}

function draw() {
  //analyser.getFloatFrequencyData(dataArray);
  //console.log("data: " + dataArray);
  dataArray = wasm.get_data(analyser);
  update_notes();
  abcjs.renderAbc("notation", notes, { scale: 2.0 });
  setTimeout(draw, 1000);
}

async function run() {
  init_notes();
    analyser = await wasm.setup();
    dataArray = new Float32Array(analyser.frequencyBinCount);
    console.log("Setup is ready");
    //setInterval(wasm.analyse_audio, 1000, analyser);
    draw();
    performanceChart = new CanvasJS.Chart("container",
    {
        zoomEnabled: false,
        panEnabled: false,
        legend: {
            horizontalAlign: "right",
            verticalAlign: "center"
        },
        axisX: {
          title: "Frequency",
          logarithmic: true,
          logarithmBase: 2
        },
        axisY: {
            includeZero: false
        },
        data: [],  // random generator below
    });
}

function update(){
    var dataPoints = [];
    if (analyser === undefined || performanceChart === undefined)
        return;
    for (var i = 0; i < analyser.frequencyBinCount; i += 1) {
        dataPoints.push({
            x: i+1,
            y: dataArray[i]
        });
    }
    performanceChart.options.data = [{type: "line", dataPoints: dataPoints}];
    performanceChart.render();
}
run();
setInterval(update, 50);
