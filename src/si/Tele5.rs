#[cfg(feature = "SiTele5")]
use leptos::*;
#[cfg(feature = "SiTele5")]
///This icon requires the feature `SiTele5` to be enabled.
#[component]
pub fn Tele5(
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
        "M6.441.682v12.894h9l-9.425 4.11a9.383 9.383 0 0012.351 4.85v.003a9.383 9.383 0 004.85-12.353l-7.694 3.353V8.293h7.5V.682zM.001.688v5.216h5.452V.688H3.607v2.484h-.439V.688h-.88v2.484h-.442V.688zm2.726 5.689v2.236H.002v2.723h5.453v-4.96zm.878 5.428v2.488h-.437v-2.486h-.88v2.486h-.442v-2.486H0v5.216h5.453v-5.218zM.002 17.467V23.2h2.732v-1.49h2.721v-2.703h-2.72v-1.54H.001z"
        /> < title > { title } < / title > < / svg >
    }
}
