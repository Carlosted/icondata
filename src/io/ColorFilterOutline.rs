#[cfg(feature = "IoColorFilterOutline")]
use leptos::*;
#[cfg(feature = "IoColorFilterOutline")]
///This icon requires the feature `IoColorFilterOutline` to be enabled.
#[component]
pub fn ColorFilterOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "184" r = "120" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< circle xmlns
        = "http://www.w3.org/2000/svg" cx = "344" cy = "328" r = "120" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< circle xmlns
        = "http://www.w3.org/2000/svg" cx = "168" cy = "328" r = "120" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
