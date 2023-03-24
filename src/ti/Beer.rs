#[cfg(feature = "TiBeer")]
use leptos::*;
#[cfg(feature = "TiBeer")]
///This icon requires the feature `TiBeer` to be enabled.
#[component]
pub fn Beer(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M10 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM12 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM14 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM18.5 6h-.5v-1c0-1.104-.896-2-2-2h-9c-1.104 0-2 .896-2 2v13c0 1.656 1.344 3 3 3h7c1.656 0 3-1.344 3-3h.5c1.93 0 3.5-1.57 3.5-3.5v-5c0-1.93-1.57-3.5-3.5-3.5zm-11.5-1h9v1h-4.444l-.118.332c-.164.458-.663.73-1.117.648l-.348-.058-.173.307c-.267.475-.765.771-1.3.771-.827 0-1.5-.673-1.5-1.5v-1.5zm9 13c0 .552-.448 1-1 1h-7c-.552 0-1-.448-1-1v-9.51c.419.317.936.51 1.5.51.784 0 1.521-.376 1.989-1 .728 0 1.383-.391 1.736-1h3.775v11zm4-3.5c0 .827-.673 1.5-1.5 1.5h-1.5v-8h1.5c.827 0 1.5.673 1.5 1.5v5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
