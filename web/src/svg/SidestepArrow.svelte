<script>
    import {calcXForArrow} from "./positionCalculations";

    export let startColumn;
    export let startColumns;
    export let endColumn;
    export let endColumns;
    export let endColumnOffset = 0;
    export let row;
    export let length = 1;

    let x1 = calcXForArrow(parseInt(startColumn), parseInt(startColumns));
    let x2 = calcXForArrow(parseInt(endColumn), parseInt(endColumns)) + parseInt(endColumnOffset);

    let y1 = parseInt(row) * 200 - 100;
    let middleLineY =  y1 + 40;
    let y2 = y1 + parseInt(length) * 200 - 110;

    let head_x = x2 - 15;
    let head_y = y2 - 1;
    // console.log("arrow:",startColumn, startColumns, endColumn, endColumns, row, length)
</script>

<line x1="{x1}" y1="{y1}" x2="{x1}" y2="{middleLineY + 5}"/>
<line x1="{x1}" y1="{middleLineY}" x2="{x2}" y2="{middleLineY}"/>
<line x1="{x2}" y1="{middleLineY - 5}" x2="{x2}" y2="{y2}"/>
<path class="head" d="M {head_x},{head_y} h 30 l -15 10 z"/>
