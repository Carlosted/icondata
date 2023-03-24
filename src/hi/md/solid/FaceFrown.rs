#[cfg(feature = "HiMdSolidFaceFrown")]
use leptos::*;
#[cfg(feature = "HiMdSolidFaceFrown")]
///This icon requires the feature `HiMdSolidFaceFrown` to be enabled.
#[component]
pub fn FaceFrown(
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
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM6.46447 14.5251C6.75736 14.818 7.23223 14.818 7.52513 14.5251C8.89196 13.1583 11.108 13.1583 12.4749 14.5251C12.7678 14.818 13.2426 14.818 13.5355 14.5251C13.8284 14.2323 13.8284 13.7574 13.5355 13.4645C11.5829 11.5119 8.41709 11.5119 6.46447 13.4645C6.17157 13.7574 6.17157 14.2323 6.46447 14.5251ZM9 8.5C9 9.32843 8.55228 10 8 10C7.44772 10 7 9.32843 7 8.5C7 7.67157 7.44772 7 8 7C8.55228 7 9 7.67157 9 8.5ZM12 10C12.5523 10 13 9.32843 13 8.5C13 7.67157 12.5523 7 12 7C11.4477 7 11 7.67157 11 8.5C11 9.32843 11.4477 10 12 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
