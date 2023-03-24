#[cfg(feature = "CgScrollH")]
use leptos::*;
#[cfg(feature = "CgScrollH")]
///This icon requires the feature `CgScrollH` to be enabled.
#[component]
pub fn ScrollH(
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
        "M7.18188 9.17154L5.76766 7.75732L1.52502 12L5.76766 16.2426L7.18188 14.8284L4.35345 12L7.18188 9.17154Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.8181 14.8284L18.2323 16.2426L22.4749 12L18.2323 7.75733L16.8181 9.17155L19.6465 12L16.8181 14.8284Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M14.9999 12C14.9999 13.6569 13.6567 15 11.9999 15C10.343 15 8.99988 13.6569 8.99988 12C8.99988 10.3431 10.343 9 11.9999 9C13.6567 9 14.9999 10.3431 14.9999 12ZM12.9999 12C12.9999 12.5523 12.5522 13 11.9999 13C11.4476 13 10.9999 12.5523 10.9999 12C10.9999 11.4477 11.4476 11 11.9999 11C12.5522 11 12.9999 11.4477 12.9999 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
