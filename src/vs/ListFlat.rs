#[cfg(feature = "VsListFlat")]
use leptos::*;
#[cfg(feature = "VsListFlat")]
///This icon requires the feature `VsListFlat` to be enabled.
#[component]
pub fn ListFlat(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "2" y = "9" width = "9" height = "1" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "2" y = "12" width = "8" height = "1" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "2" y = "6" width = "12" height = "1"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "2" y = "3" width = "11" height
        = "1" /> < title > { title } < / title > < / svg >
    }
}
