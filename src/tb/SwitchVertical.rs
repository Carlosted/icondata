#[cfg(feature = "TbSwitchVertical")]
use leptos::*;
#[cfg(feature = "TbSwitchVertical")]
///This icon requires the feature `TbSwitchVertical` to be enabled.
#[component]
pub fn SwitchVertical(
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
        "icon icon-tabler icon-tabler-switch-vertical" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 8l4 -4l4 4" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M7 4l0 9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 16l4 4l4 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 10l0 10" /> < title > { title } < / title >
        < / svg >
    }
}
