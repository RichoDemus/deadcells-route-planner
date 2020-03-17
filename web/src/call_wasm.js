const imp = import("../pkg/index.js");

export const get_biomes = async function load() {
    const {get_biomes} = await imp
        .catch(console.error);

    console.log("also js:", get_biomes());
    return get_biomes();
};
