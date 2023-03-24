#[cfg(feature = "FaSolidChessKnight")]
use leptos::*;
#[cfg(feature = "FaSolidChessKnight")]
///This icon requires the feature `FaSolidChessKnight` to be enabled.
#[component]
pub fn ChessKnight(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M80 48L66.7 61.3C54.7 73.3 48 89.5 48 106.5V238.9c0 10.7 5.3 20.7 14.2 26.6l10.6 7c14.3 9.6 32.7 10.7 48.1 3l3.2-1.6c2.6-1.3 5-2.8 7.3-4.5l49.4-37c6.6-5 15.7-5 22.3 0c10.2 7.7 9.9 23.1-.7 30.3L74.4 350C57.9 361.3 48 380 48 400H368l28.9-159c2.1-11.3 3.1-22.8 3.1-34.3V192C400 86 314 0 208 0H67.8C56.9 0 48 8.9 48 19.8c0 7.5 4.2 14.3 10.9 17.7L80 48zm24 68a20 20 0 1 1 40 0 20 20 0 1 1 -40 0zM6.6 473.4c-4.2 4.2-6.6 10-6.6 16C0 501.9 10.1 512 22.6 512H393.4c12.5 0 22.6-10.1 22.6-22.6c0-6-2.4-11.8-6.6-16L368 432H48L6.6 473.4z"
        /> < title > { title } < / title > < / svg >
    }
}
