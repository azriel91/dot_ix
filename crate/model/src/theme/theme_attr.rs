use serde::{Deserialize, Serialize};

/// Themeable attributes of nodes and edges.
///
/// These keys are used by both nodes and edges.
///
/// # Notes
///
/// * <span class=""color-label>`Stroke*</span>` keys control the border colour
///   for nodes, and the line colour for edges.
/// * <span class=""color-label>`Fill*</span>` keys control the background
///   colour for nodes, and the arrow head fill colour for edges.
/// * <span class=""color-label>`Padding*</span>` keys are only applicable to
///   nodes.
/// * <span class=""color-label>`Margin*</span>` keys are only applicable to
///   nodes.
///
/// # Colours
///
/// [Colours] are from [Tailwind CSS]. The list of names are:
///
/// * <span class="color-label">`slate`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f8fafc;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f1f5f9;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e2e8f0;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #cbd5e1;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #94a3b8;">400</span>
/// <span class="colorblk" style="background-color: #64748b;">500</span>
/// <span class="colorblk" style="background-color: #475569;">600</span>
/// <span class="colorblk" style="background-color: #334155;">700</span>
/// <span class="colorblk" style="background-color: #1e293b;">800</span>
/// <span class="colorblk" style="background-color: #0f172a;">900</span>
/// <span class="colorblk" style="background-color: #020617;">950</span>
/// * <span class="color-label">`gray`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f9fafb;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f3f4f6;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e5e7eb;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #d1d5db;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #9ca3af;">400</span>
/// <span class="colorblk" style="background-color: #6b7280;">500</span>
/// <span class="colorblk" style="background-color: #4b5563;">600</span>
/// <span class="colorblk" style="background-color: #374151;">700</span>
/// <span class="colorblk" style="background-color: #1f2937;">800</span>
/// <span class="colorblk" style="background-color: #111827;">900</span>
/// <span class="colorblk" style="background-color: #030712;">950</span>
/// * <span class="color-label">`zinc`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fafafa;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f4f4f5;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e4e4e7;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #d4d4d8;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #a1a1aa;">400</span>
/// <span class="colorblk" style="background-color: #71717a;">500</span>
/// <span class="colorblk" style="background-color: #52525b;">600</span>
/// <span class="colorblk" style="background-color: #3f3f46;">700</span>
/// <span class="colorblk" style="background-color: #27272a;">800</span>
/// <span class="colorblk" style="background-color: #18181b;">900</span>
/// <span class="colorblk" style="background-color: #09090b;">950</span>
/// * <span class="color-label">`neutral`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fafafa;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f5f5f5;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e5e5e5;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #d4d4d4;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #a3a3a3;">400</span>
/// <span class="colorblk" style="background-color: #737373;">500</span>
/// <span class="colorblk" style="background-color: #525252;">600</span>
/// <span class="colorblk" style="background-color: #404040;">700</span>
/// <span class="colorblk" style="background-color: #262626;">800</span>
/// <span class="colorblk" style="background-color: #171717;">900</span>
/// <span class="colorblk" style="background-color: #0a0a0a;">950</span>
/// * <span class="color-label">`stone`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fafaf9;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f5f5f4;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e7e5e4;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #d6d3d1;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #a8a29e;">400</span>
/// <span class="colorblk" style="background-color: #78716c;">500</span>
/// <span class="colorblk" style="background-color: #57534e;">600</span>
/// <span class="colorblk" style="background-color: #44403c;">700</span>
/// <span class="colorblk" style="background-color: #292524;">800</span>
/// <span class="colorblk" style="background-color: #1c1917;">900</span>
/// <span class="colorblk" style="background-color: #0c0a09;">950</span>
/// * <span class="color-label">`red`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fef2f2;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #fee2e2;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fecaca;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #fca5a5;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #f87171;">400</span>
/// <span class="colorblk" style="background-color: #ef4444;">500</span>
/// <span class="colorblk" style="background-color: #dc2626;">600</span>
/// <span class="colorblk" style="background-color: #b91c1c;">700</span>
/// <span class="colorblk" style="background-color: #991b1b;">800</span>
/// <span class="colorblk" style="background-color: #7f1d1d;">900</span>
/// <span class="colorblk" style="background-color: #450a0a;">950</span>
/// * <span class="color-label">`orange`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fff7ed;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #ffedd5;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fed7aa;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #fdba74;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #fb923c;">400</span>
/// <span class="colorblk" style="background-color: #f97316;">500</span>
/// <span class="colorblk" style="background-color: #ea580c;">600</span>
/// <span class="colorblk" style="background-color: #c2410c;">700</span>
/// <span class="colorblk" style="background-color: #9a3412;">800</span>
/// <span class="colorblk" style="background-color: #7c2d12;">900</span>
/// <span class="colorblk" style="background-color: #431407;">950</span>
/// * <span class="color-label">`amber`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fffbeb;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #fef3c7;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fde68a;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #fcd34d;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #fbbf24;">400</span>
/// <span class="colorblk" style="background-color: #f59e0b;">500</span>
/// <span class="colorblk" style="background-color: #d97706;">600</span>
/// <span class="colorblk" style="background-color: #b45309;">700</span>
/// <span class="colorblk" style="background-color: #92400e;">800</span>
/// <span class="colorblk" style="background-color: #78350f;">900</span>
/// <span class="colorblk" style="background-color: #451a03;">950</span>
/// * <span class="color-label">`yellow`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fefce8;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #fef9c3;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fef08a;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #fde047;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #facc15;">400</span>
/// <span class="colorblk" style="background-color: #eab308;">500</span>
/// <span class="colorblk" style="background-color: #ca8a04;">600</span>
/// <span class="colorblk" style="background-color: #a16207;">700</span>
/// <span class="colorblk" style="background-color: #854d0e;">800</span>
/// <span class="colorblk" style="background-color: #713f12;">900</span>
/// <span class="colorblk" style="background-color: #422006;">950</span>
/// * <span class="color-label">`lime`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f7fee7;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #ecfccb;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #d9f99d;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #bef264;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #a3e635;">400</span>
/// <span class="colorblk" style="background-color: #84cc16;">500</span>
/// <span class="colorblk" style="background-color: #65a30d;">600</span>
/// <span class="colorblk" style="background-color: #4d7c0f;">700</span>
/// <span class="colorblk" style="background-color: #3f6212;">800</span>
/// <span class="colorblk" style="background-color: #365314;">900</span>
/// <span class="colorblk" style="background-color: #1a2e05;">950</span>
/// * <span class="color-label">`green`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f0fdf4;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #dcfce7;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #bbf7d0;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #86efac;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #4ade80;">400</span>
/// <span class="colorblk" style="background-color: #22c55e;">500</span>
/// <span class="colorblk" style="background-color: #16a34a;">600</span>
/// <span class="colorblk" style="background-color: #15803d;">700</span>
/// <span class="colorblk" style="background-color: #166534;">800</span>
/// <span class="colorblk" style="background-color: #14532d;">900</span>
/// <span class="colorblk" style="background-color: #052e16;">950</span>
/// * <span class="color-label">`emerald`:</span>
/// <span class="colorblk txt-drk" style="background-color: #ecfdf5;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #d1fae5;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #a7f3d0;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #6ee7b7;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #34d399;">400</span>
/// <span class="colorblk" style="background-color: #10b981;">500</span>
/// <span class="colorblk" style="background-color: #059669;">600</span>
/// <span class="colorblk" style="background-color: #047857;">700</span>
/// <span class="colorblk" style="background-color: #065f46;">800</span>
/// <span class="colorblk" style="background-color: #064e3b;">900</span>
/// <span class="colorblk" style="background-color: #022c22;">950</span>
/// * <span class="color-label">`teal`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f0fdfa;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #ccfbf1;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #99f6e4;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #5eead4;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #2dd4bf;">400</span>
/// <span class="colorblk" style="background-color: #14b8a6;">500</span>
/// <span class="colorblk" style="background-color: #0d9488;">600</span>
/// <span class="colorblk" style="background-color: #0f766e;">700</span>
/// <span class="colorblk" style="background-color: #115e59;">800</span>
/// <span class="colorblk" style="background-color: #134e4a;">900</span>
/// <span class="colorblk" style="background-color: #042f2e;">950</span>
/// * <span class="color-label">`cyan`:</span>
/// <span class="colorblk txt-drk" style="background-color: #ecfeff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #cffafe;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #a5f3fc;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #67e8f9;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #22d3ee;">400</span>
/// <span class="colorblk" style="background-color: #06b6d4;">500</span>
/// <span class="colorblk" style="background-color: #0891b2;">600</span>
/// <span class="colorblk" style="background-color: #0e7490;">700</span>
/// <span class="colorblk" style="background-color: #155e75;">800</span>
/// <span class="colorblk" style="background-color: #164e63;">900</span>
/// <span class="colorblk" style="background-color: #083344;">950</span>
/// * <span class="color-label">`sky`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f0f9ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #e0f2fe;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #bae6fd;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #7dd3fc;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #38bdf8;">400</span>
/// <span class="colorblk" style="background-color: #0ea5e9;">500</span>
/// <span class="colorblk" style="background-color: #0284c7;">600</span>
/// <span class="colorblk" style="background-color: #0369a1;">700</span>
/// <span class="colorblk" style="background-color: #075985;">800</span>
/// <span class="colorblk" style="background-color: #0c4a6e;">900</span>
/// <span class="colorblk" style="background-color: #082f49;">950</span>
/// * <span class="color-label">`blue`:</span>
/// <span class="colorblk txt-drk" style="background-color: #eff6ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #dbeafe;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #bfdbfe;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #93c5fd;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #60a5fa;">400</span>
/// <span class="colorblk" style="background-color: #3b82f6;">500</span>
/// <span class="colorblk" style="background-color: #2563eb;">600</span>
/// <span class="colorblk" style="background-color: #1d4ed8;">700</span>
/// <span class="colorblk" style="background-color: #1e40af;">800</span>
/// <span class="colorblk" style="background-color: #1e3a8a;">900</span>
/// <span class="colorblk" style="background-color: #172554;">950</span>
/// * <span class="color-label">`indigo`:</span>
/// <span class="colorblk txt-drk" style="background-color: #eef2ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #e0e7ff;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #c7d2fe;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #a5b4fc;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #818cf8;">400</span>
/// <span class="colorblk" style="background-color: #6366f1;">500</span>
/// <span class="colorblk" style="background-color: #4f46e5;">600</span>
/// <span class="colorblk" style="background-color: #4338ca;">700</span>
/// <span class="colorblk" style="background-color: #3730a3;">800</span>
/// <span class="colorblk" style="background-color: #312e81;">900</span>
/// <span class="colorblk" style="background-color: #1e1b4b;">950</span>
/// * <span class="color-label">`violet`:</span>
/// <span class="colorblk txt-drk" style="background-color: #f5f3ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #ede9fe;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #ddd6fe;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #c4b5fd;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #a78bfa;">400</span>
/// <span class="colorblk" style="background-color: #8b5cf6;">500</span>
/// <span class="colorblk" style="background-color: #7c3aed;">600</span>
/// <span class="colorblk" style="background-color: #6d28d9;">700</span>
/// <span class="colorblk" style="background-color: #5b21b6;">800</span>
/// <span class="colorblk" style="background-color: #4c1d95;">900</span>
/// <span class="colorblk" style="background-color: #2e1065;">950</span>
/// * <span class="color-label">`purple`:</span>
/// <span class="colorblk txt-drk" style="background-color: #faf5ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #f3e8ff;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #e9d5ff;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #d8b4fe;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #c084fc;">400</span>
/// <span class="colorblk" style="background-color: #a855f7;">500</span>
/// <span class="colorblk" style="background-color: #9333ea;">600</span>
/// <span class="colorblk" style="background-color: #7e22ce;">700</span>
/// <span class="colorblk" style="background-color: #6b21a8;">800</span>
/// <span class="colorblk" style="background-color: #581c87;">900</span>
/// <span class="colorblk" style="background-color: #3b0764;">950</span>
/// * <span class="color-label">`fuchsia`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fdf4ff;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #fae8ff;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #f5d0fe;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #f0abfc;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #e879f9;">400</span>
/// <span class="colorblk" style="background-color: #d946ef;">500</span>
/// <span class="colorblk" style="background-color: #c026d3;">600</span>
/// <span class="colorblk" style="background-color: #a21caf;">700</span>
/// <span class="colorblk" style="background-color: #86198f;">800</span>
/// <span class="colorblk" style="background-color: #701a75;">900</span>
/// <span class="colorblk" style="background-color: #4a044e;">950</span>
/// * <span class="color-label">`pink`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fdf2f8;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #fce7f3;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fbcfe8;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #f9a8d4;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #f472b6;">400</span>
/// <span class="colorblk" style="background-color: #ec4899;">500</span>
/// <span class="colorblk" style="background-color: #db2777;">600</span>
/// <span class="colorblk" style="background-color: #be185d;">700</span>
/// <span class="colorblk" style="background-color: #9d174d;">800</span>
/// <span class="colorblk" style="background-color: #831843;">900</span>
/// <span class="colorblk" style="background-color: #500724;">950</span>
/// * <span class="color-label">`rose`:</span>
/// <span class="colorblk txt-drk" style="background-color: #fff1f2;">50</span>
/// <span class="colorblk txt-drk" style="background-color: #ffe4e6;">100</span>
/// <span class="colorblk txt-drk" style="background-color: #fecdd3;">200</span>
/// <span class="colorblk txt-drk" style="background-color: #fda4af;">300</span>
/// <span class="colorblk txt-drk" style="background-color: #fb7185;">400</span>
/// <span class="colorblk" style="background-color: #f43f5e;">500</span>
/// <span class="colorblk" style="background-color: #e11d48;">600</span>
/// <span class="colorblk" style="background-color: #be123c;">700</span>
/// <span class="colorblk" style="background-color: #9f1239;">800</span>
/// <span class="colorblk" style="background-color: #881337;">900</span>
/// <span class="colorblk" style="background-color: #4c0519;">950</span>
///
/// <style>
/// .color-label { display: inline-block; width: 80px; }
/// .colorblk { padding: 0 3px; border-radius: 3px; font-family: monospace; }
/// .txt-drk { color: #111111; }
/// </style>
///
/// [Colours]: https://tailwindcss.com/docs/customizing-colors
/// [Tailwind CSS]: https://tailwindcss.com/
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ThemeAttr {
    /// Base [colour] for shape colourable attributes, e.g. stroke/border, fill.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    ShapeColor,
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    StrokeColor,
    /// Base [colour] when a node/edge is not focused / hovered over, e.g.
    /// `"amber-600"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    StrokeColorNormal,
    /// Base [colour] when a node/edge is focused, e.g. `"amber-500"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    StrokeColorFocus,
    /// Base [colour] when a node/edge has the cursor hovering over it, e.g.
    /// `"amber-400"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    StrokeColorHover,
    /// Base [colour] when a node/edge is being clicked / pressed, e.g.
    /// `"amber-300"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    StrokeColorActive,
    /// Width of the border for nodes, or line for edges.
    StrokeWidth,
    /// Border style of the node -- dotted, dashed, solid.
    StrokeStyle,
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    FillColor,
    /// Base [colour] for the background when a node/edge is not focused /
    /// hovered over, e.g. `"amber-300"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    FillColorNormal,
    /// Base [colour] for the background when a node/edge is focused, e.g.
    /// `"amber-200"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    FillColorFocus,
    /// Base [colour] for the background when a node/edge has the cursor
    /// hovering over it, e.g. `"amber-100"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    FillColorHover,
    /// Base [colour] for the background when a node/edge is being clicked /
    /// pressed, e.g. `"amber-200"`.
    ///
    /// [colour]: https://tailwindcss.com/docs/customizing-colors
    FillColorActive,
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
}
