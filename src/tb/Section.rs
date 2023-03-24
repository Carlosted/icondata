#[cfg(feature = "TbSection")]
use leptos::*;
#[cfg(feature = "TbSection")]
///This icon requires the feature `TbSection` to be enabled.
#[component]
pub fn Section(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-section"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 20h.01" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M4 20h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 20h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 4h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 4h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 4h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 4h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 8m0 1a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1z"
        /> < title > { title } < / title > < / svg >
    }
}
