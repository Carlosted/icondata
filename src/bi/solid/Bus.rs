#[cfg(feature = "BiSolidBus")]
use leptos::*;
#[cfg(feature = "BiSolidBus")]
///This icon requires the feature `BiSolidBus` to be enabled.
#[component]
pub fn Bus(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21 6.021c.003-.146-.007-1.465-1.3-2.735C18.427 2.036 17.143 2 17 2H6.996c-.239 0-1.493.063-2.708 1.302C3.036 4.578 3 5.859 3 6v3H2v3h1v6c0 .734.406 1.373 1 1.721V21a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1h10v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1.277A1.99 1.99 0 0 0 21 18v-6h1V9h-1V6.021zM9 4h6v2H9V4zM6.5 18a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 6.5 18zm4.5-5H5V8h6v5zm6.5 5a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 17.5 18zm1.5-5h-6V8h6v5z"
        /> < title > { title } < / title > < / svg >
    }
}
