<script>
    import {onMount} from "svelte";
    import {get_biomes} from "./call_wasm";
    import Map from "./svg/Map.svelte";

    let biomes = "";
    let obj = null;

    onMount(async () => {
        biomes = await get_biomes();
    });

</script>

<style>

</style>

<div>
    <Map/>


</div>

<!--<div class="grid-container">-->
<!--	{#each biomes as row, i}-->
<!--		<div class = "biome-tier row">-->
<!--&lt;!&ndash;			<div class="biome-padding"></div>&ndash;&gt;-->
<!--			{#each row as biome, i}-->
<!--				<div class="biome">-->
<!--					{biome}-->
<!--				</div>-->
<!--&lt;!&ndash;				<div class="biome-padding"></div>&ndash;&gt;-->
<!--			{/each}-->
<!--		</div>-->
<!--		<div class="arrows">-->
<!--			arrows-->
<!--		</div>-->
<!--	{/each}-->
<!--</div>-->
