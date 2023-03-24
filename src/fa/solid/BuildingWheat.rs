#[cfg(feature = "FaSolidBuildingWheat")]
use leptos::*;
#[cfg(feature = "FaSolidBuildingWheat")]
///This icon requires the feature `FaSolidBuildingWheat` to be enabled.
#[component]
pub fn BuildingWheat(
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
        "M0 48C0 21.5 21.5 0 48 0H336c26.5 0 48 21.5 48 48V464c0 26.5-21.5 48-48 48H240V432c0-26.5-21.5-48-48-48s-48 21.5-48 48v80H48c-26.5 0-48-21.5-48-48V48zM80 224c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V240c0-8.8-7.2-16-16-16H80zm80 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V240c0-8.8-7.2-16-16-16H176c-8.8 0-16 7.2-16 16zm112-16c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V240c0-8.8-7.2-16-16-16H272zM64 112v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V112c0-8.8-7.2-16-16-16H80c-8.8 0-16 7.2-16 16zM176 96c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V112c0-8.8-7.2-16-16-16H176zm80 16v32c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V112c0-8.8-7.2-16-16-16H272c-8.8 0-16 7.2-16 16zm384 80v16c0 44.2-35.8 80-80 80H544V272c0-44.2 35.8-80 80-80h16zm0 128c0 44.2-35.8 80-80 80H544V384c0-44.2 35.8-80 80-80h16v16zm0 112c0 44.2-35.8 80-80 80H544V496c0-44.2 35.8-80 80-80h16v16zM512 496v16H496c-44.2 0-80-35.8-80-80V416h16c44.2 0 80 35.8 80 80zm0-96H496c-44.2 0-80-35.8-80-80V304h16c44.2 0 80 35.8 80 80v16zm0-128v16H496c-44.2 0-80-35.8-80-80V192h16c44.2 0 80 35.8 80 80zM528 32c13.3 0 24 10.7 24 24V160c0 13.3-10.7 24-24 24s-24-10.7-24-24V56c0-13.3 10.7-24 24-24zm96 64v32c0 13.3-10.7 24-24 24s-24-10.7-24-24V96c0-13.3 10.7-24 24-24s24 10.7 24 24zM456 72c13.3 0 24 10.7 24 24v32c0 13.3-10.7 24-24 24s-24-10.7-24-24V96c0-13.3 10.7-24 24-24z"
        /> < title > { title } < / title > < / svg >
    }
}
