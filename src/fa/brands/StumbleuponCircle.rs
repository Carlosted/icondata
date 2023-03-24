#[cfg(feature = "FaBrandsStumbleuponCircle")]
use leptos::*;
#[cfg(feature = "FaBrandsStumbleuponCircle")]
///This icon requires the feature `FaBrandsStumbleuponCircle` to be enabled.
#[component]
pub fn StumbleuponCircle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 496 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256 8C119 8 8 119 8 256s111 248 248 248 248-111 248-248S393 8 256 8zm0 177.5c-9.8 0-17.8 8-17.8 17.8v106.9c0 40.9-33.9 73.9-74.9 73.9-41.4 0-74.9-33.5-74.9-74.9v-46.5h57.3v45.8c0 10 8 17.8 17.8 17.8s17.8-7.9 17.8-17.8V200.1c0-40 34.2-72.1 74.7-72.1 40.7 0 74.7 32.3 74.7 72.6v23.7l-34.1 10.1-22.9-10.7v-20.6c.1-9.6-7.9-17.6-17.7-17.6zm167.6 123.6c0 41.4-33.5 74.9-74.9 74.9-41.2 0-74.9-33.2-74.9-74.2V263l22.9 10.7 34.1-10.1v47.1c0 9.8 8 17.6 17.8 17.6s17.8-7.9 17.8-17.6v-48h57.3c-.1 45.9-.1 46.4-.1 46.4z"
        /> < title > { title } < / title > < / svg >
    }
}
