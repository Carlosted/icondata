#[cfg(feature = "IoCardOutline")]
use leptos::*;
#[cfg(feature = "IoCardOutline")]
///This icon requires the feature `IoCardOutline` to be enabled.
#[component]
pub fn CardOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "48" y = "96" width = "416" height = "320" rx = "56" ry = "56" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "48" y1 = "192" x2 = "464" y2
        = "192" style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:60px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "128" y = "300" width = "48"
        height = "20" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:60px" /> < title > {
        title } < / title > < / svg >
    }
}
