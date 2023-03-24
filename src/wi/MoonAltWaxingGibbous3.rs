#[cfg(feature = "WiMoonAltWaxingGibbous3")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaxingGibbous3")]
///This icon requires the feature `WiMoonAltWaxingGibbous3` to be enabled.
#[component]
pub fn MoonAltWaxingGibbous3(
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
        "M3.74,14.44c0-2.03,0.5-3.91,1.51-5.64s2.37-3.1,4.1-4.1s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89s2.59,1.4,3.6,2.4&#xA;	s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4s-2.85,0.89-4.37,0.89&#xA;	s-2.98-0.3-4.37-0.89s-2.59-1.4-3.59-2.4s-1.8-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M10.54,14.44c0,4.44,1.17,7.78,3.5,10.02&#xA;	c0.07,0,0.17,0,0.3,0.01s0.25,0.02,0.35,0.02s0.2,0.01,0.3,0.01c1.36,0,2.66-0.27,3.9-0.8s2.32-1.25,3.22-2.15s1.61-1.97,2.15-3.21&#xA;	s0.8-2.54,0.8-3.91c0-1.36-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.22-2.15s-2.55-0.8-3.9-0.8&#xA;	c-0.36,0-0.63,0.01-0.81,0.03c-1.08,1.22-1.96,2.69-2.64,4.42S10.54,12.43,10.54,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
