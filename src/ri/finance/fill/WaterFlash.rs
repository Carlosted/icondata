#[cfg(feature = "RiFinanceFillWaterFlash")]
use leptos::*;
#[cfg(feature = "RiFinanceFillWaterFlash")]
///This icon requires the feature `RiFinanceFillWaterFlash` to be enabled.
#[component]
pub fn WaterFlash(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M5.636 6.636L12 .272l6.364 6.364a9 9 0 1 1-12.728 0zM13 11V6.5L8.5 13H11v4.5l4.5-6.5H13z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
