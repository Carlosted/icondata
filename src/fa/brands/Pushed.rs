#[cfg(feature = "FaBrandsPushed")]
use leptos::*;
#[cfg(feature = "FaBrandsPushed")]
///This icon requires the feature `FaBrandsPushed` to be enabled.
#[component]
pub fn Pushed(
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
        stroke_witdh = "0" style = style viewBox = "0 0 432 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M407 111.9l-98.5-9 14-33.4c10.4-23.5-10.8-40.4-28.7-37L22.5 76.9c-15.1 2.7-26 18.3-21.4 36.6l105.1 348.3c6.5 21.3 36.7 24.2 47.7 7l35.3-80.8 235.2-231.3c16.4-16.8 4.3-42.9-17.4-44.8zM297.6 53.6c5.1-.7 7.5 2.5 5.2 7.4L286 100.9 108.6 84.6l189-31zM22.7 107.9c-3.1-5.1 1-10 6.1-9.1l248.7 22.7-96.9 230.7L22.7 107.9zM136 456.4c-2.6 4-7.9 3.1-9.4-1.2L43.5 179.7l127.7 197.6c-7 15-35.2 79.1-35.2 79.1zm272.8-314.5L210.1 337.3l89.7-213.7 106.4 9.7c4 1.1 5.7 5.3 2.6 8.6z"
        /> < title > { title } < / title > < / svg >
    }
}
