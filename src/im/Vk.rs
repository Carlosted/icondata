#[cfg(feature = "ImVk")]
use leptos::*;
#[cfg(feature = "ImVk")]
///This icon requires the feature `ImVk` to be enabled.
#[component]
pub fn Vk(
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
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM12.959 11.2l-1.463 0.022c0 0-0.316 0.063-0.728-0.222-0.547-0.375-1.063-1.353-1.466-1.225-0.406 0.128-0.394 1.006-0.394 1.006s0.003 0.188-0.091 0.287c-0.1 0.109-0.3 0.131-0.3 0.131h-0.653c0 0-1.444 0.088-2.716-1.238-1.388-1.444-2.612-4.309-2.612-4.309s-0.072-0.188 0.006-0.278c0.087-0.103 0.322-0.109 0.322-0.109l1.566-0.009c0 0 0.147 0.025 0.253 0.103 0.088 0.063 0.134 0.184 0.134 0.184s0.253 0.641 0.588 1.219c0.653 1.128 0.959 1.375 1.181 1.256 0.322-0.175 0.225-1.597 0.225-1.597s0.006-0.516-0.162-0.744c-0.131-0.178-0.378-0.231-0.484-0.244-0.088-0.013 0.056-0.216 0.244-0.309 0.281-0.138 0.778-0.147 1.366-0.141 0.456 0.003 0.591 0.034 0.769 0.075 0.541 0.131 0.356 0.634 0.356 1.841 0 0.388-0.069 0.931 0.209 1.109 0.119 0.078 0.412 0.012 1.147-1.234 0.347-0.591 0.609-1.284 0.609-1.284s0.056-0.125 0.144-0.178c0.091-0.053 0.213-0.037 0.213-0.037l1.647-0.009c0 0 0.494-0.059 0.575 0.166 0.084 0.234-0.184 0.781-0.856 1.678-1.103 1.472-1.228 1.334-0.309 2.184 0.875 0.813 1.056 1.209 1.088 1.259 0.356 0.6-0.406 0.647-0.406 0.647z"
        /> < title > { title } < / title > < / svg >
    }
}
