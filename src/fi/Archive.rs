#[cfg(feature = "FiArchive")]
use leptos::*;
#[cfg(feature = "FiArchive")]
///This icon requires the feature `FiArchive` to be enabled.
#[component]
pub fn Archive(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "21 8 21 21 3 21 3 8" />< rect xmlns = "http://www.w3.org/2000/svg" x =
        "1" y = "3" width = "22" height = "5" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "10" y1 = "12" x2 = "14" y2 = "12" /> < title >
        { title } < / title > < / svg >
    }
}
