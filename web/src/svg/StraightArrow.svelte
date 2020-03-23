<script>
    import {calcXForArrow} from "./positionCalculations";

    export let column;
    export let columns;
    export let columnOffset = 0;
    export let row;
    export let length = 1;

    let x = calcXForArrow(parseInt(column), parseInt(columns)) + parseInt(columnOffset);
    let y1 = parseInt(row) * 200 + 100;
    let y2 = y1 + parseInt(length) * 200 - 110;

    let head_x = x - 15;
    let head_y = y2 - 1;
</script>

<line x1="{x}" y1="{y1}" x2="{x}" y2="{y2}"/>
<path class="head" d="M {head_x},{head_y} h 30 l -15 10 z"/>
