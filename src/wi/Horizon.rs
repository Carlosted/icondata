#[cfg(feature = "WiHorizon")]
use leptos::*;
#[cfg(feature = "WiHorizon")]
///This icon requires the feature `WiHorizon` to be enabled.
#[component]
pub fn Horizon(
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
        "M4.93,20.97c0-0.26,0.09-0.47,0.28-0.62c0.14-0.16,0.35-0.23,0.63-0.23h18.34c0.25,0,0.46,0.08,0.64,0.24&#xA;	c0.18,0.16,0.26,0.37,0.26,0.61c0,0.24-0.09,0.45-0.27,0.62s-0.39,0.27-0.63,0.27H5.84c-0.25,0-0.46-0.09-0.64-0.27&#xA;	C5.02,21.42,4.93,21.21,4.93,20.97z M6.9,12.68c0-0.26,0.08-0.47,0.23-0.63c0.17-0.18,0.38-0.26,0.65-0.26&#xA;	c0.23,0,0.43,0.09,0.6,0.26l1.5,1.5c0.18,0.18,0.27,0.39,0.27,0.63c0,0.23-0.09,0.44-0.27,0.62c-0.15,0.18-0.35,0.27-0.6,0.27&#xA;	s-0.47-0.09-0.64-0.27l-1.5-1.5C6.98,13.15,6.9,12.95,6.9,12.68z M9.83,18.27c-0.04,0.16,0.01,0.23,0.15,0.23h1.49&#xA;	c0.07,0,0.14-0.06,0.22-0.17c0.3-0.64,0.74-1.14,1.33-1.52s1.24-0.56,1.96-0.56c0.73,0,1.39,0.19,1.99,0.56s1.05,0.88,1.35,1.52&#xA;	c0.08,0.11,0.16,0.17,0.23,0.17h1.48c0.13,0,0.18-0.08,0.15-0.23c-0.34-1.13-0.99-2.05-1.95-2.76c-0.96-0.71-2.04-1.06-3.25-1.06&#xA;	c-1.2,0-2.28,0.35-3.23,1.06C10.82,16.22,10.17,17.14,9.83,18.27z M14.14,11.81V9.68c0-0.25,0.08-0.46,0.24-0.64&#xA;	c0.16-0.18,0.37-0.26,0.61-0.26c0.25,0,0.46,0.09,0.63,0.26c0.17,0.17,0.25,0.39,0.25,0.64v2.14c0,0.26-0.08,0.47-0.25,0.64&#xA;	c-0.17,0.17-0.38,0.25-0.63,0.25c-0.24,0-0.45-0.09-0.61-0.26S14.14,12.06,14.14,11.81z M19.86,14.18c0-0.24,0.08-0.45,0.25-0.63&#xA;	l1.54-1.5c0.16-0.18,0.36-0.26,0.62-0.26c0.24,0,0.44,0.08,0.6,0.25s0.23,0.38,0.23,0.64c0,0.26-0.08,0.47-0.23,0.62l-1.48,1.5&#xA;	c-0.17,0.17-0.36,0.26-0.56,0.28c-0.23,0.02-0.44-0.06-0.65-0.24S19.86,14.43,19.86,14.18z"
        /> < title > { title } < / title > < / svg >
    }
}
