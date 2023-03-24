#[cfg(feature = "WiMoonWaningGibbous2")]
use leptos::*;
#[cfg(feature = "WiMoonWaningGibbous2")]
///This icon requires the feature `WiMoonWaningGibbous2` to be enabled.
#[component]
pub fn MoonWaningGibbous2(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56&#xA;	c4.33-1.73,6.49-5.47,6.49-11.22c0-1.39-0.18-2.7-0.54-3.93s-0.85-2.31-1.47-3.23s-1.31-1.71-2.06-2.39s-1.56-1.23-2.42-1.66&#xA;	c-2.03,0-3.91,0.5-5.63,1.5S6.25,7.14,5.24,8.86S3.74,12.46,3.74,14.49z"
        /> < title > { title } < / title > < / svg >
    }
}
