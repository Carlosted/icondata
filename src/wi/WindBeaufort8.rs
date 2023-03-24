#[cfg(feature = "WiWindBeaufort8")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort8")]
///This icon requires the feature `WiWindBeaufort8` to be enabled.
#[component]
pub fn WindBeaufort8(
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
        "M4.99,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16&#xA;	c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.28,0.53&#xA;	c0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.58&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C5.05,13.21,4.99,13.34,4.99,13.5z M4.99,11.48c0,0.17,0.06,0.3,0.17,0.39&#xA;	c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26&#xA;	s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4&#xA;	c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18&#xA;	c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.58c-0.16,0-0.3,0.06-0.42,0.17&#xA;	C5.05,11.18,4.99,11.32,4.99,11.48z M17.99,19.67c0,0.73,0.29,1.29,0.86,1.66c0.57,0.38,1.34,0.57,2.31,0.57&#xA;	c0.59,0,1.12-0.06,1.57-0.18c0.46-0.12,0.81-0.27,1.07-0.44s0.46-0.38,0.62-0.62c0.16-0.24,0.26-0.46,0.31-0.66&#xA;	c0.05-0.2,0.08-0.4,0.08-0.61c0-0.41-0.12-0.77-0.36-1.06c-0.24-0.3-0.55-0.49-0.94-0.57l0.02-0.03v0.01&#xA;	c0.45-0.06,0.82-0.26,1.12-0.6c0.29-0.33,0.44-0.73,0.44-1.19c0-0.38-0.09-0.71-0.26-0.98s-0.41-0.48-0.71-0.61&#xA;	c-0.3-0.14-0.61-0.24-0.92-0.3c-0.31-0.06-0.65-0.09-1.01-0.09c-0.48,0-0.9,0.05-1.28,0.14c-0.38,0.09-0.69,0.22-0.93,0.37&#xA;	c-0.24,0.15-0.43,0.33-0.59,0.53s-0.27,0.4-0.33,0.6c-0.06,0.2-0.09,0.41-0.09,0.62c0,0.34,0.09,0.64,0.27,0.9&#xA;	c0.18,0.26,0.43,0.43,0.75,0.53v0.03c-0.56,0.06-1.04,0.27-1.42,0.61C18.18,18.67,17.99,19.12,17.99,19.67z M20.1,19.44&#xA;	c0-0.35,0.14-0.61,0.42-0.77s0.62-0.24,1.01-0.24c0.41,0,0.7,0.09,0.89,0.28c0.18,0.18,0.28,0.38,0.28,0.6v0.13&#xA;	c0,0.28-0.13,0.49-0.38,0.64c-0.25,0.14-0.58,0.22-0.97,0.22l0.03-0.01c-0.14,0-0.27-0.01-0.4-0.03s-0.27-0.06-0.41-0.11&#xA;	c-0.14-0.06-0.25-0.14-0.34-0.26C20.15,19.76,20.1,19.61,20.1,19.44z M20.86,16.37c0-0.32,0.12-0.55,0.37-0.69s0.55-0.22,0.9-0.22&#xA;	c0.3,0,0.55,0.07,0.76,0.2s0.31,0.35,0.31,0.63c0,0.07-0.02,0.15-0.05,0.23c-0.03,0.08-0.09,0.17-0.17,0.27&#xA;	c-0.08,0.1-0.21,0.18-0.39,0.24c-0.18,0.06-0.4,0.09-0.66,0.09c-0.4,0-0.68-0.08-0.84-0.23C20.94,16.75,20.86,16.57,20.86,16.37z"
        /> < title > { title } < / title > < / svg >
    }
}
