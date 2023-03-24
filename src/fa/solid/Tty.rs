#[cfg(feature = "FaSolidTty")]
use leptos::*;
#[cfg(feature = "FaSolidTty")]
///This icon requires the feature `FaSolidTty` to be enabled.
#[component]
pub fn Tty(
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
        "M29.3 241.3L6.1 200.6c-9.2-16.2-8.4-36.5 4.5-50C52.4 106.8 135.7 48 247 48s194.6 58.8 236.4 102.6c12.9 13.5 13.7 33.8 4.5 50l-23.1 40.7c-7.5 13.2-23.3 19.3-37.8 14.6l-81.1-26.6c-13.1-4.3-22-16.6-22-30.4V144c-49.6-18.1-104-18.1-153.6 0v54.8c0 13.8-8.9 26.1-22 30.4L67.1 255.8c-14.5 4.7-30.3-1.4-37.8-14.6zM23 336c0-8.8 7.2-16 16-16H71c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H39c-8.8 0-16-7.2-16-16V336zm0 96c0-8.8 7.2-16 16-16H71c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H39c-8.8 0-16-7.2-16-16V432zM135 320h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H135c-8.8 0-16-7.2-16-16V336c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H231c-8.8 0-16-7.2-16-16V336zm112-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H327c-8.8 0-16-7.2-16-16V336c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H423c-8.8 0-16-7.2-16-16V336zm16 80h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H423c-8.8 0-16-7.2-16-16V432c0-8.8 7.2-16 16-16zM119 432c0-8.8 7.2-16 16-16H359c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H135c-8.8 0-16-7.2-16-16V432z"
        /> < title > { title } < / title > < / svg >
    }
}
