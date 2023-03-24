#[cfg(feature = "SiBbc")]
use leptos::*;
#[cfg(feature = "SiBbc")]
///This icon requires the feature `SiBbc` to be enabled.
#[component]
pub fn Bbc(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.004 13c0 .744-.925.7-.925.7h-.925v-1.343h.925c.952-.007.925.644.925.644m-1.85-2.693h.704c.732.04.705.584.705.584 0 .677-.81.688-.81.688h-.6zm1.679 1.536s.633-.27.627-.985c0 0 .096-1.173-1.458-1.316h-1.724v4.917h1.977s1.65.004 1.65-1.388c0 0 .04-.947-1.072-1.228M8.37 8.58h7.258v6.84H8.371zM4.633 13c0 .744-.925.7-.925.7h-.925v-1.343h.925c.952-.007.925.644.925.644m-1.85-2.693h.705c.732.04.704.584.704.584 0 .677-.81.688-.81.688h-.599zm1.679 1.536s.633-.27.627-.985c0 0 .097-1.173-1.457-1.316H1.908v4.917h1.976s1.651.004 1.651-1.388c0 0 .04-.947-1.073-1.228M0 8.58h7.259v6.84H0zm22.52 1.316v.908s-.887-.545-1.867-.556c0 0-1.828-.036-1.91 1.752 0 0-.066 1.645 1.888 1.738 0 0 .82.099 1.932-.61v.94s-1.492.887-3.22.204c0 0-1.454-.53-1.509-2.272 0 0-.06-1.79 1.878-2.385 0 0 .517-.198 1.447-.11 0 0 .556.055 1.36.39m-5.778 5.525H24V8.58h-7.259Z"
        /> < title > { title } < / title > < / svg >
    }
}
