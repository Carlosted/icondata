#[cfg(feature = "WiCloudDown")]
use leptos::*;
#[cfg(feature = "WiCloudDown")]
///This icon requires the feature `WiCloudDown` to be enabled.
#[component]
pub fn CloudDown(
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
        "M4.61,16.88c0,1.34,0.47,2.48,1.4,3.44c0.93,0.96,2.07,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34&#xA;	c0-0.12-0.06-0.18-0.17-0.18c-0.86-0.04-1.59-0.38-2.19-1.02c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17&#xA;	c0.57-0.62,1.27-0.97,2.1-1.07L9.8,13.6c0.13,0,0.2-0.06,0.2-0.17l0.08-0.55c0.1-1.08,0.55-1.99,1.36-2.71&#xA;	c0.81-0.73,1.76-1.09,2.86-1.09c1.09,0,2.04,0.36,2.86,1.09c0.82,0.73,1.28,1.63,1.4,2.71l0.07,0.58c0,0.11,0.06,0.17,0.17,0.17&#xA;	h1.62c0.89,0,1.66,0.32,2.31,0.97c0.65,0.64,0.98,1.4,0.98,2.28c0,0.87-0.3,1.62-0.91,2.26c-0.61,0.64-1.34,0.98-2.19,1.02&#xA;	c-0.13,0-0.19,0.06-0.19,0.18v1.34c0,0.11,0.06,0.17,0.19,0.17c0.88-0.02,1.68-0.26,2.41-0.72c0.73-0.45,1.31-1.05,1.73-1.8&#xA;	s0.63-1.57,0.63-2.45c0-0.9-0.22-1.73-0.67-2.48c-0.44-0.76-1.05-1.35-1.81-1.79s-1.59-0.65-2.49-0.65h-0.33&#xA;	c-0.33-1.34-1.03-2.43-2.1-3.29s-2.31-1.28-3.69-1.28c-1.41,0-2.67,0.44-3.76,1.31s-1.8,2-2.11,3.37c-1.1,0.26-2.01,0.84-2.73,1.74&#xA;	S4.61,15.73,4.61,16.88z M11.58,18.4c0,0.24,0.08,0.44,0.24,0.6l2.59,2.61c0.12,0.16,0.32,0.23,0.57,0.23&#xA;	c0.28,0,0.48-0.08,0.61-0.23l2.6-2.61c0.16-0.17,0.24-0.38,0.24-0.6c0-0.23-0.08-0.43-0.24-0.58s-0.36-0.23-0.6-0.23&#xA;	c-0.24,0-0.44,0.08-0.62,0.23l-1.12,1.11v-3.98c0-0.24-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.23-0.61-0.23s-0.43,0.08-0.59,0.23&#xA;	c-0.16,0.16-0.23,0.35-0.23,0.59v3.98l-1.1-1.11c-0.18-0.16-0.38-0.23-0.63-0.23c-0.25,0-0.45,0.08-0.61,0.23&#xA;	C11.66,17.97,11.58,18.17,11.58,18.4z"
        /> < title > { title } < / title > < / svg >
    }
}
