#[cfg(feature = "TbLogicXor")]
use leptos::*;
#[cfg(feature = "TbLogicXor")]
///This icon requires the feature `TbLogicXor` to be enabled.
#[component]
pub fn LogicXor(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-logic-xor"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M22 12h-4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M2 9h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 15h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 19c1.778 -4.667 1.778 -9.333 0 -14" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10 5c10.667 2.1 10.667 12.6 0 14c1.806 -4.667 1.806 -9.333 0 -14z" /> < title >
        { title } < / title > < / svg >
    }
}
