#[cfg(feature = "FaSolidShieldHalved")]
use leptos::*;
#[cfg(feature = "FaSolidShieldHalved")]
///This icon requires the feature `FaSolidShieldHalved` to be enabled.
#[component]
pub fn ShieldHalved(
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
        "M240 0c4.6 0 9.2 1 13.4 2.9L441.7 82.8c22 9.3 38.4 31 38.3 57.2c-.5 99.2-41.3 280.7-213.7 363.2c-16.7 8-36.1 8-52.8 0C41.3 420.7 .5 239.2 0 140c-.1-26.2 16.3-47.9 38.3-57.2L226.7 2.9C230.8 1 235.4 0 240 0zm0 66.8V444.8C378 378 415.1 230.1 416 141.4L240 66.8l0 0z"
        /> < title > { title } < / title > < / svg >
    }
}
