#[cfg(feature = "FaRegularFaceGrinBeamSweat")]
use leptos::*;
#[cfg(feature = "FaRegularFaceGrinBeamSweat")]
///This icon requires the feature `FaRegularFaceGrinBeamSweat` to be enabled.
#[component]
pub fn FaceGrinBeamSweat(
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
        "M476.8 126.3C497.1 120.8 512 102.7 512 81c0-20-28.6-60.4-41.6-77.7c-3.2-4.4-9.6-4.4-12.8 0c-9.5 12.6-27.1 37.2-36 57.5c-.3 .7-.6 1.4-.9 2.1C417.8 69.7 416 76 416 81c0 26 21.5 47 48 47c4.4 0 8.7-.6 12.8-1.7zM395.4 41.2C355.3 15.2 307.4 0 256 0C114.6 0 0 114.6 0 256S114.6 512 256 512s256-114.6 256-256c0-35.8-7.3-69.9-20.6-100.8c-8.6 3.1-17.8 4.8-27.4 4.8c-8.9 0-17.6-1.5-25.7-4.2C454.7 185.5 464 219.7 464 256c0 114.9-93.1 208-208 208S48 370.9 48 256S141.1 48 256 48c48.7 0 93.4 16.7 128.9 44.7c-.6-3.8-.9-7.7-.9-11.7c0-11.4 3.8-22.4 7.1-30.5c1.3-3.1 2.7-6.2 4.3-9.3zM375 336.5c10.4-16.1-6.8-32.5-25.5-28.1c-28.9 6.8-60.5 10.5-93.6 10.5s-64.7-3.7-93.6-10.5c-18.7-4.4-35.9 12-25.5 28.1c24.6 38.1 68.7 63.5 119.1 63.5s94.5-25.4 119.1-63.5zM217.6 228.8l0 0 0 0 0 0c2.1 2.8 5.7 3.9 8.9 2.8s5.5-4.1 5.5-7.6c0-17.9-6.7-35.6-16.6-48.8c-9.8-13-23.9-23.2-39.4-23.2s-29.6 10.2-39.4 23.2C126.7 188.4 120 206.1 120 224c0 3.4 2.2 6.5 5.5 7.6s6.9 0 8.9-2.8l0 0 0 0 0 0 .2-.2c.2-.2 .4-.5 .7-.9c.6-.8 1.6-2 2.8-3.4c2.5-2.8 6-6.6 10.2-10.3c8.8-7.8 18.8-14 27.7-14s18.9 6.2 27.7 14c4.2 3.7 7.7 7.5 10.2 10.3c1.2 1.4 2.2 2.6 2.8 3.4c.3 .4 .6 .7 .7 .9l.2 .2 0 0zm160 0l0 0 0 0c2.1 2.8 5.7 3.9 8.9 2.8s5.5-4.1 5.5-7.6c0-17.9-6.7-35.6-16.6-48.8c-9.8-13-23.9-23.2-39.4-23.2s-29.6 10.2-39.4 23.2C286.7 188.4 280 206.1 280 224c0 3.4 2.2 6.5 5.5 7.6s6.9 0 8.9-2.8l0 0 0 0 0 0 .2-.2c.2-.2 .4-.5 .7-.9c.6-.8 1.6-2 2.8-3.4c2.5-2.8 6-6.6 10.2-10.3c8.8-7.8 18.8-14 27.7-14s18.9 6.2 27.7 14c4.2 3.7 7.7 7.5 10.2 10.3c1.2 1.4 2.2 2.6 2.8 3.4c.3 .4 .6 .7 .7 .9l.2 .2 0 0 0 0z"
        /> < title > { title } < / title > < / svg >
    }
}
