#[cfg(feature = "HiMdSolidBeaker")]
use leptos::*;
#[cfg(feature = "HiMdSolidBeaker")]
///This icon requires the feature `HiMdSolidBeaker` to be enabled.
#[component]
pub fn Beaker(
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
        "M8.49999 3.52795V8.17157C8.49999 8.90092 8.21026 9.60039 7.69453 10.1161L6.47812 11.3325C7.68604 11.2911 8.89515 11.5003 10.0275 11.9532L10.5296 12.1541C11.8561 12.6847 13.3098 12.8115 14.708 12.5187L12.3054 10.1161C11.7897 9.60039 11.5 8.90092 11.5 8.17157V3.52795C11.0023 3.50937 10.5023 3.5 9.99999 3.5C9.49771 3.5 8.99766 3.50937 8.49999 3.52795ZM13 3.61218C13.0635 3.61695 13.1269 3.62186 13.1903 3.62693C13.6032 3.65992 13.9646 3.35194 13.9976 2.93905C14.0306 2.52615 13.7226 2.16468 13.3097 2.13169C12.9714 2.10466 12.6319 2.08173 12.2913 2.06296C11.5327 2.02117 10.7688 2 9.99999 2C9.23118 2 8.46724 2.02117 7.70873 2.06296C7.36812 2.08173 7.02862 2.10466 6.69025 2.13169C6.27736 2.16468 5.96938 2.52615 6.00237 2.93905C6.03536 3.35194 6.39683 3.65992 6.80973 3.62693C6.8731 3.62186 6.93653 3.61695 6.99999 3.61218V8.17157C6.99999 8.50309 6.86829 8.82104 6.63387 9.05546L2.60034 13.089C1.10385 14.5855 1.78334 17.2391 4.00336 17.5645C5.9611 17.8515 7.96343 18 9.99999 18C12.0366 18 14.0389 17.8515 15.9966 17.5645C18.2166 17.2391 18.8961 14.5855 17.3996 13.089L13.3661 9.05546C13.1317 8.82104 13 8.50309 13 8.17157V3.61218Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
