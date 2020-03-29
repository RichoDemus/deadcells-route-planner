const imp = import("../pkg/index.js");

export const get_biomes = async function load(blacklist) {
    const {get_biomes} = await imp
        .catch(console.error);

    // console.log("call_wasm.get_biomes", blacklist);
    return get_biomes(blacklist);
};
