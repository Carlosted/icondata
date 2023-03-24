#[cfg(feature = "WiWindBeaufort10")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort10")]
///This icon requires the feature `WiWindBeaufort10` to be enabled.
#[component]
pub fn WindBeaufort10(
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
        "M3.15,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16&#xA;	c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53&#xA;	s0.91-0.17,1.26-0.52s0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H3.75&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C3.21,13.21,3.15,13.34,3.15,13.5z M3.15,11.48c0,0.17,0.06,0.3,0.17,0.39&#xA;	c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26&#xA;	s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4&#xA;	c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18&#xA;	c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H3.75c-0.16,0-0.3,0.06-0.42,0.17&#xA;	C3.21,11.18,3.15,11.32,3.15,11.48z M15.97,21.8h2.46l1.64-7.94h-2.45L15.97,21.8z M20.16,18.88c0,0.52,0.08,0.98,0.24,1.38&#xA;	s0.38,0.72,0.66,0.95c0.27,0.23,0.58,0.4,0.9,0.52s0.68,0.17,1.05,0.17c0.61,0,1.16-0.12,1.64-0.38c0.48-0.25,0.86-0.56,1.13-0.93&#xA;	c0.27-0.37,0.5-0.79,0.68-1.25c0.18-0.47,0.3-0.89,0.37-1.27c0.06-0.38,0.09-0.73,0.09-1.05c0-0.97-0.27-1.72-0.8-2.25&#xA;	s-1.24-0.8-2.13-0.8c-1.03,0-1.93,0.46-2.7,1.37C20.54,16.26,20.16,17.44,20.16,18.88z M22.21,18.98c0-0.16,0.01-0.35,0.04-0.59&#xA;	c0.03-0.23,0.08-0.51,0.16-0.84c0.08-0.32,0.18-0.62,0.3-0.9c0.12-0.27,0.29-0.5,0.52-0.69c0.22-0.19,0.47-0.29,0.75-0.29&#xA;	c0.27,0,0.48,0.09,0.65,0.27c0.16,0.18,0.24,0.44,0.24,0.79c0,0.96-0.17,1.78-0.5,2.45s-0.75,1.01-1.23,1.01&#xA;	C22.52,20.19,22.21,19.79,22.21,18.98z"
        /> < title > { title } < / title > < / svg >
    }
}
