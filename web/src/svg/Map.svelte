<script>
    import Biome from "./Biome.svelte";
    import {onMount} from "svelte";
    import {getBiomes} from "../callWasm";
    import StraightArrow from "./StraightArrow.svelte";
    import SidestepArrow from "./SidestepArrow.svelte";
    import {backlistedBiomes, bossCells} from "../stores";

    let biomes = [];
    let paths = [];
    onMount(async () => {
        await updateBiomes([]);
        // biomes = await get_biomes();
        //
        // paths = biomes.get("paths");
        // biomes.delete("paths");

        // console.log("biomes: ", biomes);
    });

    function clickety(e) {
        console.log(e);
        // e.target.parentElement.classList.add("hidden-arrow");
        // e.target.classList.toggle("hidden-arrow");
        e.target.parentElement.childNodes.forEach(node => node.classList.toggle("hidden-arrow"));
        // e.target.style.fill = "blue";
        // e.target.style.opacity = "0.1";
        console.log(e.target.parentElement)
        // alert("click")
    }

    // todo these are fired on load
    const unsubscribe = backlistedBiomes.subscribe(value => {
        console.log("blacklist updated:", value);
        updateBiomes(value, $bossCells);
    });

    const unsubscribe2 = bossCells.subscribe(value => {
        console.log("bossCells updated:", value);
        updateBiomes($backlistedBiomes, value);
    });

    async function updateBiomes(blacklist, bossCells) {
        console.log("boss cells:", bossCells)
        biomes = await getBiomes(Array.from(blacklist), bossCells);
        // console.log("data from rust: ", biomes);
        paths = biomes.get("paths");
        biomes.delete("paths");
        console.log("paths: ", paths);
    }

    // const biomes = [
    //     ["Prisoners Quarters"],
    //     ["arboretum", "promenade", "sewers"],
    //     ["depths", "corrupted"],
    //     ["Morass", "Ossuary", "Ramparts", "Ancient Sewers"]
    // ];
</script>

<style>
    svg {
        width: 100%;
        height: 100%;
    }
</style>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 950 2350"> <!-- todo tweak this -->
    <!--    <view id="tightcrop" viewBox="4.5 1.5 51 49" />-->
    <defs>
        <g id="arrow-head">
            <path class="head" d="M -15,-10 h 30 l -15 10 z"/>
        </g>
    </defs>

    {#each [...biomes] as [i,tier], ii}
        {#each tier as biome, j}
            <Biome
                    id="{biome.id}"
                    x="{j}"
                    rowSize="{tier.length}"
                    name="{biome.name}"
                    row="{biome.row}"
                    enabled="{biome.enabled}"
                    powerScrolls="{biome.power_scrolls}"
                    dualPowerScrolls="{biome.dual_power_scrolls}"
                    cursedChestChance="{biome.cursed_chest_chance}"
            />
        {/each}
    {/each}

    {#each [...paths] as path, i}
        {#if path.id === "Haven-Throne"}
            <SidestepArrow id="{path.id}"
                           startColumn="{path.startColumn}"
                           startColumns="{path.startColumns}"
                           endColumn="{path.endColumn}"
                           endColumns="{path.endColumns}"
                           row="{path.row}"
                           length="{path.length}"
                           endColumnOffset="120"
                           enabled="{path.enabled}"
            />
            {:else}
            <SidestepArrow id="{path.id}"
                           startColumn="{path.startColumn}"
                           startColumns="{path.startColumns}"
                           endColumn="{path.endColumn}"
                           endColumns="{path.endColumns}"
                           row="{path.row}"
                           length="{path.length}"
                           enabled="{path.enabled}"
            />
            {/if}}

    {/each}



        <!--    Tier one, Prisoners Quarters-->
<!--    <StraightArrow id="pq-potc" column="1" columns="1" row="0"/>-->
<!--&lt;!&ndash;    <SidestepArrow id="pq-da" startColumn="1" startColumns="1" endColumn="1" endColumns="3" row="0"/>&ndash;&gt;-->
<!--    <SidestepArrow id="pq-ts" startColumn="1" startColumns="1" endColumn="3" endColumns="3" row="0"/>-->

<!--    &lt;!&ndash;    Tier two, Arboretum, promenade and sewers&ndash;&gt;-->
<!--    <StraightArrow id="da-motb" column="1" columns="3" row="1" length="2"/>-->
<!--    <SidestepArrow id="da-pd" startColumn="1" startColumns="3" endColumn="1" endColumns="2" row="1"/>-->
<!--    <SidestepArrow id="da-r" startColumn="1" startColumns="3" endColumn="3" endColumns="4" endColumnOffset="-60" row="1" length="2"/>-->
<!--    <SidestepArrow id="potc-dp" startColumn="2" startColumns="3" endColumn="1" endColumns="2" row="1"/>-->
<!--    <StraightArrow id="potc-o" column="2" columns="3" columnOffset="-60" row="1" length="2"/>-->
<!--    <StraightArrow id="potc-r" column="2" columns="3" columnOffset="+30" row="1" length="2"/>-->
<!--    <SidestepArrow id="ts-r" startColumn="3" startColumns="3" endColumn="3" endColumns="4" endColumnOffset="-60" row="1" length="2"/>-->
<!--    <SidestepArrow id="ts-r" startColumn="3" startColumns="3" endColumn="2" endColumns="2" row="1"/>-->
<!--    <StraightArrow id="ts-as" column="3" columns="3" columnOffset="+60" row="1" length="2"/>-->

<!--    &lt;!&ndash;    Optional tier, prison depths and corrupted prison&ndash;&gt;-->
<!--    <SidestepArrow id="pd-motb" startColumn="1" startColumns="2" endColumn="1" endColumns="4" row="2"/>-->
<!--    <SidestepArrow id="pd-o" startColumn="1" startColumns="2" endColumn="2" endColumns="4" row="2"/>-->
<!--    <SidestepArrow id="cp-r" startColumn="2" startColumns="2" endColumn="3" endColumns="4" row="2"/>-->
<!--    <SidestepArrow id="cp-as" startColumn="2" startColumns="2" endColumn="4" endColumns="4" row="2"/>-->

<!--    &lt;!&ndash;    Tier 3, Morass, Ossuary, Ramparts and Ancient Sewers&ndash;&gt;-->
<!--    <SidestepArrow id="motb-tn" startColumn="1" startColumns="4" endColumn="1" endColumns="3" row="3"/>-->
<!--    <SidestepArrow id="o-bb" startColumn="2" startColumns="4" endColumn="2" endColumns="3" row="3"/>-->
<!--    <SidestepArrow id="r-bb" startColumn="3" startColumns="4" endColumn="2" endColumns="3" row="3"/>-->
<!--    <SidestepArrow id="r-ic" startColumn="3" startColumns="4" endColumn="3" endColumns="3" row="3"/>-->
<!--    <SidestepArrow id="as-ic" startColumn="4" startColumns="4" endColumn="3" endColumns="3" row="3"/>-->

<!--&lt;!&ndash;    Tier 4, The Next, Black Bridge, Insuffarable Crypt &ndash;&gt;-->
<!--    <StraightArrow id="tn-sv" column="1" columns="3" row="4"/>-->
<!--    <SidestepArrow id="tn-g" startColumn="1" startColumns="3" endColumn="3" endColumns="3" row="4"/>-->
<!--    <SidestepArrow id="bb-sv" startColumn="2" startColumns="3" endColumn="1" endColumns="3" row="4"/>-->
<!--    <StraightArrow id="bb-ss" column="2" columns="3" row="4"/>-->
<!--    <SidestepArrow id="ic-ss" startColumn="3" startColumns="3" endColumn="2" endColumns="3" row="4"/>-->
<!--    <StraightArrow id="bb-ss" column="3" columns="3" row="4"/>-->

<!--&lt;!&ndash;    Tier 5, Stilt Village, Slumbering Sanctuary and Graveyard&ndash;&gt;-->
<!--    <StraightArrow id="sv-ct" column="1" columns="3" row="5"/>-->
<!--    <SidestepArrow id="sv-fs" startColumn="1" startColumns="3" endColumn="2" endColumns="3" row="5"/>-->
<!--    <SidestepArrow id="ss-ct" startColumn="2" startColumns="3" endColumn="1" endColumns="3" row="5"/>-->
<!--    <StraightArrow id="ss-fs" column="2" columns="3" row="5"/>-->
<!--    <SidestepArrow id="ss-c" startColumn="2" startColumns="3" endColumn="3" endColumns="3" row="5"/>-->
<!--    <SidestepArrow id="g-fs" startColumn="3" startColumns="3" endColumn="2" endColumns="3" row="5"/>-->
<!--    <StraightArrow id="g-c" column="3" columns="3" row="5"/>-->

<!--&lt;!&ndash;    Tier 6, Clock Tower, Forgotten Sepulcher and Cavern&ndash;&gt;-->
<!--    <SidestepArrow id="ct-cr" startColumn="1" startColumns="3" endColumn="1" endColumns="2" row="6"/>-->
<!--    <SidestepArrow id="fs-cr" startColumn="2" startColumns="3" endColumn="1" endColumns="2" row="6"/>-->
<!--    <SidestepArrow id="fs-gh" startColumn="2" startColumns="3" endColumn="2" endColumns="2" row="6"/>-->
<!--    <SidestepArrow id="c-gh" startColumn="3" startColumns="3" endColumn="2" endColumns="2" row="6"/>-->

<!--&lt;!&ndash;    Tier 7, Clock Room and Guardians Haven&ndash;&gt;-->
<!--    <SidestepArrow id="cr-hpc" startColumn="1" startColumns="2" endColumn="1" endColumns="1" row="7"/>-->
<!--    <SidestepArrow id="gh-hpc" startColumn="2" startColumns="2" endColumn="1" endColumns="1" row="7"/>-->
<!--    <SidestepArrow id="gh-tr" startColumn="2" startColumns="2" endColumn="1" endColumns="1" endColumnOffset="110" row="7" length="2"/>-->

<!--&lt;!&ndash;    last few single-biome tiers &ndash;&gt;-->
<!--    <StraightArrow id="hpk-tr" column="1" columns="1" row="8"/>-->
<!--    <StraightArrow id="tr-a" column="1" columns="1" row="9"/>-->
<!--    <StraightArrow id="a-o" column="1" columns="1" row="10"/>-->


</svg>
