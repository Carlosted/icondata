#[cfg(feature = "TbLockAccessOff")]
use leptos::*;
#[cfg(feature = "TbLockAccessOff")]
///This icon requires the feature `TbLockAccessOff` to be enabled.
#[component]
pub fn LockAccessOff(
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
        "icon icon-tabler icon-tabler-lock-access-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 8v-2c0 -.554 .225 -1.055 .588 -1.417" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16v2a2 2 0 0 0 2 2h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4h2a2 2 0 0 1 2 2v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20h2c.55 0 1.05 -.222 1.41 -.582" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15 11a1 1 0 0 1 1 1m-.29 3.704a1 1 0 0 1 -.71 .296h-6a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h2"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 11v-1m1.182 -2.826a2 2 0 0 1 2.818 1.826v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
