#[cfg(feature = "WiSolarEclipse")]
use leptos::*;
#[cfg(feature = "WiSolarEclipse")]
///This icon requires the feature `WiSolarEclipse` to be enabled.
#[component]
pub fn SolarEclipse(
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
        "M4.37,14.62c0-0.24,0.08-0.45,0.25-0.62c0.17-0.16,0.38-0.24,0.6-0.24h2.04c0.23,0,0.42,0.08,0.58,0.25&#xA;	c0.15,0.17,0.23,0.37,0.23,0.61S8,15.06,7.85,15.23c-0.15,0.17-0.35,0.25-0.58,0.25H5.23c-0.23,0-0.43-0.08-0.6-0.25&#xA;	C4.46,15.06,4.37,14.86,4.37,14.62z M7.23,21.55c0-0.23,0.08-0.43,0.23-0.61l1.47-1.43c0.15-0.16,0.35-0.23,0.59-0.23&#xA;	c0.24,0,0.44,0.08,0.6,0.23s0.24,0.34,0.24,0.57c0,0.24-0.08,0.46-0.24,0.64L8.7,22.14c-0.41,0.32-0.82,0.32-1.23,0&#xA;	C7.31,21.98,7.23,21.78,7.23,21.55z M7.23,7.71c0-0.23,0.08-0.43,0.23-0.61C7.66,6.93,7.87,6.85,8.1,6.85&#xA;	c0.22,0,0.42,0.08,0.59,0.24l1.43,1.47c0.16,0.15,0.24,0.35,0.24,0.59c0,0.24-0.08,0.44-0.24,0.6s-0.36,0.24-0.6,0.24&#xA;	c-0.24,0-0.44-0.08-0.59-0.24L7.47,8.32C7.31,8.16,7.23,7.95,7.23,7.71z M9.78,14.62c0-0.93,0.23-1.8,0.7-2.6s1.1-1.44,1.91-1.91&#xA;	s1.67-0.7,2.6-0.7c0.7,0,1.37,0.14,2.02,0.42c0.64,0.28,1.2,0.65,1.66,1.12c0.47,0.47,0.84,1.02,1.11,1.66&#xA;	c0.27,0.64,0.41,1.32,0.41,2.02c0,0.94-0.23,1.81-0.7,2.61c-0.47,0.8-1.1,1.43-1.9,1.9c-0.8,0.47-1.67,0.7-2.61,0.7&#xA;	s-1.81-0.23-2.61-0.7c-0.8-0.47-1.43-1.1-1.9-1.9C10.02,16.43,9.78,15.56,9.78,14.62z M14.14,22.4c0-0.24,0.08-0.44,0.25-0.6&#xA;	s0.37-0.24,0.6-0.24c0.24,0,0.45,0.08,0.61,0.24s0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25&#xA;	s-0.44-0.08-0.6-0.25c-0.17-0.17-0.25-0.38-0.25-0.62V22.4z M14.14,6.9V4.86c0-0.23,0.08-0.43,0.25-0.6C14.56,4.09,14.76,4,15,4&#xA;	s0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6V6.9c0,0.23-0.08,0.42-0.25,0.58S15.23,7.71,15,7.71s-0.44-0.08-0.6-0.23&#xA;	S14.14,7.13,14.14,6.9z M14.25,11.22c0.87,0.11,1.6,0.49,2.19,1.15c0.59,0.66,0.89,1.44,0.89,2.33c0,0.83-0.26,1.56-0.78,2.2&#xA;	c-0.52,0.63-1.18,1.04-1.98,1.21c0.2,0.02,0.35,0.04,0.44,0.04c0.97,0,1.81-0.35,2.5-1.04s1.04-1.52,1.04-2.5&#xA;	c0-0.96-0.35-1.78-1.04-2.47c-0.69-0.68-1.52-1.02-2.5-1.02C14.74,11.14,14.49,11.17,14.25,11.22z M19.66,20.08&#xA;	c0-0.23,0.08-0.42,0.23-0.56c0.15-0.16,0.34-0.23,0.56-0.23c0.24,0,0.44,0.08,0.6,0.23l1.46,1.43c0.16,0.17,0.24,0.38,0.24,0.61&#xA;	c0,0.23-0.08,0.43-0.24,0.59c-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.42C19.74,20.55,19.66,20.34,19.66,20.08z M19.66,9.16&#xA;	c0-0.25,0.08-0.45,0.23-0.59l1.42-1.47c0.17-0.16,0.37-0.24,0.59-0.24c0.24,0,0.44,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6&#xA;	c0,0.25-0.08,0.46-0.24,0.62l-1.46,1.43c-0.18,0.16-0.38,0.24-0.6,0.24c-0.23,0-0.41-0.08-0.56-0.24S19.66,9.4,19.66,9.16z&#xA;	 M21.92,14.62c0-0.24,0.08-0.44,0.24-0.62c0.16-0.16,0.35-0.24,0.57-0.24h2.02c0.23,0,0.43,0.09,0.6,0.26&#xA;	c0.17,0.17,0.26,0.37,0.26,0.6s-0.09,0.43-0.26,0.6c-0.17,0.17-0.37,0.25-0.6,0.25h-2.02c-0.23,0-0.43-0.08-0.58-0.25&#xA;	S21.92,14.86,21.92,14.62z"
        /> < title > { title } < / title > < / svg >
    }
}
