#[cfg(feature = "WiThunderstorm")]
use leptos::*;
#[cfg(feature = "WiThunderstorm")]
///This icon requires the feature `WiThunderstorm` to be enabled.
#[component]
pub fn Thunderstorm(
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
        "M4.63,16.91c0,1.11,0.33,2.1,0.99,2.97s1.52,1.47,2.58,1.79l-0.66,1.68c-0.03,0.14,0.02,0.22,0.14,0.22h2.13l-0.98,4.3h0.28&#xA;	l3.92-5.75c0.04-0.04,0.04-0.09,0.01-0.14c-0.03-0.05-0.08-0.07-0.15-0.07h-2.18l2.48-4.64c0.07-0.14,0.02-0.22-0.14-0.22h-2.94&#xA;	c-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.87c-0.71-0.18-1.3-0.57-1.77-1.16c-0.47-0.59-0.7-1.26-0.7-2.01c0-0.83,0.28-1.55,0.85-2.17&#xA;	c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.07c0.13,0,0.2-0.06,0.2-0.18l0.07-0.51c0.11-1.08,0.56-1.99,1.37-2.72&#xA;	c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.85,1.1c0.82,0.73,1.28,1.64,1.4,2.72l0.07,0.58c0,0.11,0.06,0.17,0.18,0.17h1.6&#xA;	c0.91,0,1.68,0.32,2.32,0.95c0.64,0.63,0.97,1.4,0.97,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.33,0.97-2.2,1.04&#xA;	c-0.13,0-0.2,0.06-0.2,0.18v1.37c0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.39-1.51s1.39-2.11,1.39-3.45&#xA;	c0-0.9-0.22-1.73-0.67-2.49c-0.44-0.76-1.05-1.36-1.81-1.8c-0.77-0.44-1.6-0.66-2.5-0.66H20.1c-0.33-1.33-1.04-2.42-2.11-3.26&#xA;	s-2.3-1.27-3.68-1.27c-1.41,0-2.67,0.44-3.76,1.31s-1.79,1.99-2.1,3.36c-1.11,0.26-2.02,0.83-2.74,1.73S4.63,15.76,4.63,16.91z&#xA;	 M12.77,26.62c0,0.39,0.19,0.65,0.58,0.77c0.01,0,0.05,0,0.11,0.01c0.06,0.01,0.11,0.01,0.14,0.01c0.17,0,0.33-0.05,0.49-0.15&#xA;	c0.16-0.1,0.27-0.26,0.32-0.48l2.25-8.69c0.06-0.24,0.04-0.45-0.07-0.65c-0.11-0.19-0.27-0.32-0.5-0.39&#xA;	c-0.17-0.02-0.26-0.03-0.26-0.03c-0.16,0-0.32,0.05-0.47,0.15c-0.15,0.1-0.26,0.25-0.31,0.45l-2.26,8.72&#xA;	C12.78,26.44,12.77,26.53,12.77,26.62z M16.93,23.56c0,0.13,0.03,0.26,0.1,0.38c0.14,0.22,0.31,0.37,0.51,0.44&#xA;	c0.11,0.03,0.21,0.05,0.3,0.05s0.2-0.02,0.32-0.08c0.21-0.09,0.35-0.28,0.42-0.57l1.44-5.67c0.03-0.14,0.05-0.23,0.05-0.27&#xA;	c0-0.15-0.05-0.3-0.16-0.45s-0.26-0.26-0.46-0.32c-0.17-0.02-0.26-0.03-0.26-0.03c-0.17,0-0.33,0.05-0.47,0.15&#xA;	c-0.14,0.1-0.24,0.25-0.3,0.45l-1.46,5.7c0,0.02,0,0.05-0.01,0.11C16.93,23.5,16.93,23.53,16.93,23.56z"
        /> < title > { title } < / title > < / svg >
    }
}
