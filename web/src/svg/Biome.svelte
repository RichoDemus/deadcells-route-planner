<script>
    import {calcXForBiome} from "./positionCalculations";
    import {backlistedBiomes} from "../stores";

    export let id;
    export let name = "no-name";
    export let row = "-";
    export let x;
    export let rowSize;
    export let enabled

    let xNew = row * 200 - 200;

    const minX = 0;
    const maxX = 1000 - 200;

    let newX = calcXForBiome(parseInt(x), parseInt(rowSize));
    // console.log("name is ", name, " and row size is ", rowSize, " and newX ", newX);
    // console.log("name is ", name, " row is ", row, " and row size is ", rowSize, " and newX ", newX);

    function click(event) {
        backlistedBiomes.update(biomes => {
            biomes.add(id);
            return biomes;
        });
    }
</script>

<style>
    rect {
        stroke: black;
    }
    text {
        fill: green;
    }
    .disabled {
        opacity: 0.2;
    }
</style>

<rect class:disabled="{ !enabled }" on:click={click} x="{newX}" y="{xNew}" width="200" height="100"/>
<text class:disabled="{ !enabled }" x="{newX+10}" y="{xNew+20}">{name}<!--({row})--></text>
