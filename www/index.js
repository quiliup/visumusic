import * as wasm from "visumusic";

var dataArray;
var analyser;

function draw() {
    analyser.getFloatFrequencyData(dataArray);
    //console.log("data: " + dataArray);
    setTimeout(draw, 1000);
    update();
}

async function run() {
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
