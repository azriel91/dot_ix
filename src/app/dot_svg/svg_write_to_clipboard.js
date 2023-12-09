const svg_src = document.getElementById("svg_div").innerHTML;
const svg_first_chunk = svg_src.slice(0, 384);

// Inject stylesheet if the SVG does not already include it.
let svg_with_styles = "";
if (svg_first_chunk.includes("<style>")) {
    svg_with_styles = svg_src;
} else {
    const tw_stylesheet = [...document.styleSheets].find(ss => ss.href === null && ss.rules.length > 80);
    const tw_svg_styles = [...tw_stylesheet.cssRules].map((rule) => rule.cssText).join("").replace(/&/g, '&amp;');
    svg_with_styles = svg_src.replace("<g ",
`<style>
${tw_svg_styles}
</style>
<g `);
}

navigator.clipboard.writeText(svg_with_styles).then(
    () => { /* clipboard successfully set */ },
    e => console.error(`Clipboard write failed. Error: ${e}`),
);
