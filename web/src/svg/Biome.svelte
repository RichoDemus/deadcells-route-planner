<script>
    import {calcXForBiome} from "./positionCalculations";

    export let name = "no-name";
    export let y;
    export let x;
    export let rowSize;


    const minX = 0;
    const maxX = 1000 - 200;

    let newX = calcXForBiome(parseInt(x), parseInt(rowSize));
    console.log("name is ", name, " and row size is ", rowSize, " and newX ", newX);

</script>

<style>
    text {
        fill: green;
    }
</style>

<rect x="{newX}" y="{y*200}" width="200" height="100"/>
<text x="{newX+10}" y="{y*200+20}">{name}</text>
