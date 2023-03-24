#[cfg(feature = "CgChevronDoubleDownR")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleDownR")]
///This icon requires the feature `CgChevronDoubleDownR` to be enabled.
#[component]
pub fn ChevronDoubleDownR(
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
        "M7.75732 8.46448L9.17154 7.05026L12 9.87869L14.8284 7.05029L16.2426 8.46451L12 12.7071L7.75732 8.46448Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.17154 11.2929L7.75732 12.7071L11.9999 16.9498L16.2426 12.7071L14.8284 11.2929L12 14.1213L9.17154 11.2929Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 5C1 2.79086 2.79086 1 5 1H19C21.2091 1 23 2.79086 23 5V19C23 21.2091 21.2091 23 19 23H5C2.79086 23 1 21.2091 1 19V5ZM5 3H19C20.1046 3 21 3.89543 21 5V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V5C3 3.89543 3.89543 3 5 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
