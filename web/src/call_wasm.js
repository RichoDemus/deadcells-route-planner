const imp = import("../pkg/index.js");

export const from_wasm = async function load() {
    const {hello} = await imp
        .catch(console.error);

    console.log("also js:", hello());
    return hello();
};
