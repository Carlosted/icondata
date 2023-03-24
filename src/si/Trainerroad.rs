#[cfg(feature = "SiTrainerroad")]
use leptos::*;
#[cfg(feature = "SiTrainerroad")]
///This icon requires the feature `SiTrainerroad` to be enabled.
#[component]
pub fn Trainerroad(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.289 14.039c.157-.064.44-.199.51-.234 1.105-.56 1.92-1.222 2.42-1.966.527-.756.8-1.658.78-2.579 0-1.253-.456-2.193-1.398-2.874-.922-.668-2.225-.971-3.874-1.012H1.357L0 8.421h5.528c.014 0 .028.005.038.016a.02.02 0 01.004.019L2.785 16.85h3.668c.063 0 .12-.041.14-.102l2.759-8.303a.043.043 0 01.042-.024l2.823.001c.014 0 .028.005.038.015a.02.02 0 01.004.019L9.473 16.85h3.669c.064 0 .12-.042.14-.103l.742-2.26a.043.043 0 01.042-.024s2.452.005 2.452.003c.864 1.363 1.807 2.878 2.616 4.16l3.844-.002c.118 0 .19-.13.125-.229l-2.832-4.321c-.01-.022.013-.025.018-.035zm-.45-3.355c-.437.412-1.185.612-2.163.612h-2.583l.952-2.874 2.353.001c1.14.017 1.826.514 1.838 1.337.007.35-.138.688-.397.924z"
        /> < title > { title } < / title > < / svg >
    }
}
