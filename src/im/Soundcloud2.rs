#[cfg(feature = "ImSoundcloud2")]
use leptos::*;
#[cfg(feature = "ImSoundcloud2")]
///This icon requires the feature `ImSoundcloud2` to be enabled.
#[component]
pub fn Soundcloud2(
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
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM2.75 11h-0.5l-0.25-1.5 0.25-1.5h0.5l0.25 1.5-0.25 1.5zM4.75 11h-0.5l-0.25-2 0.25-2h0.5l0.25 2-0.25 2zM6.75 11h-0.5l-0.25-3 0.25-3h0.5l0.25 3-0.25 3zM12.894 11c-0.031 0-4.706-0.003-4.709-0.003-0.1-0.009-0.181-0.097-0.184-0.2v-5.394c0-0.1 0.034-0.15 0.162-0.2 0.331-0.128 0.703-0.203 1.088-0.203 1.566 0 2.85 1.2 2.987 2.734 0.203-0.084 0.425-0.131 0.656-0.131 0.938 0 1.7 0.762 1.7 1.7s-0.762 1.697-1.7 1.697z"
        /> < title > { title } < / title > < / svg >
    }
}
