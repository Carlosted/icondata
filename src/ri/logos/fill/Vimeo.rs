#[cfg(feature = "RiLogosFillVimeo")]
use leptos::*;
#[cfg(feature = "RiLogosFillVimeo")]
///This icon requires the feature `RiLogosFillVimeo` to be enabled.
#[component]
pub fn Vimeo(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M1.173 8.301c-.281-.413-.252-.413.328-.922 1.232-1.082 2.394-2.266 3.736-3.212 1.215-.852 2.826-1.402 3.927-.047 1.014 1.249 1.038 3.142 1.295 4.65.257 1.564.503 3.164 1.051 4.66.152.421.443 1.217.968 1.284.678.093 1.368-1.096 1.683-1.54.817-1.18 1.925-2.769 1.785-4.286-.138-1.612-1.878-1.309-2.966-.924.175-1.809 1.858-3.843 3.48-4.53 1.72-.714 4.276-.702 5.14 1.237.923 2.102.093 4.543-.912 6.448-1.097 2.068-2.509 3.982-4.018 5.77-1.331 1.588-2.906 3.33-4.89 4.089-2.267.864-3.61-.82-4.382-2.77-.843-2.123-1.262-4.506-1.87-6.717-.256-.934-.56-1.997-1.167-2.768-.792-.995-1.692-.06-2.474.477-.269-.267-.491-.607-.714-.899z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
