// number of dataPoints by default. 
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
function init() {
    //  jQuery(".render").removeClass('active');
    jQuery(".generate").addClass('disabled');
    jQuery(".render").removeClass('disabled');
    jQuery(".renderTime").text('');
}
init();
function render() {
    var startRender = new Date();
    performanceChart.render();
    var endRender = new Date();
    jQuery(".render").addClass('disabled');
    jQuery(".generate").removeClass('disabled');
    //  jQuery(".generate").removeClass('active');
    jQuery(".renderTime").text((endRender - startRender) + " ms");
}
function generate() {
    var startGen = new Date();
    var limit = dps;
    var y = 0;
    var dataSeries = { type: "line" };
    var dataPoints = [];
    for (var i = 0; i < limit; i += 1) {
        y += (Math.random() * 10 - 5);
        dataPoints.push({
            x: i,
            y: y
        });
    }
    dataSeries.dataPoints = dataPoints;
    performanceChart.options.data = [dataSeries];
    var endGen = new Date();
    jQuery(".genTime").text((endGen - startGen) + " ms |");
    jQuery(".render").click(render);
}
jQuery("#dps").change(function() {
    dps = (jQuery(this).val());
});
jQuery(".generate").click(generate);
function update(){
    //jQuery(".generate").click();
    generate();
    //jQuery(".render").click();
    render();
}
setInterval(update, 50);
/*
jQuery(document).ready(function(){
setTimeout(function(){
    jQuery(".generate").click();
    jQuery(".render").click();
}, 1000);
});
*/
