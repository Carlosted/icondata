#[cfg(feature = "ImBubbles")]
use leptos::*;
#[cfg(feature = "ImBubbles")]
///This icon requires the feature `ImBubbles` to be enabled.
#[component]
pub fn Bubbles(
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
        stroke_witdh = "0" style = style version = "1.1" width = "18" height = "16"
        viewBox = "0 0 18 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M17 14.081c0 0.711 0.407 1.327 1 1.628v0.249c-0.166 0.023-0.335 0.035-0.508 0.035-1.063 0-2.021-0.446-2.699-1.16-0.41 0.109-0.844 0.168-1.293 0.168-2.485 0-4.5-1.791-4.5-4s2.015-4 4.5-4c2.485 0 4.5 1.791 4.5 4 0 0.865-0.309 1.665-0.834 2.32-0.107 0.232-0.166 0.489-0.166 0.761zM8 0c4.351 0 7.89 2.822 7.997 6.336-0.768-0.343-1.619-0.524-2.497-0.524-1.493 0-2.903 0.523-3.971 1.472-1.107 0.984-1.717 2.304-1.717 3.716 0 0.698 0.149 1.373 0.433 1.997-0.082 0.002-0.164 0.003-0.246 0.003-0.424 0-0.841-0.027-1.247-0.079-1.718 1.718-3.77 2.027-5.753 2.072v-0.421c1.071-0.525 2-1.48 2-2.572 0-0.152-0.012-0.302-0.034-0.448-1.809-1.192-2.966-3.012-2.966-5.052 0-3.59 3.582-6.5 8-6.5z"
        /> < title > { title } < / title > < / svg >
    }
}
