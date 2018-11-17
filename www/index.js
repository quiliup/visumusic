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
    //console.log("data: " + dataArray);
    update();
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


var dps = 5000;
var data = [];
var performanceChart = new CanvasJS.Chart("container",
    {
        zoomEnabled: false,
        panEnabled: false,
        legend: {
            horizontalAlign: "right",
            verticalAlign: "center"
        },
        axisY:{
            includeZero: false
        },
        data: data,  // random generator below
    });
function render() {
    var startRender = new Date();
    performanceChart.render();
    var endRender = new Date();
    //jQuery(".render").addClass('disabled');
    //jQuery(".generate").removeClass('disabled');
    //  jQuery(".generate").removeClass('active');
    //jQuery(".renderTime").text((endRender - startRender) + " ms");
}
function update(){
    var dataPoints = [];
    if (analyser === undefined)
        return;
    for (var i = 0; i < analyser.frequencyBinCount; i += 1) {
        dataPoints.push({
            x: i,
            y: dataArray[i]
        });
    }
    performanceChart.options.data = [{type: "line", dataPoints: dataPoints}];
    render();
}
run();
setInterval(update, 50);
