#[cfg(feature = "HiMdSolidTruck")]
use leptos::*;
#[cfg(feature = "HiMdSolidTruck")]
///This icon requires the feature `HiMdSolidTruck` to be enabled.
#[component]
pub fn Truck(
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
        "M6.5 3C5.4488 3 4.40669 3.03958 3.37512 3.11734C2.58952 3.17656 2 3.83452 2 4.60628V10.5H11V4.60628C11 3.83452 10.4105 3.17656 9.62488 3.11734C8.59331 3.03958 7.5512 3 6.5 3Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 12V14.5C2 15.3284 2.67157 16 3.5 16H3.54148C3.77952 14.5811 5.0135 13.5 6.5 13.5C7.9865 13.5 9.22048 14.5811 9.45852 16H10.25C10.6642 16 11 15.6642 11 15.25V12H2Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 18C7.32843 18 8 17.3284 8 16.5C8 15.6716 7.32843 15 6.5 15C5.67157 15 5 15.6716 5 16.5C5 17.3284 5.67157 18 6.5 18Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.25 5C12.8358 5 12.5 5.33579 12.5 5.75V14.2639C13.0308 13.7889 13.7316 13.5 14.5 13.5C15.8814 13.5 17.0447 14.4336 17.3933 15.7043C17.7639 15.4289 18.0037 14.9852 17.9883 14.4772C17.8967 11.4639 17.2717 8.58345 16.204 5.92808C15.9743 5.35688 15.4206 5 14.8229 5H13.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.5 18C15.3284 18 16 17.3284 16 16.5C16 15.6716 15.3284 15 14.5 15C13.6716 15 13 15.6716 13 16.5C13 17.3284 13.6716 18 14.5 18Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
