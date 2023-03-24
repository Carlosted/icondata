#[cfg(feature = "HiMdSolidCloudArrowDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidCloudArrowDown")]
///This icon requires the feature `HiMdSolidCloudArrowDown` to be enabled.
#[component]
pub fn CloudArrowDown(
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
        "M5.5 17C3.01472 17 1 14.9853 1 12.5C1 10.5184 2.28084 8.83595 4.05979 8.2354C4.02046 7.9961 4 7.75044 4 7.5C4 5.01472 6.01472 3 8.5 3C10.1404 3 11.5758 3.87771 12.3621 5.18913C12.7189 5.06655 13.1017 5 13.5 5C15.433 5 17 6.567 17 8.5C17 8.83334 16.9534 9.15579 16.8664 9.4612C18.1353 10.1318 19 11.4649 19 13C19 15.2091 17.2091 17 15 17H5.5ZM10.75 7.75C10.75 7.33579 10.4142 7 10 7C9.58579 7 9.25 7.33579 9.25 7.75V12.3401L7.29959 10.2397C7.01774 9.93613 6.54319 9.91855 6.23966 10.2004C5.93613 10.4823 5.91855 10.9568 6.20041 11.2603L9.45041 14.7603C9.59231 14.9132 9.79145 15 10 15C10.2086 15 10.4077 14.9132 10.5496 14.7603L13.7996 11.2603C14.0814 10.9568 14.0639 10.4823 13.7603 10.2004C13.4568 9.91855 12.9823 9.93613 12.7004 10.2397L10.75 12.3401V7.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
