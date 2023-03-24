#[cfg(feature = "SiAvast")]
use leptos::*;
#[cfg(feature = "SiAvast")]
///This icon requires the feature `SiAvast` to be enabled.
#[component]
pub fn Avast(
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
        "M10.2941 2.991a3.0286 3.0286 0 0 1 4.4499 1.2039l7.0018 14.8042a11.937 11.937 0 0 0 2.2539-6.9131C24.0464 5.4569 18.7112.047 12.0834.0004 5.4556-.0463.047 5.2889.0004 11.9167a11.9356 11.9356 0 0 0 2.2213 7.0344l.2813-.0613 4.5692-1.008c.1287-.0286.1967.1454.084.2127L3.3736 20.337a11.9563 11.9563 0 0 0 8.5431 3.6625c3.76.0267 7.1258-1.68 9.3444-4.3705L9.8095 7.5735a3.0272 3.0272 0 0 1 .4846-4.5826zm2.1493 13.6089-7.3731.64a1.302 1.302 0 1 1 .1866-2.5666l7.2031 1.6972c.1287.0314.114.2174-.0166.2294zM9.03 10.116l8.9404 7.2324c.102.0827.01.2447-.1133.198L7.1035 13.4713a1.9593 1.9593 0 1 1 1.9266-3.3552z"
        /> < title > { title } < / title > < / svg >
    }
}
