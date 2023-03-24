#[cfg(feature = "IoStorefront")]
use leptos::*;
#[cfg(feature = "IoStorefront")]
///This icon requires the feature `IoStorefront` to be enabled.
#[component]
pub fn Storefront(
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
        "M480,448H468a4,4,0,0,1-4-4V273.51a4,4,0,0,0-5.24-3.86,104.92,104.92,0,0,1-28.32,4.78c-1.18,0-2.3.05-3.4.05a108.22,108.22,0,0,1-52.85-13.64,8.23,8.23,0,0,0-8,0,108.18,108.18,0,0,1-52.84,13.64,106.11,106.11,0,0,1-52.46-13.79,8.21,8.21,0,0,0-8.09,0,108.14,108.14,0,0,1-53.16,13.8,106.19,106.19,0,0,1-52.77-14,8.25,8.25,0,0,0-8.16,0,106.19,106.19,0,0,1-52.77,14c-1.09,0-2.19,0-3.37-.05h-.06a104.91,104.91,0,0,1-29.28-5.09A4,4,0,0,0,48,273.15V444a4,4,0,0,1-4,4H32.5c-8.64,0-16.1,6.64-16.48,15.28A16,16,0,0,0,32,480H479.5c8.64,0,16.1-6.64,16.48-15.28A16,16,0,0,0,480,448ZM224,380a4,4,0,0,1-4,4H132a4,4,0,0,1-4-4V316a12,12,0,0,1,12-12h72a12,12,0,0,1,12,12ZM380,448H308a4,4,0,0,1-4-4V316a12,12,0,0,1,12-12h56a12,12,0,0,1,12,12V444A4,4,0,0,1,380,448Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M492.57,170.28,449.65,71.79C438.41,47.62,412.74,32,384.25,32H127.7C99.21,32,73.54,47.62,62.3,71.79L19.38,170.28c-9,19.41,2.89,39.34,2.9,39.35l.28.45c.49.78,1.36,2,1.89,2.78.05.06.09.13.14.2l5,6.05a7.45,7.45,0,0,0,.6.65l5,4.83.42.36A69.65,69.65,0,0,0,45,231.73v.05a74,74,0,0,0,36,10.67c.82,0,1.64,0,2.47,0a76.08,76.08,0,0,0,51.89-20.31l.33-.31a7.94,7.94,0,0,1,10.89,0l.33.31a77.3,77.3,0,0,0,104.46,0,8,8,0,0,1,10.87,0h0a77.31,77.31,0,0,0,104.21.23,7.88,7.88,0,0,1,10.71,0,76.81,76.81,0,0,0,52.31,20.08l2.49,0a71.35,71.35,0,0,0,35-10.7v0c.95-.57,1.86-1.17,2.78-1.77A71.33,71.33,0,0,0,488,212.17l1.74-2.63q.26-.4.48-.84C491.88,205.32,500.78,187.94,492.57,170.28Z"
        /> < title > { title } < / title > < / svg >
    }
}
