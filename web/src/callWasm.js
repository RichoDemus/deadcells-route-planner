const imp = import("../pkg/index.js");

export const getBiomes = async function load(blacklist) {
    const {getBiomes} = await imp
        .catch(console.error);

    // console.log("call_wasm.get_biomes", blacklist);
    return getBiomes(blacklist);
};
