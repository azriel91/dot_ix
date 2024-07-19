const svg_src = document.getElementById("svg_div").innerHTML;
const svg_first_chunk = svg_src.slice(0, 512);

// Inject stylesheet if the SVG does not already include it.
let svg_with_styles = "";
if (svg_first_chunk.includes("/* TW_PLACEHOLDER */")) {
    const font_inline = `@font-face {
  font-family: 'liberationmono';
  src: url(data:application/x-font-woff;charset=utf-8;base64,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) format('woff');
}`;

    const tw_stylesheet = [...document.styleSheets]
        .find(ss => ss.href === null
            && ss.cssRules.length > 80
            && [...ss.cssRules].every(rule => !rule.cssText.includes('codico')) // monaco editor styles
        );
    const tw_svg_styles = [...tw_stylesheet.cssRules].map((rule) => rule.cssText).join("").replace(/&/g, '&amp;');
    svg_with_styles = svg_src.replace("/* TW_PLACEHOLDER */", `${font_inline}\n${tw_svg_styles}`);
} else {
    svg_with_styles = svg_src;
}

svg_with_styles = svg_with_styles
    .replaceAll("&nbsp;", "&#160;");

navigator.clipboard.writeText(svg_with_styles).then(
    () => { /* clipboard successfully set */ },
    e => console.error(`Clipboard write failed. Error: ${e}`),
);
