#[cfg(feature = "WiDayLightWind")]
use leptos::*;
#[cfg(feature = "WiDayLightWind")]
///This icon requires the feature `WiDayLightWind` to be enabled.
#[component]
pub fn DayLightWind(
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
        "M2.32,14.85c0,0.24,0.09,0.44,0.26,0.6c0.16,0.17,0.36,0.25,0.6,0.25h9.42c0.23,0,0.43-0.08,0.59-0.25&#xA;	c0.16-0.17,0.24-0.37,0.24-0.6c0-0.23-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.24-0.59-0.24H3.18c-0.24,0-0.44,0.08-0.61,0.24&#xA;	C2.4,14.42,2.32,14.62,2.32,14.85z M2.65,21c0,0.24,0.08,0.44,0.25,0.6c0.16,0.17,0.36,0.25,0.6,0.25h9.43&#xA;	c0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6s-0.08-0.43-0.25-0.59s-0.37-0.24-0.61-0.24H3.51&#xA;	c-0.24,0-0.44,0.08-0.6,0.24C2.74,20.57,2.65,20.77,2.65,21z M4.02,17.9c0,0.24,0.08,0.43,0.25,0.59c0.17,0.16,0.38,0.23,0.63,0.23&#xA;	h9.4c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.25-0.08-0.45-0.23-0.61c-0.16-0.16-0.35-0.24-0.59-0.24H4.9&#xA;	c-0.25,0-0.46,0.08-0.63,0.24C4.11,17.45,4.02,17.65,4.02,17.9z M6.45,11.55c0-0.24,0.09-0.44,0.26-0.62&#xA;	c0.17-0.16,0.38-0.24,0.6-0.24h2.03c0.23,0,0.42,0.08,0.58,0.25c0.16,0.17,0.23,0.37,0.23,0.61c0,0.24-0.08,0.44-0.23,0.6&#xA;	c-0.16,0.17-0.35,0.25-0.58,0.25H7.31c-0.24,0-0.44-0.08-0.61-0.25C6.53,11.98,6.45,11.78,6.45,11.55z M9.31,4.63&#xA;	c0-0.22,0.08-0.43,0.24-0.61c0.19-0.16,0.4-0.24,0.63-0.24c0.22,0,0.42,0.08,0.59,0.24l1.42,1.47c0.16,0.15,0.24,0.35,0.24,0.59&#xA;	c0,0.24-0.08,0.44-0.24,0.6c-0.16,0.16-0.36,0.24-0.6,0.24c-0.24,0-0.44-0.08-0.59-0.24L9.55,5.25C9.39,5.07,9.31,4.87,9.31,4.63z&#xA;	 M11.86,11.43v-0.07c0.02-0.91,0.27-1.75,0.74-2.53c0.47-0.77,1.11-1.38,1.9-1.83s1.65-0.67,2.57-0.67c0.7,0,1.37,0.14,2.02,0.42&#xA;	c0.64,0.28,1.2,0.65,1.66,1.12c0.47,0.47,0.84,1.02,1.11,1.66c0.27,0.64,0.41,1.32,0.41,2.02c0,0.94-0.23,1.8-0.69,2.6&#xA;	s-1.09,1.43-1.88,1.89c-0.79,0.47-1.66,0.7-2.6,0.71h-0.2c-0.07,0-0.13-0.02-0.18-0.07c-0.05-0.05-0.08-0.11-0.08-0.18v-1.22&#xA;	c0-0.13,0.07-0.2,0.22-0.2h0.24c0.96-0.01,1.79-0.35,2.47-1.05c0.68-0.69,1.03-1.52,1.03-2.49c0-0.96-0.35-1.78-1.04-2.47&#xA;	c-0.69-0.68-1.52-1.02-2.5-1.02c-0.94,0-1.76,0.32-2.44,0.98c-0.68,0.65-1.04,1.44-1.08,2.37c0,0.06-0.02,0.11-0.07,0.17&#xA;	s-0.13,0.09-0.25,0.09h-1.14C11.93,11.67,11.86,11.59,11.86,11.43z M16.23,21.31v-1.99c0-0.24,0.08-0.44,0.24-0.6&#xA;	s0.36-0.24,0.6-0.24c0.24,0,0.45,0.08,0.61,0.24s0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25&#xA;	c-0.24,0-0.44-0.08-0.6-0.25S16.23,21.56,16.23,21.31z M16.23,3.83V1.78c0-0.24,0.08-0.44,0.25-0.6s0.36-0.25,0.6-0.25&#xA;	c0.23,0,0.43,0.08,0.6,0.25s0.25,0.37,0.25,0.6v2.04c0,0.23-0.08,0.42-0.25,0.58c-0.17,0.15-0.37,0.23-0.6,0.23&#xA;	c-0.24,0-0.44-0.08-0.6-0.23C16.31,4.25,16.23,4.06,16.23,3.83z M21.74,17.01c0-0.23,0.07-0.42,0.23-0.56&#xA;	c0.15-0.16,0.34-0.23,0.57-0.23c0.24,0,0.44,0.08,0.6,0.23l1.45,1.42c0.16,0.17,0.24,0.38,0.24,0.61c0,0.23-0.08,0.43-0.24,0.59&#xA;	c-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.43C21.82,17.48,21.74,17.26,21.74,17.01z M21.74,6.08c0-0.25,0.07-0.45,0.23-0.59l1.42-1.47&#xA;	c0.18-0.16,0.37-0.24,0.59-0.24c0.24,0,0.44,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6c0,0.25-0.08,0.46-0.24,0.62l-1.45,1.43&#xA;	c-0.18,0.16-0.38,0.24-0.6,0.24c-0.23,0-0.41-0.08-0.57-0.24S21.74,6.32,21.74,6.08z M24,11.55c0-0.23,0.08-0.44,0.25-0.62&#xA;	c0.16-0.16,0.35-0.24,0.56-0.24h2.03c0.23,0,0.43,0.09,0.61,0.26c0.17,0.17,0.26,0.37,0.26,0.6c0,0.23-0.09,0.43-0.26,0.6&#xA;	c-0.18,0.17-0.38,0.25-0.61,0.25h-2.03c-0.23,0-0.42-0.08-0.58-0.25C24.08,11.99,24,11.79,24,11.55z"
        /> < title > { title } < / title > < / svg >
    }
}
