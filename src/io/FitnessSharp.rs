#[cfg(feature = "IoFitnessSharp")]
use leptos::*;
#[cfg(feature = "IoFitnessSharp")]
///This icon requires the feature `IoFitnessSharp` to be enabled.
#[component]
pub fn FitnessSharp(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,272H442.66a261.41,261.41,0,0,1-18.25,32H480Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M32,240v32H69.34a225.1,225.1,0,0,1-12.4-32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M304,259.78,252.27,363.24l-48-160L169.89,272H69.34c10,20.92,23.5,41.41,40.63,61.68,40.12,47.46,94.25,79.75,137,108.32l9,6,9-6c42.78-28.57,96.91-60.86,137-108.32A322.78,322.78,0,0,0,424.41,304h-98.3Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M211.73,116.76l48,160L304,188.22,345.89,272h96.77A213.13,213.13,0,0,0,464,176.65C463.37,114.54,413.54,64,352.92,64c-48.11,0-80.1,28-96.92,48.21C239.18,92,207.19,64,159.08,64,98.46,64,48.63,114.54,48,176.65A211.23,211.23,0,0,0,56.94,240h93.17Z"
        /> < title > { title } < / title > < / svg >
    }
}
