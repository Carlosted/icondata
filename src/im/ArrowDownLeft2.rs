#[cfg(feature = "ImArrowDownLeft2")]
use leptos::*;
#[cfg(feature = "ImArrowDownLeft2")]
///This icon requires the feature `ImArrowDownLeft2` to be enabled.
#[component]
pub fn ArrowDownLeft2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.293 2.293l-8.293 8.293v-3.586c0-0.552-0.448-1-1-1s-1 0.448-1 1v6c0 0.404 0.244 0.769 0.617 0.924 0.124 0.051 0.254 0.076 0.383 0.076v0.001l6-0c0.552 0 1-0.448 1-1s-0.448-1-1-1h-3.586l8.293-8.293c0.195-0.195 0.293-0.451 0.293-0.707s-0.098-0.512-0.293-0.707c-0.39-0.391-1.024-0.391-1.414 0v0z"
        /> < title > { title } < / title > < / svg >
    }
}
