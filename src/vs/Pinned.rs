#[cfg(feature = "VsPinned")]
use leptos::*;
#[cfg(feature = "VsPinned")]
///This icon requires the feature `VsPinned` to be enabled.
#[component]
pub fn Pinned(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 2h7v.278c0 .406-.086.778-.258 1.117-.172.339-.42.63-.742.875v2.86c.307.145.583.328.828.546.245.219.456.464.633.735.177.27.31.565.398.882.089.318.136.646.141.985v.5H8V14l-.5 1-.5-1v-3.222H3v-.5c0-.339.047-.664.14-.977.094-.312.227-.607.4-.883A3.404 3.404 0 0 1 5 7.13V4.27a2.561 2.561 0 0 1-.734-.875A2.505 2.505 0 0 1 4 2.278V2zm1.086.778c.042.125.094.232.156.32a1.494 1.494 0 0 0 .461.43L6 3.715v4.102l-.336.117c-.411.146-.76.383-1.047.711C4.331 8.973 4.09 9.573 4 10h7c-.088-.427-.33-1.027-.617-1.355a2.456 2.456 0 0 0-1.047-.71L9 7.816V3.715l.297-.18c.094-.057.177-.122.25-.195a2.28 2.28 0 0 0 .21-.242.968.968 0 0 0 .157-.32H5.086z"
        /> < title > { title } < / title > < / svg >
    }
}
