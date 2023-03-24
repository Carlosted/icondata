#[cfg(feature = "HiMdSolidWrench")]
use leptos::*;
#[cfg(feature = "HiMdSolidWrench")]
///This icon requires the feature `HiMdSolidWrench` to be enabled.
#[component]
pub fn Wrench(
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
        "M19 5.5C19 7.98528 16.9853 10 14.5 10C14.4022 10 14.3051 9.99688 14.2088 9.99073C13.3358 9.93497 12.4014 10.1183 11.8414 10.7903L5.81681 18.0198C5.29925 18.6409 4.53256 19 3.7241 19C2.21962 19 1 17.7804 1 16.2759C1 15.4674 1.3591 14.7008 1.98017 14.1832L9.20974 8.15855C9.88173 7.59855 10.065 6.66418 10.0093 5.79122C10.0031 5.69494 10 5.59783 10 5.5C10 3.01472 12.0147 1 14.5 1C14.9823 1 15.4469 1.07588 15.8825 1.21636C16.2067 1.32092 16.2735 1.72672 16.0327 1.9676L13.3398 4.66042C13.2094 4.79088 13.1582 4.98403 13.2292 5.15431C13.5334 5.88351 14.1172 6.46695 14.8466 6.77074C15.0168 6.84163 15.2098 6.79041 15.3402 6.66002L18.0325 3.96772C18.2734 3.72683 18.6792 3.79367 18.7838 4.11791C18.9242 4.55338 19 5.01783 19 5.5ZM4 17C4.55228 17 5 16.5523 5 16C5 15.4477 4.55228 15 4 15C3.44772 15 3 15.4477 3 16C3 16.5523 3.44772 17 4 17Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
