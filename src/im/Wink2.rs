#[cfg(feature = "ImWink2")]
use leptos::*;
#[cfg(feature = "ImWink2")]
///This icon requires the feature `ImWink2` to be enabled.
#[component]
pub fn Wink2(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8c4.418 0 8-3.582 8-8s-3.582-8-8-8zM11 4c0.552 0 1 0.672 1 1.5s-0.448 1.5-1 1.5-1-0.672-1-1.5 0.448-1.5 1-1.5zM5.5 4.876c0.932 0 1.594 0.349 1.594 0.895 0 0.116 0.060 0.672-0.003 0.775-0.232-0.384-0.856-0.659-1.591-0.659s-1.359 0.275-1.591 0.659c-0.062-0.103-0.003-0.659-0.003-0.775 0-0.546 0.662-0.895 1.594-0.895zM7.818 13c-1.863 0-3.498-1.004-4.42-2.515 1.1 0.86 3.040 1.028 5.083 0.625 2.191-0.433 3.892-1.43 4.507-2.759-0.338 2.624-2.524 4.649-5.17 4.649z"
        /> < title > { title } < / title > < / svg >
    }
}
