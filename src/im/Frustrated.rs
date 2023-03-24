#[cfg(feature = "ImFrustrated")]
use leptos::*;
#[cfg(feature = "ImFrustrated")]
///This icon requires the feature `ImFrustrated` to be enabled.
#[component]
pub fn Frustrated(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M5.724 4.428c-0.543-0.271-1.080-0.407-1.102-0.413-0.268-0.067-0.539 0.096-0.606 0.364s0.096 0.539 0.364 0.606c0.275 0.070 0.602 0.189 0.89 0.334-0.166 0.179-0.268 0.418-0.268 0.681 0 0.552 0.448 1 1 1s1-0.448 1-1c0-0.018-0.001-0.036-0.002-0.054-0.032-0.741-0.706-1.234-1.275-1.518z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM3.695 12.87c0.167 0.083 0.356 0.13 0.555 0.13h7.5c0.199 0 0.387-0.047 0.555-0.13-1.147 1.014-2.654 1.63-4.305 1.63s-3.158-0.616-4.305-1.63zM4 11.75v-1.5c0-0.136 0.114-0.25 0.25-0.25h1.75v2h-1.75c-0.136 0-0.25-0.114-0.25-0.25zM7 12v-2h2v2h-2zM10 12v-2h1.75c0.136 0 0.25 0.114 0.25 0.25v1.5c0 0.136-0.114 0.25-0.25 0.25h-1.75zM12.87 12.305c0.083-0.167 0.13-0.356 0.13-0.555v-1.5c0-0.689-0.561-1.25-1.25-1.25h-7.5c-0.689 0-1.25 0.561-1.25 1.25v1.5c0 0.199 0.047 0.387 0.13 0.555-1.014-1.147-1.63-2.654-1.63-4.305 0-3.59 2.91-6.5 6.5-6.5s6.5 2.91 6.5 6.5c0 1.651-0.616 3.158-1.63 4.305z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.379 4.015c-0.023 0.006-0.559 0.141-1.102 0.413-0.568 0.284-1.243 0.776-1.275 1.518-0.001 0.018-0.002 0.036-0.002 0.054 0 0.552 0.448 1 1 1s1-0.448 1-1c0-0.263-0.102-0.503-0.268-0.681 0.288-0.144 0.614-0.264 0.89-0.334 0.268-0.067 0.431-0.338 0.364-0.606s-0.338-0.431-0.606-0.364z"
        /> < title > { title } < / title > < / svg >
    }
}
