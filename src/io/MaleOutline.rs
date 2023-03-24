#[cfg(feature = "IoMaleOutline")]
use leptos::*;
#[cfg(feature = "IoMaleOutline")]
///This icon requires the feature `IoMaleOutline` to be enabled.
#[component]
pub fn MaleOutline(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "216" cy = "296" r = "152" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "448 160 448 64 352 64" fill = "none"
        stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round" stroke -
        width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "324" y1 = "188"
        x2 = "448" y2 = "64" fill = "none" stroke = "#000" stroke - linecap = "round"
        stroke - linejoin = "round" stroke - width = "32" /> < title > { title } < /
        title > < / svg >
    }
}
