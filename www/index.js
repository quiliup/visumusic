import * as wasm from "visumusic";
import abcjs from "abcjs";

var notes; // string
var note_counter = 0;
var first_note = true;
var this_y_max = 10;
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
            title: "Frequency in Hz",
            logarithmic: true,
            logarithmBase: 2
        },
        axisY: {
            includeZero: false,
            minimum: 0,
            maximum: this_y_max
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

    if (note.endsWith(",,") || note.endsWith("''")) {
      note = "z"
    }

    notes += note;
    note_counter += 1;
    if (note_counter % 4 == 0) {
        notes += "|";
    } else {
        notes += " ";
    }
    if (note_counter % 20 == 0) {
        notes += "\n|";
        var split = notes.split(/\n/)
        if (split.length > 5 + 2) {
            // Remove a line
            notes = split.slice(0, 5).join("\n") + "\n" + split[6] + "\n" + split[7];
        }
    }
    abcjs.renderAbc("notation", notes, { scale: 2.0 , staffwidth: 1000});
}

function update_chart() {
    let update_chart_start = new Date();
    let dataPoints = wasm.get_data(analyser);
    //console.log(dataArray.max);
    var peaks = wasm.get_peaks(analyser);
    this_y_max = this_y_max * 0.97 + peaks.max * 0.03;
    peaks = peaks.peaks;
    for(var obj in peaks) {
        //dataPoints[peaks[obj].index].markerColor = "red";
        //dataPoints[peaks[obj].index].markerType = "triangle";
        dataPoints[peaks[obj].index].indexLabel = dataPoints[peaks[obj].index].x + "Hz";
    }
    // The following line does not work but is needed for functionality
    //console.log(performanceChart.axisY[0].maximum);
    performanceChart.options.data = [{type: "line", dataPoints: dataPoints}];
    performanceChart.options.axisY.maximum = this_y_max * 1.3;
    performanceChart.render();

    let update_chart_end = new Date();
    //console.log("update took " + (update_chart_end-update_chart_start) + "ms");
}

async function run() {
    notes = "X: 1\nT:visumusic\nM:4/4\nL:1/4\nK:treble-8\n|";
    analyser = await wasm.setup();
    console.log("Setup is ready");
    setInterval(update_notes, 1000);
    setInterval(update_chart, 50);
}
run();
