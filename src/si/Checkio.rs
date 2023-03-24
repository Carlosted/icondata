#[cfg(feature = "SiCheckio")]
use leptos::*;
#[cfg(feature = "SiCheckio")]
///This icon requires the feature `SiCheckio` to be enabled.
#[component]
pub fn Checkio(
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
        "M10.846 8.886L24 2.932v13.82L9.621 21.068 0 14.09l3.35-9.956 7.496 4.751v.001zm-4.582 2.067l3.923-1.768-6.065-3.85 2.142 5.618zm-5.393 2.44l4.842-2.187-2.179-5.717-2.662 7.904H.871zm22.526 2.54V4.256l-5.96 7.37 5.96 4.307zm-12.865 4.233l12.497-3.758-5.973-4.316-6.524 8.074zM.94 14.029l8.092 5.867-3.106-8.124L.94 14.029zm21.722-9.826c-5.085 2.296-10.163 4.6-15.25 6.895l9.445.284 5.805-7.178v-.001zM9.775 20.143l6.608-8.173-9.844-.29 3.236 8.462v.001z"
        /> < title > { title } < / title > < / svg >
    }
}
