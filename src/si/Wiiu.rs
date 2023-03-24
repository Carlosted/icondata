#[cfg(feature = "SiWiiu")]
use leptos::*;
#[cfg(feature = "SiWiiu")]
///This icon requires the feature `SiWiiu` to be enabled.
#[component]
pub fn Wiiu(
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
        "M11.133 8.432c-.465 0-.835.359-.835.814.007.454.381.817.835.812.488 0 .873-.358.873-.81 0-.455-.385-.816-.873-.816zm2.93 0c-.465 0-.848.359-.848.814 0 .442.383.812.848.812.477 0 .861-.358.861-.81 0-.455-.385-.816-.861-.816zm3.655.011c-.53 0-.99.335-.99.858v3.315c0 .809.56 1.289 1.377 1.289h4.647c.689 0 1.248-.477 1.248-1.162V9.345c0-.412-.308-.86-.688-.86h-1.075v2.799c0 2.122-3.701 2.104-3.701.04v-2.88h-.818zm1.679 0v2.453c0 1.636 1.934 1.402 1.934.256V8.445h-1.934v-.002zM4.833 8.77c-.465 0-.776.232-.938.756-.152.533-1.116 4.242-1.116 4.242l-1.267-4.94H0s1.451 5.264 1.65 5.881c.15.476.521.86 1.058.86.627 0 .917-.454 1.045-.86.14-.421 1.08-3.895 1.08-3.895s.942 3.476 1.069 3.895c.14.406.431.86 1.047.86.547 0 .906-.385 1.07-.86.196-.617 1.65-5.881 1.65-5.881H8.148l-1.258 4.94s-.963-3.709-1.125-4.242c-.15-.526-.479-.756-.93-.756h-.002zm5.605 2.09v4.662h1.441V10.86h-1.441zm2.916 0v4.662h1.442V10.86h-1.442z"
        /> < title > { title } < / title > < / svg >
    }
}
