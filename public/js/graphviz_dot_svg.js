import { Graphviz } from "https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js";

const graphviz = await Graphviz.load();

export function graphviz_dot_svg(dot_src) {
    return graphviz.layout(dot_src, "svg", "dot");
}
