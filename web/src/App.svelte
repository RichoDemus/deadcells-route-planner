<script>
	import {onMount} from "svelte";
	import {get_biomes} from "./call_wasm";

	let biomes = "";
	let obj = null;

	onMount( async () => {
		biomes = await get_biomes();
	});

</script>

<style>
	li {
		color: purple;
	}
</style>

<ul>
	{#each biomes as {name}, i}
		<li>{name}</li>
	{/each}
</ul>
