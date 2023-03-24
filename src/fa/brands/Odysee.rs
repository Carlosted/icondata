#[cfg(feature = "FaBrandsOdysee")]
use leptos::*;
#[cfg(feature = "FaBrandsOdysee")]
///This icon requires the feature `FaBrandsOdysee` to be enabled.
#[component]
pub fn Odysee(
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
        "M406.7 463c-42.3 30.8-94.4 49-150.7 49C144.9 512 50.3 441.2 14.9 342.2c2.4 1.7 5.9 3.6 7.9 4.4c16.3 7.4 40.1-5.4 62.9-28.7c6.9-6.9 14.4-12.4 22.8-17.3c18.3-11.9 37.6-20.8 58.4-27.2c0 0 22.3 34.2 43.1 74.8s-22.3 54-27.2 54c-.3 0-.8 0-1.5-.1c-11-.5-70-3-56 51.1c14.9 57.4 97.5 36.6 139.6 8.9s31.7-118.3 31.7-118.3c41.1-6.4 54 37.1 57.9 59.4c.8 4.6 1.1 9.9 1.4 15.5c1.1 21.2 2.3 45.6 35.3 46.4c5.3 0 10.6-.8 15.5-2zm-95.3-23.7c-2-.5-3.5-2.5-3-5c1-2.5 3-3.5 5-3s3.5 3 3 5s-2.5 3.5-5 3zm-207-95.6c1.5-.5 3.5 1 4 3c0 2-1 4-3 4c-1.5 .5-3.5-1-4-3c-.5-1.5 1-3.5 3-4zM451.8 421C489.3 376.4 512 318.8 512 256c0-67.5-26.1-128.9-68.8-174.7c-.1 23.5-6.1 48.2-16.8 69.2c-11.9 20.3-49 58.9-69.8 78.7c-.7 .3-1.1 .9-1.5 1.4c-.2 .2-.3 .4-.5 .6c-5 6.9-4 16.8 3 21.8c21.3 15.8 56.4 45.6 59.4 72.8c3.5 34.9 27.9 75.6 34.2 86.2l0 0c.8 1.3 1.3 2.1 1.4 2.4c0 2.2-.4 4.3-.8 6.5zM390.7 251c-.5 3 1 5.9 4 6.4s5.9-1 6.4-4s-1-5.9-4-6.4c-3-1-5.9 1-6.4 4zm61.4-60.9l-11.4 5.4-3 12.9-5.4-11.4-12.9-3 11.4-5.4 3-12.9 5.4 11.4 12.9 3zM395.5 41.3c-16.2 8.2-22.1 32.8-29 61.4l0 0c-.3 1.4-.7 2.8-1 4.2c-9.5 38.5-30.6 37.6-41.7 37.2c-1.1 0-2-.1-2.9-.1c-5.1 0-6-4-8.9-17.1c-2.6-12.1-6.9-32-17.9-63.6C271.4-2.5 211.4 13.9 165.9 41.1C110.6 74.2 131.5 143 146.1 190.5c.7 2.2 1.4 4.4 2 6.6c-4 4-13.8 7.5-26 11.9c-12.1 4.3-26.6 9.5-40.3 16.9C47.9 243.9 11.5 274.9 2 288.5C.7 277.8 0 267 0 256C0 114.6 114.6 0 256 0c51.4 0 99.4 15.2 139.5 41.3zM58.9 189.6c-1.5-2-4.5-3-6.4-1.5s-3 4.5-1.5 6.4s4.5 3 6.4 1.5c2.5-1.5 3-4.5 1.5-6.4zM327.3 64.9c2-1.5 5-.5 6.4 1.5c1.5 2.5 1 5.4-1.5 6.4c-2 1.5-5 .5-6.4-1.5s-.5-5 1.5-6.4zM95.1 105c-.5 1.5 .5 3 2 3c1.5 .5 3-.5 3-2c.5-1.5-.5-3-2-3s-3 .5-3 2zm84.7-.5c-3.5-43.1 37.1-54 37.1-54c44.1-15.4 56 5.9 66.4 37.6s3 42.6-38.6 58.9s-61.9-4.5-64.9-42.6zm89.6 14.9h1c2.5 0 5-2 5-5c2-6.9 1-14.4-2-20.8c-1.5-2-4-3.5-6.4-2.5c-3 1-4.5 4-3.5 6.9c2 4.5 3 9.9 1.5 14.9c-.5 3 1.5 5.9 4.5 6.4zm-9.9-41.6c-2 0-4-1-5-3s-2-3.5-3-5c-2-2-2-5.4 0-7.4s5.4-2 7.4 0c2 2.5 3.5 5 5 7.4s.5 5.9-2.5 7.4c-.6 0-1 .2-1.3 .3c-.2 .1-.4 .2-.6 .2z"
        /> < title > { title } < / title > < / svg >
    }
}
