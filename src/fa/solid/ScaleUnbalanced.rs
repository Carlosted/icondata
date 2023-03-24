#[cfg(feature = "FaSolidScaleUnbalanced")]
use leptos::*;
#[cfg(feature = "FaSolidScaleUnbalanced")]
///This icon requires the feature `FaSolidScaleUnbalanced` to be enabled.
#[component]
pub fn ScaleUnbalanced(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M521.1 62.4c16.8-5.6 25.8-23.7 20.2-40.5S517.6-3.9 500.9 1.6l-113 37.7C374 15.8 348.3 0 319 0c-44.2 0-80 35.8-80 80c0 3 .2 5.9 .5 8.8L116.9 129.6c-16.8 5.6-25.8 23.7-20.2 40.5s23.7 25.8 40.5 20.2l135.5-45.2c4.5 3.2 9.3 5.9 14.4 8.2V480c0 17.7 14.3 32 32 32H511c17.7 0 32-14.3 32-32s-14.3-32-32-32H351V153.3c21-9.2 37.2-27 44.2-49l125.9-42zM438.6 288L511 163.8 583.4 288H438.6zM511 384c62.9 0 115.2-34 126-78.9c2.6-11-1-22.3-6.7-32.1L535.1 109.8c-5-8.6-14.2-13.8-24.1-13.8s-19.1 5.3-24.1 13.8L391.7 273.1c-5.7 9.8-9.3 21.1-6.7 32.1C395.8 350 448.1 384 511 384zM128.2 291.8L200.6 416H55.7l72.4-124.2zM2.2 433.1C13 478 65.3 512 128.2 512s115.2-34 126-78.9c2.6-11-1-22.3-6.7-32.1L152.2 237.8c-5-8.6-14.2-13.8-24.1-13.8s-19.1 5.3-24.1 13.8L8.9 401.1c-5.7 9.8-9.3 21.1-6.7 32.1z"
        /> < title > { title } < / title > < / svg >
    }
}
