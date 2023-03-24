#[cfg(feature = "CgRing")]
use leptos::*;
#[cfg(feature = "CgRing")]
///This icon requires the feature `CgRing` to be enabled.
#[component]
pub fn Ring(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6.34326 3.68567C6.83159 3.19735 7.36734 2.78535 7.93569 2.44968C10.4352 0.97348 13.5651 0.97348 16.0645 2.44968C16.6329 2.78535 17.1686 3.19735 17.657 3.68567L12.0001 9.34253L6.34326 3.68567ZM12.0001 6.5141L9.4131 3.92709C11.0447 3.14768 12.9556 3.14768 14.5871 3.92709L12.0001 6.5141Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 12.6576C2 9.52653 3.43899 6.73156 5.69172 4.89796L7.11528 6.32152C5.22043 7.78449 4 10.0785 4 12.6576C4 17.0759 7.58172 20.6576 12 20.6576C16.4183 20.6576 20 17.0759 20 12.6576C20 10.0785 18.7796 7.78451 16.8847 6.32154L18.3083 4.89798C20.561 6.73158 22 9.52654 22 12.6576C22 18.1804 17.5228 22.6576 12 22.6576C6.47715 22.6576 2 18.1804 2 12.6576Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
