#[cfg(feature = "FaSolidHouseMedicalCircleXmark")]
use leptos::*;
#[cfg(feature = "FaSolidHouseMedicalCircleXmark")]
///This icon requires the feature `FaSolidHouseMedicalCircleXmark` to be enabled.
#[component]
pub fn HouseMedicalCircleXmark(
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
        "M320 368c0 59.5 29.5 112.1 74.8 144H128.1c-35.3 0-64-28.7-64-64V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L522.1 193.9c-8.5-1.3-17.3-1.9-26.1-1.9c-54.7 0-103.5 24.9-135.8 64H320V208c0-8.8-7.2-16-16-16H272c-8.8 0-16 7.2-16 16v48H208c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h48v48c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16zM496 224a144 144 0 1 1 0 288 144 144 0 1 1 0-288zm22.6 144l36.7-36.7c6.2-6.2 6.2-16.4 0-22.6s-16.4-6.2-22.6 0L496 345.4l-36.7-36.7c-6.2-6.2-16.4-6.2-22.6 0s-6.2 16.4 0 22.6L473.4 368l-36.7 36.7c-6.2 6.2-6.2 16.4 0 22.6s16.4 6.2 22.6 0L496 390.6l36.7 36.7c6.2 6.2 16.4 6.2 22.6 0s6.2-16.4 0-22.6L518.6 368z"
        /> < title > { title } < / title > < / svg >
    }
}
