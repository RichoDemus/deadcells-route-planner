<script>
    import {calcXForArrow} from "./positionCalculations";

    // export let id;
    export let startColumn;
    export let startColumns;
    export let endColumn;
    export let endColumns;
    export let endColumnOffset = 0;
    export let row;
    export let length = 1;
    export let enabled;

    let x1 = calcXForArrow(parseInt(startColumn), parseInt(startColumns));
    let x2 = calcXForArrow(parseInt(endColumn), parseInt(endColumns)) + parseInt(endColumnOffset);

    let y1 = parseInt(row) * 200 - 100;
    let middleLineY =  y1 + 40;
    let y2 = y1 + parseInt(length) * 200 - 110;

    let head_x = x2 - 15;
    let head_y = y2 - 1;

    let classes = "";
    // enabled = true;
    // console.log("arrow:",id, "start-column",startColumn, "/",startColumns, "end-column",endColumn, "/",endColumns,"row", row, "length",length)
   //  console.log("enabled:", enabled, "type", typeof enabled);
</script>

<style>
    line {
        stroke-width: 10;
        stroke-linecap: butt;
        stroke: black;
        /*opacity: 0.3;*/
    }
    path {
        stroke: black;
    }
    .disabled {
        opacity: 0.1;
    }
</style>

<line class:disabled="{ !enabled }" x1="{x1}" y1="{y1}" x2="{x1}" y2="{middleLineY + 5}"/>
<line class:disabled="{ !enabled }" x1="{x1}" y1="{middleLineY}" x2="{x2}" y2="{middleLineY}"/>
<line class:disabled="{ !enabled }" x1="{x2}" y1="{middleLineY - 5}" x2="{x2}" y2="{y2}"/>
<path class:disabled="{ !enabled }" d="M {head_x},{head_y} h 30 l -15 10 z"/>
