#[cfg(feature = "IoHourglassOutline")]
use leptos::*;
#[cfg(feature = "IoHourglassOutline")]
///This icon requires the feature `IoHourglassOutline` to be enabled.
#[component]
pub fn HourglassOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M145.61,464H366.39c19.8,0,35.55-16.29,33.42-35.06C386.06,308,304,310,304,256s83.11-51,95.8-172.94c2-18.78-13.61-35.06-33.41-35.06H145.61c-19.8,0-35.37,16.28-33.41,35.06C124.89,205,208,201,208,256s-82.06,52-95.8,172.94C110.06,447.71,125.81,464,145.61,464Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M343.3,432H169.13c-15.6,0-20-18-9.06-29.16C186.55,376,240,356.78,240,326V224c0-19.85-38-35-61.51-67.2-3.88-5.31-3.49-12.8,6.37-12.8H327.59c8.41,0,10.23,7.43,6.4,12.75C310.82,189,272,204.05,272,224V326c0,30.53,55.71,47,80.4,76.87C362.35,414.91,358.87,432,343.3,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
