#[cfg(feature = "CgDuplicate")]
use leptos::*;
#[cfg(feature = "CgDuplicate")]
///This icon requires the feature `CgDuplicate` to be enabled.
#[component]
pub fn Duplicate(
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
        "M19 5H7V3H21V17H19V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 13V11H11V13H13V15H11V17H9V15H7V13H9Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d = "M3 7H17V21H3V7ZM5 9H15V19H5V9Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
