import * as wasm from "visumusic";
import abcjs from "abcjs";

var notes; // string
var note_counter = 0;
var first_note = true;
var last_y_max = 10;
var this_y_max = 10;
var next_y_max = 10;
var analyser;
var performanceChart = new CanvasJS.Chart("container",
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
            includeZero: false,
            minimum: 0,
            maximum: 10
        },
        data: [],
    });

function update_notes() {
    if (first_note) {
        first_note = false;
        return;
    }
    var max_freq = wasm.get_max_frequency(analyser); // array [i,
    var freq = max_freq.freq; // max_freq.value
    var note = wasm.note_for_frequency(freq);

    if (note.endsWith(",,,") || note.endsWith("''")) {
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
    abcjs.renderAbc("notation", notes, { scale: 2.0 , staffwidth: 1000});
}

function update_chart() {
    let update_chart_start = new Date();
    let dataPoints = wasm.get_data(analyser);
    performanceChart.options.data = [{type: "line", dataPoints: dataPoints}];
    //console.log(dataArray.max);
    var peaks = wasm.get_peaks(analyser);
    this_y_max = peaks.max*1.1;
    peaks = peaks.peaks;
    for(var obj in peaks) {
        console.log("object has obj.x:" + obj.x +
                    " obj.y: " + obj.y +
                    " obj.index: " + obj.index);
    }
    // The following line does not work but is needed for functionality
    //performanceChart.axisY[0].maximum = this_y_max;
    //console.log(performanceChart.axisY[0].maximum);
    performanceChart.render();

    let update_chart_end = new Date();
    //console.log("update took " + (update_chart_end-update_chart_start) + "ms");
}

async function run() {
    notes = "X: 1\nT:visumusic\nM:4/4\nL:1/4\nK:none\n|";
    analyser = await wasm.setup();
    console.log("Setup is ready");
    //setInterval(wasm.analyse_audio, 1000, analyser);
    setInterval(update_notes, 1000);
    setInterval(update_chart, 50);
}
run();
