#[cfg(feature = "HiMdSolidArrowSmallLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowSmallLeft")]
///This icon requires the feature `HiMdSolidArrowSmallLeft` to be enabled.
#[component]
pub fn ArrowSmallLeft(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15 10C15 10.4142 14.6642 10.75 14.25 10.75H7.61208L9.76983 12.7094C10.0684 12.9965 10.0777 13.4713 9.79062 13.7698C9.50353 14.0684 9.02875 14.0777 8.73017 13.7906L5.23017 10.5406C5.08311 10.3992 5 10.204 5 10C5 9.79599 5.08311 9.60078 5.23017 9.45938L8.73017 6.20938C9.02875 5.92228 9.50353 5.93159 9.79062 6.23017C10.0777 6.52875 10.0684 7.00353 9.76983 7.29063L7.61208 9.25L14.25 9.25C14.6642 9.25 15 9.58579 15 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
