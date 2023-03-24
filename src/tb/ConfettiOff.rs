#[cfg(feature = "TbConfettiOff")]
use leptos::*;
#[cfg(feature = "TbConfettiOff")]
///This icon requires the feature `TbConfettiOff` to be enabled.
#[component]
pub fn ConfettiOff(
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
        "icon icon-tabler icon-tabler-confetti-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 5h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 5v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11.5 4l-.5 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 5h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 4v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 9l-1 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 13l2 -.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 19h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 19v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 16.518l-6.518 -6.518l-4.39 9.58a1 1 0 0 0 1.329 1.329l9.579 -4.39v0z" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title }
        < / title > < / svg >
    }
}
