<script>
    import {calcXForArrow} from "./positionCalculations";

    // export let id;
    export let startColumn;
    export let startColumns;
    export let startColumnOffset = 0;
    export let endColumn;
    export let endColumns;
    export let endColumnOffset = 0;
    export let row;
    export let length = 1;
    export let enabled;

    let x1 = calcXForArrow(parseInt(startColumn), parseInt(startColumns))  + parseInt(startColumnOffset);
    let x2 = calcXForArrow(parseInt(endColumn), parseInt(endColumns))  + parseInt(endColumnOffset);

    let y1 = parseInt(row) * 200 - 100;
    let middleLineY =  y1 + 40;
    let y2 = y1 + parseInt(length) * 200 - 110;

    let head_x = x2 - 15;
    let head_y = y2 - 1;

    let classes = "";
    // enabled = true;
    // console.log("arrow:",id, "start-column",startColumn, "/",startColumns, "end-column",endColumn, "/",endColumns,"row", row, "length",length)
   //  console.log("enabled:", enabled, "type", typeof enabled);

    // todo pick and remove color to avoid duplicates
    const array = ["powderblue",
        "lightblue",
        "lightskyblue",
        "skyblue",
        "deepskyblue",
        "lightsteelblue",
        "dodgerblue",
        "cornflowerblue",
        "steelblue",
        "royalblue",
        "blue",
        "mediumblue",
        "darkblue",
        "navy",
        "midnightblue",
        "mediumslateblue",
        "slateblue",
        "darkslateblue"];
    // let color = "black";
    const color = array[Math.floor(Math.random() * array.length)];
</script>

<style>
    line {
        stroke-width: 10;
        stroke-linecap: butt;
        stroke: var(--theme-color);
        /*opacity: 0.3;*/
        /*fill: none;*/
        /*stroke: #646464;*/
        /*stroke-width: 1px;*/
        /*stroke-dasharray: 2,2;*/
        /*stroke-linejoin: round;*/
        /*outline: 3px solid green;*/
    }
    path {
        stroke: var(--theme-color);
        fill: var(--theme-color);
    }
    .disabled {
        opacity: 0.1;
    }
</style>

<line class:disabled="{ !enabled }" style="--theme-color: {color}" x1="{x1}" y1="{y1}" x2="{x1}" y2="{middleLineY + 5}"/>
<line class:disabled="{ !enabled }" style="--theme-color: {color}" x1="{x1}" y1="{middleLineY}" x2="{x2}" y2="{middleLineY}"/>
<line class:disabled="{ !enabled }" style="--theme-color: {color}" x1="{x2}" y1="{middleLineY - 5}" x2="{x2}" y2="{y2}"/>
<path class:disabled="{ !enabled }" style="--theme-color: {color}" d="M {head_x},{head_y} h 30 l -15 10 z"/>
