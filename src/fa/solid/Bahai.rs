#[cfg(feature = "FaSolidBahai")]
use leptos::*;
#[cfg(feature = "FaSolidBahai")]
///This icon requires the feature `FaSolidBahai` to be enabled.
#[component]
pub fn Bahai(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M245.2 0c13.6 0 25.6 9.2 29.1 22.4l22.5 84.4 71.5-50.2c11.2-7.8 26.2-7.2 36.7 1.6s13.7 23.5 7.9 35.8l-37 79.1 87 7.5c13.6 1.2 24.7 11.3 27.1 24.8s-4.6 26.8-17 32.5l-79.2 36.8 61.8 61.7c9.7 9.6 11.6 24.6 4.8 36.4s-20.7 17.6-33.9 14L342.1 364l7.7 87c1.2 13.6-6.9 26.3-19.7 31s-27.2 .1-35-11.1l-50-71.6-50 71.6c-7.8 11.2-22.2 15.7-35 11.1s-20.9-17.4-19.7-31l7.7-87L63.9 386.7c-13.2 3.5-27.1-2.2-33.9-14s-4.8-26.7 4.8-36.4l61.8-61.7L17.4 237.9c-12.4-5.8-19.3-19.1-17-32.5s13.5-23.6 27.1-24.8l87-7.5L77.5 94C71.8 81.6 75 66.9 85.5 58.1s25.5-9.4 36.7-1.6l71.5 50.2 22.5-84.4C219.6 9.2 231.5 0 245.2 0zm0 147l-4.5 16.9c-2.5 9.5-9.6 17.2-18.8 20.5s-19.6 2-27.6-3.7l-14.3-10 7.4 15.8c4.2 8.9 3.7 19.3-1.2 27.8s-13.7 14.1-23.5 14.9l-17.4 1.5 15.9 7.4c8.9 4.1 15.3 12.4 17 22.1s-1.4 19.6-8.4 26.6l-12.4 12.4 16.9-4.5c9.5-2.6 19.7-.3 27.2 6s11.5 15.9 10.6 25.7l-1.5 17.4 10-14.3c5.6-8.1 14.9-12.9 24.7-12.9s19.1 4.8 24.7 12.9l10 14.3-1.5-17.4c-.9-9.8 3.1-19.4 10.6-25.7s17.7-8.6 27.2-6l16.9 4.5-12.4-12.4c-7-6.9-10.1-16.9-8.4-26.6s8-17.9 17-22.1l15.9-7.4-17.4-1.5c-9.8-.8-18.6-6.4-23.5-14.9s-5.4-18.9-1.2-27.8l7.4-15.8-14.3 10c-8.1 5.7-18.4 7-27.6 3.7s-16.3-11-18.8-20.5L245.2 147z"
        /> < title > { title } < / title > < / svg >
    }
}
