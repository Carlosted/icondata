#[cfg(feature = "HiLgOutlineCloudArrowUp")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCloudArrowUp")]
///This icon requires the feature `HiLgOutlineCloudArrowUp` to be enabled.
#[component]
pub fn CloudArrowUp(
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
        "M12 16.5L12 9.75M12 9.75L15 12.75M12 9.75L9 12.75M6.75 19.5C4.26472 19.5 2.25 17.4853 2.25 15C2.25 13.0071 3.54555 11.3167 5.3404 10.7252C5.28105 10.4092 5.25 10.0832 5.25 9.75C5.25 6.85051 7.60051 4.5 10.5 4.5C12.9312 4.5 14.9765 6.1526 15.5737 8.39575C15.8654 8.30113 16.1767 8.25 16.5 8.25C18.1569 8.25 19.5 9.59315 19.5 11.25C19.5 11.5981 19.4407 11.9324 19.3316 12.2433C20.7453 12.7805 21.75 14.1479 21.75 15.75C21.75 17.8211 20.0711 19.5 18 19.5H6.75Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
