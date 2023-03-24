#[cfg(feature = "FaSolidHandSpock")]
use leptos::*;
#[cfg(feature = "FaSolidHandSpock")]
///This icon requires the feature `FaSolidHandSpock` to be enabled.
#[component]
pub fn HandSpock(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M214.9 23.7C210.3 6.6 192.8-3.5 175.7 1.1s-27.2 22.1-22.6 39.2L206 237.8c2.5 9.2-4.5 18.2-14 18.2c-6.4 0-12-4.2-13.9-10.3L134.6 102.7c-5.1-16.9-23-26.4-39.9-21.3s-26.4 23-21.3 39.9l62.8 206.4c2.4 7.9-7.2 13.8-13.2 8.1L67.6 283c-16-15.2-41.3-14.6-56.6 1.4s-14.6 41.3 1.4 56.6L124.8 448c43.1 41.1 100.4 64 160 64h10.9 8.2c.1 0 .1-.1 .1-.1s.1-.1 .1-.1c58.3-3.5 108.6-43.2 125.3-99.7l81.2-275c5-16.9-4.7-34.7-21.6-39.8s-34.7 4.7-39.8 21.6L411.5 247.1c-1.6 5.3-6.4 8.9-12 8.9c-7.9 0-13.8-7.3-12.2-15.1l36-170.3c3.7-17.3-7.4-34.3-24.7-37.9s-34.3 7.4-37.9 24.7L323.1 235.1c-2.6 12.2-13.3 20.9-25.8 20.9c-11.9 0-22.4-8-25.4-19.5l-57-212.8z"
        /> < title > { title } < / title > < / svg >
    }
}
