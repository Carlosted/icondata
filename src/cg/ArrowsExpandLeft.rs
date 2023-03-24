#[cfg(feature = "CgArrowsExpandLeft")]
use leptos::*;
#[cfg(feature = "CgArrowsExpandLeft")]
///This icon requires the feature `CgArrowsExpandLeft` to be enabled.
#[component]
pub fn ArrowsExpandLeft(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.1005 4.10052V2.10052H2.10046L2.10046 10.1005H4.10046L4.10046 5.51471L9.87875 11.293L11.293 9.87878L5.51471 4.10052H10.1005Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.8995 13.8995H21.8995V21.8995H13.8995V19.8995H18.4853L12.7071 14.1212L14.1213 12.707L19.8995 18.4853V13.8995Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
