#[cfg(feature = "HiMdSolidArrowLongLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowLongLeft")]
///This icon requires the feature `HiMdSolidArrowLongLeft` to be enabled.
#[component]
pub fn ArrowLongLeft(
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
        "M18 10C18 10.4142 17.6642 10.75 17.25 10.75L4.6599 10.75L6.76034 12.7004C7.06387 12.9823 7.08145 13.4568 6.79959 13.7603C6.51774 14.0639 6.04319 14.0815 5.73966 13.7996L2.23966 10.5496C2.08684 10.4077 2 10.2086 2 10C2 9.79145 2.08684 9.59232 2.23966 9.45041L5.73966 6.20041C6.0432 5.91856 6.51774 5.93613 6.79959 6.23966C7.08145 6.5432 7.06387 7.01775 6.76034 7.2996L4.6599 9.25L17.25 9.25C17.6642 9.25 18 9.58579 18 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
