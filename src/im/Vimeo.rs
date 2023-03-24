#[cfg(feature = "ImVimeo")]
use leptos::*;
#[cfg(feature = "ImVimeo")]
///This icon requires the feature `ImVimeo` to be enabled.
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.994 4.281c-0.072 1.556-1.159 3.691-3.263 6.397-2.175 2.825-4.016 4.241-5.522 4.241-0.931 0-1.722-0.859-2.366-2.581-0.431-1.578-0.859-3.156-1.291-4.734-0.478-1.722-0.991-2.581-1.541-2.581-0.119 0-0.538 0.253-1.256 0.753l-0.753-0.969c0.791-0.694 1.569-1.388 2.334-2.081 1.053-0.909 1.844-1.387 2.372-1.438 1.244-0.119 2.013 0.731 2.3 2.553 0.309 1.966 0.525 3.188 0.647 3.666 0.359 1.631 0.753 2.447 1.184 2.447 0.334 0 0.838-0.528 1.509-1.588 0.669-1.056 1.028-1.862 1.078-2.416 0.097-0.912-0.262-1.372-1.078-1.372-0.384 0-0.778 0.088-1.184 0.263 0.787-2.575 2.287-3.825 4.506-3.753 1.641 0.044 2.416 1.109 2.322 3.194z"
        /> < title > { title } < / title > < / svg >
    }
}
