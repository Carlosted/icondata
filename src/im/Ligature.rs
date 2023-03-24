#[cfg(feature = "ImLigature")]
use leptos::*;
#[cfg(feature = "ImLigature")]
///This icon requires the feature `ImLigature` to be enabled.
#[component]
pub fn Ligature(
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
        "M12 13.622c0-0.001 0-0.001 0-0.002l-0.005-6.821-1.992 0.097h-3.936v-0.336c0-1.274 0.091-2.546 0.269-3.042 0.123-0.343 0.353-0.652 0.683-0.919 0.322-0.261 0.643-0.393 0.955-0.393 0.262 0 0.48 0.045 0.647 0.134 0.235 0.134 0.464 0.359 0.682 0.669 0.577 0.82 0.812 1.038 0.939 1.131 0.216 0.158 0.477 0.238 0.776 0.238 0.292 0 0.546-0.109 0.757-0.324 0.209-0.213 0.315-0.479 0.315-0.792 0-0.335-0.139-0.691-0.414-1.057-0.268-0.358-0.683-0.652-1.232-0.875-0.536-0.218-1.14-0.329-1.793-0.329-0.949 0-1.813 0.228-2.568 0.678-0.757 0.451-1.337 1.077-1.725 1.863-0.359 0.728-0.333 2.105-0.355 3.355h-1.965v1.116h1.962v5.073c0 1.12-0.342 1.422-0.472 1.583-0.179 0.222-0.509 0.455-0.944 0.455h-0.604v0.878h6.021v-0.878h-0.105c-1.424 0-1.828-0.154-1.828-1.888 0-0 0-0.001 0-0.001l-0.001-5.222h2.191c1.163 0 1.43 0.054 1.491 0.077 0.074 0.028 0.169 0.075 0.204 0.143 0.014 0.026 0.081 0.391 0.081 1.296v3.917c0 0.913-0.111 1.217-0.179 1.319-0.145 0.222-0.319 0.345-0.854 0.358v0.879h4.588v-0.873c-1.431 0-1.588-0.153-1.588-1.505z"
        /> < title > { title } < / title > < / svg >
    }
}
