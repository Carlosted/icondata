#[cfg(feature = "HiMdSolidChevronUpDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronUpDown")]
///This icon requires the feature `HiMdSolidChevronUpDown` to be enabled.
#[component]
pub fn ChevronUpDown(
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
        "M10 3C10.2086 3 10.4077 3.08684 10.5496 3.23966L13.7996 6.73966C14.0815 7.0432 14.0639 7.51774 13.7603 7.7996C13.4568 8.08145 12.9823 8.06387 12.7004 7.76034L10 4.85221L7.2996 7.76034C7.01775 8.06387 6.5432 8.08145 6.23966 7.79959C5.93613 7.51774 5.91856 7.04319 6.20041 6.73966L9.45041 3.23966C9.59232 3.08684 9.79145 3 10 3ZM6.23967 12.2004C6.5432 11.9186 7.01775 11.9361 7.2996 12.2397L10 15.1478L12.7004 12.2397C12.9823 11.9361 13.4568 11.9186 13.7603 12.2004C14.0639 12.4823 14.0815 12.9568 13.7996 13.2603L10.5496 16.7603C10.4077 16.9132 10.2086 17 10 17C9.79145 17 9.59232 16.9132 9.45041 16.7603L6.20041 13.2603C5.91856 12.9568 5.93613 12.4823 6.23967 12.2004Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
