#[cfg(feature = "FaSolidCrutch")]
use leptos::*;
#[cfg(feature = "FaSolidCrutch")]
///This icon requires the feature `FaSolidCrutch` to be enabled.
#[component]
pub fn Crutch(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M297.4 9.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-160-160c-12.5-12.5-32.8-12.5-45.3 0zm-96 144l-34.8 34.8c-12.9 12.9-21.9 29.2-25.8 47.1L116.8 342.9c-1.3 5.9-4.3 11.4-8.6 15.7L9.4 457.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l98.8-98.8c4.3-4.3 9.7-7.3 15.7-8.6l107.6-23.9c17.8-4 34.1-12.9 47.1-25.8l34.7-34.7c0 0 .1-.1 .1-.1s.1-.1 .1-.1l74.6-74.6-45.3-45.3L336 242.7 269.3 176l52.1-52.1L276.1 78.6l-74.7 74.7zM224 221.3L290.7 288l-12.2 12.2c-4.3 4.3-9.7 7.3-15.7 8.6l-76.7 17 17-76.7c1.3-5.9 4.3-11.4 8.6-15.7L224 221.3z"
        /> < title > { title } < / title > < / svg >
    }
}
