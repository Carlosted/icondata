#[cfg(feature = "BiRegularCaptions")]
use leptos::*;
#[cfg(feature = "BiRegularCaptions")]
///This icon requires the feature `BiRegularCaptions` to be enabled.
#[component]
pub fn Captions(
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
        "M6 10v4c0 1.103.897 2 2 2h3v-2H8v-4h3V8H8c-1.103 0-2 .897-2 2zm7 0v4c0 1.103.897 2 2 2h3v-2h-3v-4h3V8h-3c-1.103 0-2 .897-2 2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 4H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zM4 18V6h16l.002 12H4z"
        /> < title > { title } < / title > < / svg >
    }
}
