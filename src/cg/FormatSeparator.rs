#[cfg(feature = "CgFormatSeparator")]
use leptos::*;
#[cfg(feature = "CgFormatSeparator")]
///This icon requires the feature `CgFormatSeparator` to be enabled.
#[component]
pub fn FormatSeparator(
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
        "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" opacity =
        "0.5" >< path d =
        "M16 5C16.5523 5 17 4.55229 17 4C17 3.44772 16.5523 3 16 3H8C7.44771 3 7 3.44772 7 4C7 4.55228 7.44771 5 8 5L16 5Z"
        fill = "currentColor" />< path d =
        "M16 7C16.5523 7 17 7.44772 17 8C17 8.55229 16.5523 9 16 9H8C7.44771 9 7 8.55229 7 8C7 7.44772 7.44771 7 8 7H16Z"
        fill = "currentColor" />< path d =
        "M17 12C17 12.5523 16.5523 13 16 13L8 13C7.44771 13 7 12.5523 7 12C7 11.4477 7.44771 11 8 11L16 11C16.5523 11 17 11.4477 17 12Z"
        fill = "currentColor" />< path d =
        "M16 21C16.5523 21 17 20.5523 17 20C17 19.4477 16.5523 19 16 19L8 19C7.44771 19 7 19.4477 7 20C7 20.5523 7.44771 21 8 21H16Z"
        fill = "currentColor" /></ g >< path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M21 16C21 16.5523 20.5523 17 20 17L4 17C3.44772 17 3 16.5523 3 16C3 15.4477 3.44772 15 4 15L20 15C20.5523 15 21 15.4477 21 16Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
