#[cfg(feature = "SiSonarcloud")]
use leptos::*;
#[cfg(feature = "SiSonarcloud")]
///This icon requires the feature `SiSonarcloud` to be enabled.
#[component]
pub fn Sonarcloud(
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
        "M22.406 10.9a7.088 7.088 0 00-3.377-2.358v-.085c0-4.035-3.156-7.324-7.047-7.324-3.893 0-7.05 3.282-7.05 7.325v.1C2.081 9.492 0 12.268 0 15.542c0 4.035 3.159 7.325 7.05 7.325a6.907 6.907 0 004.952-2.108 6.885 6.885 0 004.947 2.108c3.884 0 7.051-3.282 7.051-7.325a7.572 7.572 0 00-1.594-4.643zM16.95 21.014c-2.903 0-5.267-2.456-5.267-5.474a.91.91 0 00-.89-.924.906.906 0 00-.892.925c0 1.368.367 2.651.994 3.748a5.156 5.156 0 01-3.845 1.733c-2.904 0-5.27-2.457-5.27-5.474 0-3.016 2.366-5.473 5.27-5.473.63 0 1.241.117 1.827.335.007 0 .013.007.02.007.203.071.489.21.578.287a.858.858 0 001.249-.1.942.942 0 00-.097-1.3c-.39-.342-.995-.575-1.144-.63a6.814 6.814 0 00-2.425-.443c-.113 0-.225 0-.338.007.12-2.916 2.433-5.247 5.27-5.247 2.903 0 5.267 2.456 5.267 5.474a5.569 5.569 0 01-2.215 4.463.948.948 0 00-.21 1.283c.171.25.45.39.727.39a.86.86 0 00.516-.172 7.381 7.381 0 002.709-4.02c2.035.785 3.449 2.829 3.449 5.139-.007 3.01-2.371 5.466-5.283 5.466z"
        /> < title > { title } < / title > < / svg >
    }
}
