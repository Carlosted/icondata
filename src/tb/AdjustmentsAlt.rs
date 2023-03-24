#[cfg(feature = "TbAdjustmentsAlt")]
use leptos::*;
#[cfg(feature = "TbAdjustmentsAlt")]
///This icon requires the feature `TbAdjustmentsAlt` to be enabled.
#[component]
pub fn AdjustmentsAlt(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-adjustments-alt" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 8h4v4h-4z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 4l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12l0 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 14h4v4h-4z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 4l0 10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 18l0 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 5h4v4h-4z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 4l0 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 9l0 11" /> < title > { title } < / title >
        < / svg >
    }
}
