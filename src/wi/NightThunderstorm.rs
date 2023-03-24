#[cfg(feature = "WiNightThunderstorm")]
use leptos::*;
#[cfg(feature = "WiNightThunderstorm")]
///This icon requires the feature `WiNightThunderstorm` to be enabled.
#[component]
pub fn NightThunderstorm(
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
        "M4.28,16.89c0,1.11,0.33,2.11,0.99,2.98s1.52,1.46,2.56,1.75l-0.64,1.68c-0.05,0.14,0,0.22,0.14,0.22h2.12l-1.04,4.19h0.28&#xA;	l3.97-5.62c0.04-0.04,0.04-0.09,0.01-0.14c-0.03-0.05-0.08-0.07-0.15-0.07h-2.17l2.47-4.61c0.07-0.14,0.02-0.22-0.14-0.22H9.74&#xA;	c-0.09,0-0.16,0.05-0.23,0.14l-1.07,2.87c-0.71-0.17-1.3-0.56-1.77-1.14s-0.7-1.26-0.7-2.02c0-0.83,0.28-1.55,0.84-2.16&#xA;	s1.26-0.96,2.1-1.06l0.53-0.04c0.12,0,0.18-0.06,0.18-0.18l0.07-0.53c0.07-0.71,0.3-1.35,0.69-1.94c0.39-0.58,0.9-1.04,1.52-1.37&#xA;	s1.29-0.5,2.01-0.5c1.08,0,2.03,0.37,2.84,1.1c0.81,0.73,1.27,1.63,1.39,2.71l0.08,0.56c0,0.12,0.06,0.19,0.17,0.19h1.62&#xA;	c0.89,0,1.65,0.32,2.3,0.96s0.97,1.39,0.97,2.27c0,0.86-0.3,1.61-0.9,2.25s-1.33,0.97-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.18v1.34&#xA;	c0,0.11,0.07,0.17,0.2,0.17c0.87-0.02,1.67-0.26,2.4-0.72c0.73-0.45,1.31-1.05,1.72-1.8s0.63-1.56,0.63-2.43&#xA;	c0-0.73-0.14-1.4-0.42-2.01c0.78-0.93,1.17-2.03,1.17-3.31c0-0.71-0.14-1.38-0.42-2.02c-0.28-0.64-0.65-1.2-1.12-1.67&#xA;	c-0.47-0.47-1.02-0.84-1.67-1.12c-0.64-0.28-1.32-0.42-2.02-0.42c-1.54,0-2.83,0.58-3.86,1.73c-0.81-0.43-1.71-0.65-2.7-0.65&#xA;	c-1.41,0-2.66,0.44-3.75,1.31s-1.79,1.99-2.1,3.35c-1.1,0.26-2.01,0.83-2.73,1.73S4.28,15.74,4.28,16.89z M12.21,26.77&#xA;	c0,0.16,0.05,0.32,0.15,0.46s0.25,0.25,0.45,0.31l0.25,0.03c0.42,0,0.68-0.2,0.8-0.6l2.43-8.89c0.06-0.23,0.04-0.45-0.07-0.64&#xA;	c-0.11-0.2-0.27-0.33-0.49-0.4c-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.34,0.28-0.4,0.5l-2.45,8.9&#xA;	C12.22,26.67,12.21,26.76,12.21,26.77z M16.35,23.74c0,0.4,0.21,0.67,0.62,0.8c0.17,0.02,0.26,0.03,0.26,0.03&#xA;	c0.11,0,0.23-0.02,0.35-0.08c0.2-0.09,0.34-0.27,0.42-0.55l1.64-5.85c0.06-0.23,0.04-0.45-0.08-0.64c-0.11-0.2-0.28-0.33-0.51-0.4&#xA;	c-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.33,0.28-0.39,0.5l-1.62,5.89C16.37,23.64,16.35,23.72,16.35,23.74z M18.02,9.04&#xA;	c0.68-0.64,1.5-0.96,2.48-0.96c0.97,0,1.8,0.34,2.48,1.02c0.69,0.68,1.03,1.51,1.03,2.48c0,0.63-0.17,1.25-0.51,1.85&#xA;	c-0.96-0.96-2.12-1.44-3.48-1.44h-0.32C19.42,10.84,18.86,9.86,18.02,9.04z"
        /> < title > { title } < / title > < / svg >
    }
}
