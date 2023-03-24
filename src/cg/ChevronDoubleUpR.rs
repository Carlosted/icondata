#[cfg(feature = "CgChevronDoubleUpR")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleUpR")]
///This icon requires the feature `CgChevronDoubleUpR` to be enabled.
#[component]
pub fn ChevronDoubleUpR(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.8284 12.4813L16.2426 11.067L12 6.82444L7.75732 11.0671L9.17154 12.4813L12 9.65286L14.8284 12.4813Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.8284 16.7239L16.2426 15.3097L12 11.0671L7.75732 15.3097L9.17154 16.7239L12 13.8955L14.8284 16.7239Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M23 4.77411C23 2.56497 21.2091 0.774109 19 0.774109H5C2.79086 0.774109 1 2.56497 1 4.77411V18.7741C1 20.9832 2.79086 22.7741 5 22.7741H19C21.2091 22.7741 23 20.9832 23 18.7741V4.77411ZM19 2.77411H5C3.89543 2.77411 3 3.66954 3 4.77411V18.7741C3 19.8787 3.89543 20.7741 5 20.7741H19C20.1046 20.7741 21 19.8787 21 18.7741V4.77411C21 3.66954 20.1046 2.77411 19 2.77411Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
