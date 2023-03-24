#[cfg(feature = "HiMdSolidLink")]
use leptos::*;
#[cfg(feature = "HiMdSolidLink")]
///This icon requires the feature `HiMdSolidLink` to be enabled.
#[component]
pub fn Link(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.2322 4.23223C13.2085 3.25592 14.7915 3.25592 15.7678 4.23223C16.7441 5.20854 16.7441 6.79146 15.7678 7.76777L14.5434 8.9921C14.2505 9.28499 14.2505 9.75987 14.5434 10.0528C14.8363 10.3457 15.3112 10.3457 15.6041 10.0528L16.8284 8.82843C18.3905 7.26633 18.3905 4.73367 16.8284 3.17157C15.2663 1.60948 12.7337 1.60948 11.1716 3.17157L8.17157 6.17157C6.60948 7.73367 6.60948 10.2663 8.17157 11.8284C8.24449 11.9013 8.31963 11.9709 8.3968 12.0372C8.71107 12.307 9.18457 12.271 9.45441 11.9567C9.72424 11.6425 9.68822 11.169 9.37395 10.8991C9.32556 10.8576 9.27828 10.8138 9.23223 10.7678C8.25592 9.79145 8.25592 8.20854 9.23223 7.23223L12.2322 4.23223Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.6032 7.96278C11.2889 7.69295 10.8154 7.72897 10.5456 8.04324C10.2758 8.3575 10.3118 8.83101 10.626 9.10084C10.6744 9.14239 10.7217 9.18618 10.7678 9.23222C11.7441 10.2085 11.7441 11.7914 10.7678 12.7678L7.76777 15.7678C6.79146 16.7441 5.20854 16.7441 4.23223 15.7678C3.25592 14.7914 3.25592 13.2085 4.23223 12.2322L5.45657 11.0079C5.74946 10.715 5.74946 10.2401 5.45657 9.94723C5.16367 9.65433 4.6888 9.65433 4.39591 9.94723L3.17157 11.1716C1.60948 12.7337 1.60948 15.2663 3.17157 16.8284C4.73367 18.3905 7.26633 18.3905 8.82843 16.8284L11.8284 13.8284C13.3905 12.2663 13.3905 9.73366 11.8284 8.17156C11.7555 8.09864 11.6804 8.02904 11.6032 7.96278Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
