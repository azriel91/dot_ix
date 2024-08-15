import { Graphviz } from "https://cdn.jsdelivr.net/npm/@hpcc-js/wasm@2.18.2/dist/graphviz.js";

const graphviz = await Graphviz.load();

/// # Parameters
///
/// * `dot_src`: The dot source to pass to graphviz dot.
/// * `opts`: The `Options` to pass to graphviz.
///
///     For the WASM build of GraphViz, the `images` key is what allows images to be inlined.
///
///     <https://hpcc-systems.github.io/hpcc-js-wasm/graphviz/interfaces/Options.html>
export function graphviz_dot_svg(dot_src, opts) {
    return graphviz.layout(dot_src, "svg", "dot", opts);
}
