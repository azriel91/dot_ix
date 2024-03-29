use serde::{Deserialize, Serialize};

/// Themeable attributes of nodes and edges.
///
/// These keys are used by both nodes and edges.
///
/// # Notes
///
/// * `Extra` are classes that are copied as-is onto the relevant SVG/HTML
///   elements.
/// * `ShapeColor` controls the border and fill colours for nodes, and the line
///   and arrow head colours for edges.
/// * `Stroke*` keys control the border colour for nodes, and the line colour
///   for edges.
/// * `Fill*` keys control the background colour for nodes, and the arrow head
///   fill colour for edges.
/// * `Padding*` keys are only applicable to nodes.
/// * `Margin*` keys are only applicable to nodes.
///
/// # Colours
///
/// [Colours] are from [Tailwind CSS]. The list of names are:
///
/// * <span class="color-label">key:</span>
/// <span class="blk">&nbsp;50</span>
/// <span class="blk">100</span>
/// <span class="blk">200</span>
/// <span class="blk">300</span>
/// <span class="blk">400</span>
/// <span class="blk">500</span>
/// <span class="blk">600</span>
/// <span class="blk">700</span>
/// <span class="blk">800</span>
/// <span class="blk">900</span>
/// <span class="blk">950</span>
/// * <span class="color-label">`slate`:</span>
/// <span class="colorblk" style="background-color: #f8fafc;"></span>
/// <span class="colorblk" style="background-color: #f1f5f9;"></span>
/// <span class="colorblk" style="background-color: #e2e8f0;"></span>
/// <span class="colorblk" style="background-color: #cbd5e1;"></span>
/// <span class="colorblk" style="background-color: #94a3b8;"></span>
/// <span class="colorblk" style="background-color: #64748b;"></span>
/// <span class="colorblk" style="background-color: #475569;"></span>
/// <span class="colorblk" style="background-color: #334155;"></span>
/// <span class="colorblk" style="background-color: #1e293b;"></span>
/// <span class="colorblk" style="background-color: #0f172a;"></span>
/// <span class="colorblk" style="background-color: #020617;"></span>
/// * <span class="color-label">`gray`:</span>
/// <span class="colorblk" style="background-color: #f9fafb;"></span>
/// <span class="colorblk" style="background-color: #f3f4f6;"></span>
/// <span class="colorblk" style="background-color: #e5e7eb;"></span>
/// <span class="colorblk" style="background-color: #d1d5db;"></span>
/// <span class="colorblk" style="background-color: #9ca3af;"></span>
/// <span class="colorblk" style="background-color: #6b7280;"></span>
/// <span class="colorblk" style="background-color: #4b5563;"></span>
/// <span class="colorblk" style="background-color: #374151;"></span>
/// <span class="colorblk" style="background-color: #1f2937;"></span>
/// <span class="colorblk" style="background-color: #111827;"></span>
/// <span class="colorblk" style="background-color: #030712;"></span>
/// * <span class="color-label">`zinc`:</span>
/// <span class="colorblk" style="background-color: #fafafa;"></span>
/// <span class="colorblk" style="background-color: #f4f4f5;"></span>
/// <span class="colorblk" style="background-color: #e4e4e7;"></span>
/// <span class="colorblk" style="background-color: #d4d4d8;"></span>
/// <span class="colorblk" style="background-color: #a1a1aa;"></span>
/// <span class="colorblk" style="background-color: #71717a;"></span>
/// <span class="colorblk" style="background-color: #52525b;"></span>
/// <span class="colorblk" style="background-color: #3f3f46;"></span>
/// <span class="colorblk" style="background-color: #27272a;"></span>
/// <span class="colorblk" style="background-color: #18181b;"></span>
/// <span class="colorblk" style="background-color: #09090b;"></span>
/// * <span class="color-label">`neutral`:</span>
/// <span class="colorblk" style="background-color: #fafafa;"></span>
/// <span class="colorblk" style="background-color: #f5f5f5;"></span>
/// <span class="colorblk" style="background-color: #e5e5e5;"></span>
/// <span class="colorblk" style="background-color: #d4d4d4;"></span>
/// <span class="colorblk" style="background-color: #a3a3a3;"></span>
/// <span class="colorblk" style="background-color: #737373;"></span>
/// <span class="colorblk" style="background-color: #525252;"></span>
/// <span class="colorblk" style="background-color: #404040;"></span>
/// <span class="colorblk" style="background-color: #262626;"></span>
/// <span class="colorblk" style="background-color: #171717;"></span>
/// <span class="colorblk" style="background-color: #0a0a0a;"></span>
/// * <span class="color-label">`stone`:</span>
/// <span class="colorblk" style="background-color: #fafaf9;"></span>
/// <span class="colorblk" style="background-color: #f5f5f4;"></span>
/// <span class="colorblk" style="background-color: #e7e5e4;"></span>
/// <span class="colorblk" style="background-color: #d6d3d1;"></span>
/// <span class="colorblk" style="background-color: #a8a29e;"></span>
/// <span class="colorblk" style="background-color: #78716c;"></span>
/// <span class="colorblk" style="background-color: #57534e;"></span>
/// <span class="colorblk" style="background-color: #44403c;"></span>
/// <span class="colorblk" style="background-color: #292524;"></span>
/// <span class="colorblk" style="background-color: #1c1917;"></span>
/// <span class="colorblk" style="background-color: #0c0a09;"></span>
/// * <span class="color-label">`red`:</span>
/// <span class="colorblk" style="background-color: #fef2f2;"></span>
/// <span class="colorblk" style="background-color: #fee2e2;"></span>
/// <span class="colorblk" style="background-color: #fecaca;"></span>
/// <span class="colorblk" style="background-color: #fca5a5;"></span>
/// <span class="colorblk" style="background-color: #f87171;"></span>
/// <span class="colorblk" style="background-color: #ef4444;"></span>
/// <span class="colorblk" style="background-color: #dc2626;"></span>
/// <span class="colorblk" style="background-color: #b91c1c;"></span>
/// <span class="colorblk" style="background-color: #991b1b;"></span>
/// <span class="colorblk" style="background-color: #7f1d1d;"></span>
/// <span class="colorblk" style="background-color: #450a0a;"></span>
/// * <span class="color-label">`orange`:</span>
/// <span class="colorblk" style="background-color: #fff7ed;"></span>
/// <span class="colorblk" style="background-color: #ffedd5;"></span>
/// <span class="colorblk" style="background-color: #fed7aa;"></span>
/// <span class="colorblk" style="background-color: #fdba74;"></span>
/// <span class="colorblk" style="background-color: #fb923c;"></span>
/// <span class="colorblk" style="background-color: #f97316;"></span>
/// <span class="colorblk" style="background-color: #ea580c;"></span>
/// <span class="colorblk" style="background-color: #c2410c;"></span>
/// <span class="colorblk" style="background-color: #9a3412;"></span>
/// <span class="colorblk" style="background-color: #7c2d12;"></span>
/// <span class="colorblk" style="background-color: #431407;"></span>
/// * <span class="color-label">`slate`:</span>
/// <span class="colorblk" style="background-color: #fffbeb;"></span>
/// <span class="colorblk" style="background-color: #fef3c7;"></span>
/// <span class="colorblk" style="background-color: #fde68a;"></span>
/// <span class="colorblk" style="background-color: #fcd34d;"></span>
/// <span class="colorblk" style="background-color: #fbbf24;"></span>
/// <span class="colorblk" style="background-color: #f59e0b;"></span>
/// <span class="colorblk" style="background-color: #d97706;"></span>
/// <span class="colorblk" style="background-color: #b45309;"></span>
/// <span class="colorblk" style="background-color: #92400e;"></span>
/// <span class="colorblk" style="background-color: #78350f;"></span>
/// <span class="colorblk" style="background-color: #451a03;"></span>
/// * <span class="color-label">`yellow`:</span>
/// <span class="colorblk" style="background-color: #fefce8;"></span>
/// <span class="colorblk" style="background-color: #fef9c3;"></span>
/// <span class="colorblk" style="background-color: #fef08a;"></span>
/// <span class="colorblk" style="background-color: #fde047;"></span>
/// <span class="colorblk" style="background-color: #facc15;"></span>
/// <span class="colorblk" style="background-color: #eab308;"></span>
/// <span class="colorblk" style="background-color: #ca8a04;"></span>
/// <span class="colorblk" style="background-color: #a16207;"></span>
/// <span class="colorblk" style="background-color: #854d0e;"></span>
/// <span class="colorblk" style="background-color: #713f12;"></span>
/// <span class="colorblk" style="background-color: #422006;"></span>
/// * <span class="color-label">`lime`:</span>
/// <span class="colorblk" style="background-color: #f7fee7;"></span>
/// <span class="colorblk" style="background-color: #ecfccb;"></span>
/// <span class="colorblk" style="background-color: #d9f99d;"></span>
/// <span class="colorblk" style="background-color: #bef264;"></span>
/// <span class="colorblk" style="background-color: #a3e635;"></span>
/// <span class="colorblk" style="background-color: #84cc16;"></span>
/// <span class="colorblk" style="background-color: #65a30d;"></span>
/// <span class="colorblk" style="background-color: #4d7c0f;"></span>
/// <span class="colorblk" style="background-color: #3f6212;"></span>
/// <span class="colorblk" style="background-color: #365314;"></span>
/// <span class="colorblk" style="background-color: #1a2e05;"></span>
/// * <span class="color-label">`green`:</span>
/// <span class="colorblk" style="background-color: #f0fdf4;"></span>
/// <span class="colorblk" style="background-color: #dcfce7;"></span>
/// <span class="colorblk" style="background-color: #bbf7d0;"></span>
/// <span class="colorblk" style="background-color: #86efac;"></span>
/// <span class="colorblk" style="background-color: #4ade80;"></span>
/// <span class="colorblk" style="background-color: #22c55e;"></span>
/// <span class="colorblk" style="background-color: #16a34a;"></span>
/// <span class="colorblk" style="background-color: #15803d;"></span>
/// <span class="colorblk" style="background-color: #166534;"></span>
/// <span class="colorblk" style="background-color: #14532d;"></span>
/// <span class="colorblk" style="background-color: #052e16;"></span>
/// * <span class="color-label">`emerald`:</span>
/// <span class="colorblk" style="background-color: #ecfdf5;"></span>
/// <span class="colorblk" style="background-color: #d1fae5;"></span>
/// <span class="colorblk" style="background-color: #a7f3d0;"></span>
/// <span class="colorblk" style="background-color: #6ee7b7;"></span>
/// <span class="colorblk" style="background-color: #34d399;"></span>
/// <span class="colorblk" style="background-color: #10b981;"></span>
/// <span class="colorblk" style="background-color: #059669;"></span>
/// <span class="colorblk" style="background-color: #047857;"></span>
/// <span class="colorblk" style="background-color: #065f46;"></span>
/// <span class="colorblk" style="background-color: #064e3b;"></span>
/// <span class="colorblk" style="background-color: #022c22;"></span>
/// * <span class="color-label">`teal`:</span>
/// <span class="colorblk" style="background-color: #f0fdfa;"></span>
/// <span class="colorblk" style="background-color: #ccfbf1;"></span>
/// <span class="colorblk" style="background-color: #99f6e4;"></span>
/// <span class="colorblk" style="background-color: #5eead4;"></span>
/// <span class="colorblk" style="background-color: #2dd4bf;"></span>
/// <span class="colorblk" style="background-color: #14b8a6;"></span>
/// <span class="colorblk" style="background-color: #0d9488;"></span>
/// <span class="colorblk" style="background-color: #0f766e;"></span>
/// <span class="colorblk" style="background-color: #115e59;"></span>
/// <span class="colorblk" style="background-color: #134e4a;"></span>
/// <span class="colorblk" style="background-color: #042f2e;"></span>
/// * <span class="color-label">`cyan`:</span>
/// <span class="colorblk" style="background-color: #ecfeff;"></span>
/// <span class="colorblk" style="background-color: #cffafe;"></span>
/// <span class="colorblk" style="background-color: #a5f3fc;"></span>
/// <span class="colorblk" style="background-color: #67e8f9;"></span>
/// <span class="colorblk" style="background-color: #22d3ee;"></span>
/// <span class="colorblk" style="background-color: #06b6d4;"></span>
/// <span class="colorblk" style="background-color: #0891b2;"></span>
/// <span class="colorblk" style="background-color: #0e7490;"></span>
/// <span class="colorblk" style="background-color: #155e75;"></span>
/// <span class="colorblk" style="background-color: #164e63;"></span>
/// <span class="colorblk" style="background-color: #083344;"></span>
/// * <span class="color-label">`sky`:</span>
/// <span class="colorblk" style="background-color: #f0f9ff;"></span>
/// <span class="colorblk" style="background-color: #e0f2fe;"></span>
/// <span class="colorblk" style="background-color: #bae6fd;"></span>
/// <span class="colorblk" style="background-color: #7dd3fc;"></span>
/// <span class="colorblk" style="background-color: #38bdf8;"></span>
/// <span class="colorblk" style="background-color: #0ea5e9;"></span>
/// <span class="colorblk" style="background-color: #0284c7;"></span>
/// <span class="colorblk" style="background-color: #0369a1;"></span>
/// <span class="colorblk" style="background-color: #075985;"></span>
/// <span class="colorblk" style="background-color: #0c4a6e;"></span>
/// <span class="colorblk" style="background-color: #082f49;"></span>
/// * <span class="color-label">`blue`:</span>
/// <span class="colorblk" style="background-color: #eff6ff;"></span>
/// <span class="colorblk" style="background-color: #dbeafe;"></span>
/// <span class="colorblk" style="background-color: #bfdbfe;"></span>
/// <span class="colorblk" style="background-color: #93c5fd;"></span>
/// <span class="colorblk" style="background-color: #60a5fa;"></span>
/// <span class="colorblk" style="background-color: #3b82f6;"></span>
/// <span class="colorblk" style="background-color: #2563eb;"></span>
/// <span class="colorblk" style="background-color: #1d4ed8;"></span>
/// <span class="colorblk" style="background-color: #1e40af;"></span>
/// <span class="colorblk" style="background-color: #1e3a8a;"></span>
/// <span class="colorblk" style="background-color: #172554;"></span>
/// * <span class="color-label">`indigo`:</span>
/// <span class="colorblk" style="background-color: #eef2ff;"></span>
/// <span class="colorblk" style="background-color: #e0e7ff;"></span>
/// <span class="colorblk" style="background-color: #c7d2fe;"></span>
/// <span class="colorblk" style="background-color: #a5b4fc;"></span>
/// <span class="colorblk" style="background-color: #818cf8;"></span>
/// <span class="colorblk" style="background-color: #6366f1;"></span>
/// <span class="colorblk" style="background-color: #4f46e5;"></span>
/// <span class="colorblk" style="background-color: #4338ca;"></span>
/// <span class="colorblk" style="background-color: #3730a3;"></span>
/// <span class="colorblk" style="background-color: #312e81;"></span>
/// <span class="colorblk" style="background-color: #1e1b4b;"></span>
/// * <span class="color-label">`violet`:</span>
/// <span class="colorblk" style="background-color: #f5f3ff;"></span>
/// <span class="colorblk" style="background-color: #ede9fe;"></span>
/// <span class="colorblk" style="background-color: #ddd6fe;"></span>
/// <span class="colorblk" style="background-color: #c4b5fd;"></span>
/// <span class="colorblk" style="background-color: #a78bfa;"></span>
/// <span class="colorblk" style="background-color: #8b5cf6;"></span>
/// <span class="colorblk" style="background-color: #7c3aed;"></span>
/// <span class="colorblk" style="background-color: #6d28d9;"></span>
/// <span class="colorblk" style="background-color: #5b21b6;"></span>
/// <span class="colorblk" style="background-color: #4c1d95;"></span>
/// <span class="colorblk" style="background-color: #2e1065;"></span>
/// * <span class="color-label">`purple`:</span>
/// <span class="colorblk" style="background-color: #faf5ff;"></span>
/// <span class="colorblk" style="background-color: #f3e8ff;"></span>
/// <span class="colorblk" style="background-color: #e9d5ff;"></span>
/// <span class="colorblk" style="background-color: #d8b4fe;"></span>
/// <span class="colorblk" style="background-color: #c084fc;"></span>
/// <span class="colorblk" style="background-color: #a855f7;"></span>
/// <span class="colorblk" style="background-color: #9333ea;"></span>
/// <span class="colorblk" style="background-color: #7e22ce;"></span>
/// <span class="colorblk" style="background-color: #6b21a8;"></span>
/// <span class="colorblk" style="background-color: #581c87;"></span>
/// <span class="colorblk" style="background-color: #3b0764;"></span>
/// * <span class="color-label">`fuchsia`:</span>
/// <span class="colorblk" style="background-color: #fdf4ff;"></span>
/// <span class="colorblk" style="background-color: #fae8ff;"></span>
/// <span class="colorblk" style="background-color: #f5d0fe;"></span>
/// <span class="colorblk" style="background-color: #f0abfc;"></span>
/// <span class="colorblk" style="background-color: #e879f9;"></span>
/// <span class="colorblk" style="background-color: #d946ef;"></span>
/// <span class="colorblk" style="background-color: #c026d3;"></span>
/// <span class="colorblk" style="background-color: #a21caf;"></span>
/// <span class="colorblk" style="background-color: #86198f;"></span>
/// <span class="colorblk" style="background-color: #701a75;"></span>
/// <span class="colorblk" style="background-color: #4a044e;"></span>
/// * <span class="color-label">`pink`:</span>
/// <span class="colorblk" style="background-color: #fdf2f8;"></span>
/// <span class="colorblk" style="background-color: #fce7f3;"></span>
/// <span class="colorblk" style="background-color: #fbcfe8;"></span>
/// <span class="colorblk" style="background-color: #f9a8d4;"></span>
/// <span class="colorblk" style="background-color: #f472b6;"></span>
/// <span class="colorblk" style="background-color: #ec4899;"></span>
/// <span class="colorblk" style="background-color: #db2777;"></span>
/// <span class="colorblk" style="background-color: #be185d;"></span>
/// <span class="colorblk" style="background-color: #9d174d;"></span>
/// <span class="colorblk" style="background-color: #831843;"></span>
/// <span class="colorblk" style="background-color: #500724;"></span>
/// * <span class="color-label">`rose`:</span>
/// <span class="colorblk" style="background-color: #fff1f2;"></span>
/// <span class="colorblk" style="background-color: #ffe4e6;"></span>
/// <span class="colorblk" style="background-color: #fecdd3;"></span>
/// <span class="colorblk" style="background-color: #fda4af;"></span>
/// <span class="colorblk" style="background-color: #fb7185;"></span>
/// <span class="colorblk" style="background-color: #f43f5e;"></span>
/// <span class="colorblk" style="background-color: #e11d48;"></span>
/// <span class="colorblk" style="background-color: #be123c;"></span>
/// <span class="colorblk" style="background-color: #9f1239;"></span>
/// <span class="colorblk" style="background-color: #881337;"></span>
/// <span class="colorblk" style="background-color: #4c0519;"></span>
///
/// <style>
/// .color-label { display: inline-block; width: 80px; }
/// .blk { font-family: monospace; }
/// .colorblk:after { content: '   '; }
/// .colorblk {
///     white-space: pre;
///     border-radius: 3px;
///     font-family: monospace;
/// }
/// </style>
///
/// [Colours]: https://tailwindcss.com/docs/customizing-colors
/// [Tailwind CSS]: https://tailwindcss.com/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeAttr {
    /// Extra classes to attach as is.
    Extra,
    /// Colour for element background/arrow head for all states, e.g. `"slate"`.
    ///
    /// This will be used for all [`HighlightState`]s if not overridden by one
    /// of the more specific variants.
    ///
    /// [`HighlightState`]: crate::theme::HighlightState
    FillColor,
    /// Colour for the background when an element is not focused / hovered
    /// over, e.g. `"slate"`.
    FillColorNormal,
    /// Colour for the background when an element is focused, e.g. `"slate"`.
    FillColorFocus,
    /// Colour for the background when an element has the cursor hovering over
    /// it, e.g. `"slate"`.
    FillColorHover,
    /// Colour for the background when an element is being clicked / pressed,
    /// e.g. `"slate"`.
    FillColorActive,
    /// Shade for element background/arrow head for all states, e.g. `"600"`.
    ///
    /// This will be used for all [`HighlightState`]s if not overridden by one
    /// of the more specific variants.
    ///
    /// [`HighlightState`]: crate::theme::HighlightState
    FillShade,
    /// Shade for the background when an element is not focused /
    /// hovered over, e.g. `"300"` for nodes, "800" for edges.
    FillShadeNormal,
    /// Shade for the background when an element is focused, e.g.
    /// `"200"` for nodes, "700" for edges.
    FillShadeFocus,
    /// Shade for the background when an element has the cursor
    /// hovering over it, e.g. `"100"` for nodes, "600" for edges.
    FillShadeHover,
    /// Shade for the background when an element is being clicked /
    /// pressed, e.g. `"200"` for nodes, "700" for edges.
    FillShadeActive,
    /// All padding within a node, e.g. `"1.5"` in `"p-1.5"`.
    ///
    /// This key has no effect on edges.
    Padding,
    /// Left and right padding within a node, e.g. `"1.5"` in `"px-1.5"`.
    ///
    /// This key has no effect on edges.
    PaddingX,
    /// Top and bottom padding within a node, e.g. `"1.5"` in `"py-1.5"`.
    ///
    /// This key has no effect on edges.
    PaddingY,
    /// All margins around a node, e.g. `"1.5"` in `"m-1.5"`.
    ///
    /// This key has no effect on edges.
    Margin,
    /// Left and right margins around a node, e.g. `"1.5"` in `"mx-1.5"`.
    ///
    /// This key has no effect on edges.
    MarginX,
    /// Top and bottom margins around a node, e.g. `"1.5"` in `"my-1.5"`.
    ///
    /// This key has no effect on edges.
    MarginY,
    /// Outline colour for elements for all states, e.g. `"blue"`. Defaults to
    /// `"blue"`.
    OutlineColor,
    /// Outline colour when an element is not focused / hovered over, e.g.
    /// `"blue"`. Defaults to nothing.
    OutlineColorNormal,
    /// Outline colour when an element is focused, e.g. `"blue"`. Defaults to
    /// nothing.
    OutlineColorFocus,
    /// Outline colour when an element has the cursor hovering over it, e.g.
    /// `"blue"`. Defaults to nothing.
    OutlineColorHover,
    /// Outline colour when an element is being clicked / pressed, e.g.
    /// `"blue"`. Defaults to nothing.
    OutlineColorActive,
    /// Shade for lines and borders for all states, e.g. `"600"`.
    ///
    /// This will be used for all [`HighlightState`]s if not overridden by one
    /// of the more specific variants.
    ///
    /// [`HighlightState`]: crate::theme::HighlightState
    OutlineShade,
    /// Outline shade when an element is not focused / hovered over, e.g.
    /// `"600"` for nodes, "900" for edges. Defaults to nothing.
    OutlineShadeNormal,
    /// Outline shade when an element is focused, e.g.
    /// `"500"` for nodes, "800" for edges. Defaults to the aforementioned
    /// values.
    OutlineShadeFocus,
    /// Outline shade when an element has the cursor hovering over it, e.g.
    /// `"400"` for nodes, "700" for edges. Defaults to nothing.
    OutlineShadeHover,
    /// Outline shade when an element is being clicked / pressed, e.g.
    /// `"700"` for nodes, "950" for edges. Defaults to nothing.
    OutlineShadeActive,
    /// Width of the border for nodes, or line for edges. Defaults to `"2"`.
    OutlineWidth,
    /// Outline style for all states, e.g. "none", "solid", "dashed", "dotted".
    OutlineStyle,
    /// Outline style when an element is not focused / hovered over, e.g.
    /// "none", "solid", "dashed", "dotted".
    OutlineStyleNormal,
    /// Outline style when an element is focused, e.g. "none", "solid",
    /// "dashed", "dotted".
    OutlineStyleFocus,
    /// Outline style when an element has the cursor hovering over it, e.g.
    /// "none", "solid", "dashed", "dotted".
    OutlineStyleHover,
    /// Outline style when an element is being clicked / pressed, e.g. "none",
    /// "solid", "dashed", "dotted".
    OutlineStyleActive,
    /// Base colour for shape colourable attributes, e.g. stroke/border, fill.
    ShapeColor,
    /// Line/border colour for elements for all states, e.g. `"slate"`.
    StrokeColor,
    /// Line/border colour when an element is not focused / hovered over, e.g.
    /// `"slate"`.
    StrokeColorNormal,
    /// Line/border colour when an element is focused, e.g. `"slate"`.
    StrokeColorFocus,
    /// Line/border colour when an element has the cursor hovering over it, e.g.
    /// `"slate"`.
    StrokeColorHover,
    /// Line/border colour when an element is being clicked / pressed, e.g.
    /// `"slate"`.
    StrokeColorActive,
    /// Shade for lines and borders for all states, e.g. `"600"`.
    ///
    /// This will be used for all [`HighlightState`]s if not overridden by one
    /// of the more specific variants.
    ///
    /// [`HighlightState`]: crate::theme::HighlightState
    StrokeShade,
    /// Line/border shade when an element is not focused / hovered over, e.g.
    /// `"600"` for nodes, "900" for edges.
    StrokeShadeNormal,
    /// Line/border shade when an element is focused, e.g.
    /// `"500"` for nodes, "800" for edges.
    StrokeShadeFocus,
    /// Line/border shade when an element has the cursor hovering over it, e.g.
    /// `"400"` for nodes, "700" for edges.
    StrokeShadeHover,
    /// Line/border shade when an element is being clicked / pressed, e.g.
    /// `"500"` for nodes, "800" for edges.
    StrokeShadeActive,
    /// Width of the border for nodes, or line for edges.
    StrokeWidth,
    /// Line/border style for all states, e.g. "none", "solid", "dashed",
    /// "dotted".
    StrokeStyle,
    /// Line/border style when an element is not focused / hovered over, e.g.
    /// "none", "solid", "dashed", "dotted".
    StrokeStyleNormal,
    /// Line/border style when an element is focused, e.g. "none", "solid",
    /// "dashed", "dotted".
    StrokeStyleFocus,
    /// Line/border style when an element has the cursor hovering over it, e.g.
    /// "none", "solid", "dashed", "dotted".
    StrokeStyleHover,
    /// Line/border style when an element is being clicked / pressed, e.g.
    /// "none", "solid", "dashed", "dotted".
    StrokeStyleActive,
}
