#[cfg(feature = "FaSolidPersonFalling")]
use leptos::*;
#[cfg(feature = "FaSolidPersonFalling")]
///This icon requires the feature `FaSolidPersonFalling` to be enabled.
#[component]
pub fn PersonFalling(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256 0c17.7 0 32 14.3 32 32l0 9.8c0 54.6-27.9 104.6-72.5 133.6l.2 .3L272.5 256l87.5 0c15.1 0 29.3 7.1 38.4 19.2l43.2 57.6c10.6 14.1 7.7 34.2-6.4 44.8s-34.2 7.7-44.8-6.4L352 320l-96 0h-1.4l92.3 142.6c9.6 14.8 5.4 34.6-9.5 44.3s-34.6 5.4-44.3-9.5L132.5 249.2c-2.9 9.2-4.5 19-4.5 29l0 73.8c0 17.7-14.3 32-32 32s-32-14.3-32-32V278.2c0-65.1 39.6-123.7 100.1-147.9C200.3 115.8 224 80.8 224 41.8l0-9.8c0-17.7 14.3-32 32-32zM80 32a48 48 0 1 1 0 96 48 48 0 1 1 0-96z"
        /> < title > { title } < / title > < / svg >
    }
}
