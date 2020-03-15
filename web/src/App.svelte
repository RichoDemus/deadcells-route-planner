<script>
	import {onMount} from "svelte";
	import {from_wasm} from "./call_wasm";

	export let name;
	let fromWasm = "";
	let obj = null;

	onMount( async () => {
		fromWasm = await from_wasm();
	});

</script>

<style>
	h1 {
		color: purple;
	}
</style>

<h1>Hello {name}!</h1>
<span>from was: {fromWasm}</span> <br/>
