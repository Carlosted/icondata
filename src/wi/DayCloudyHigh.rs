#[cfg(feature = "WiDayCloudyHigh")]
use leptos::*;
#[cfg(feature = "WiDayCloudyHigh")]
///This icon requires the feature `WiDayCloudyHigh` to be enabled.
#[component]
pub fn DayCloudyHigh(
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
        "M3.95,13.05c0-0.93,0.29-1.75,0.87-2.48s1.31-1.2,2.19-1.4c0.26-1.1,0.82-2,1.7-2.71s1.88-1.06,3.01-1.06&#xA;	c1.1,0,2.08,0.35,2.95,1.04s1.43,1.57,1.68,2.65h0.26c1.1,0,2.04,0.39,2.82,1.16c0.78,0.77,1.17,1.71,1.17,2.81&#xA;	c0,0.01,0,0.02,0,0.04c0,0.02,0,0.04,0,0.06c0.75,0.8,1.12,1.75,1.12,2.85c0,0.76-0.19,1.46-0.57,2.1&#xA;	c-0.38,0.65-0.89,1.16-1.53,1.53c-0.64,0.38-1.34,0.56-2.09,0.56c-0.96,0-1.82-0.3-2.56-0.89s-1.24-1.35-1.48-2.26H7.79&#xA;	C6.72,17,5.81,16.59,5.07,15.82S3.95,14.12,3.95,13.05z M5.31,13.05c0,0.7,0.24,1.31,0.73,1.82s1.07,0.79,1.75,0.82h8.99&#xA;	c0.68-0.03,1.27-0.3,1.75-0.82c0.49-0.52,0.73-1.12,0.73-1.82c0-0.71-0.26-1.32-0.79-1.83c-0.53-0.52-1.14-0.77-1.86-0.77h-1.29&#xA;	c-0.09,0-0.14-0.05-0.14-0.14l-0.07-0.47c-0.09-0.87-0.46-1.6-1.12-2.19s-1.42-0.89-2.28-0.89c-0.89,0-1.66,0.29-2.31,0.88&#xA;	S8.4,8.96,8.31,9.83L8.25,10.3c0,0.09-0.05,0.14-0.16,0.14h-0.4C7.02,10.52,6.45,10.8,6,11.3C5.54,11.79,5.31,12.38,5.31,13.05z&#xA;	 M11.51,22.06c-0.25-0.33-0.25-0.65,0-0.98l1.13-1.15c0.14-0.12,0.31-0.18,0.52-0.18c0.19,0,0.34,0.06,0.46,0.18&#xA;	c0.12,0.12,0.18,0.28,0.18,0.47c0,0.2-0.06,0.36-0.18,0.48l-1.14,1.18c-0.12,0.12-0.29,0.19-0.49,0.19&#xA;	C11.79,22.25,11.63,22.18,11.51,22.06z M14.9,17.04c0.21,0.54,0.56,0.97,1.04,1.3c0.48,0.33,1.01,0.5,1.6,0.5&#xA;	c0.77,0,1.43-0.28,1.97-0.83c0.54-0.56,0.81-1.23,0.81-2.02c0-0.39-0.06-0.74-0.19-1.05c-0.33,0.61-0.8,1.11-1.39,1.49&#xA;	c-0.6,0.38-1.25,0.58-1.96,0.61H14.9z M16.85,22.23c0-0.19,0.07-0.34,0.2-0.47c0.13-0.12,0.3-0.19,0.48-0.19&#xA;	c0.18,0,0.35,0.07,0.5,0.21c0.12,0.12,0.19,0.27,0.19,0.45v1.64c0,0.19-0.07,0.35-0.2,0.49c-0.13,0.14-0.3,0.21-0.48,0.21&#xA;	s-0.35-0.07-0.48-0.21c-0.13-0.14-0.2-0.3-0.2-0.49V22.23z M21.26,20.4c0-0.18,0.06-0.33,0.19-0.46c0.13-0.12,0.29-0.19,0.47-0.19&#xA;	c0.19,0,0.35,0.06,0.47,0.18l1.18,1.15c0.13,0.14,0.2,0.3,0.2,0.48c0,0.19-0.07,0.35-0.2,0.48c-0.13,0.13-0.3,0.2-0.49,0.2&#xA;	c-0.21,0-0.37-0.06-0.5-0.19l-1.13-1.18C21.32,20.73,21.26,20.57,21.26,20.4z M21.26,11.59c0-0.19,0.06-0.35,0.19-0.47l1.13-1.18&#xA;	c0.14-0.12,0.3-0.19,0.5-0.19c0.19,0,0.35,0.06,0.5,0.19c0.13,0.15,0.2,0.32,0.2,0.51c0,0.18-0.07,0.33-0.2,0.48l-1.18,1.15&#xA;	c-0.12,0.12-0.28,0.19-0.47,0.19s-0.35-0.06-0.47-0.19C21.32,11.94,21.26,11.78,21.26,11.59z M23.08,15.99&#xA;	c0-0.19,0.06-0.35,0.19-0.48c0.12-0.13,0.28-0.2,0.47-0.2h1.62c0.19,0,0.36,0.07,0.5,0.2s0.21,0.29,0.21,0.48&#xA;	c0,0.19-0.07,0.36-0.21,0.49c-0.14,0.13-0.3,0.2-0.5,0.2h-1.62c-0.19,0-0.34-0.07-0.47-0.2C23.14,16.35,23.08,16.19,23.08,15.99z"
        /> < title > { title } < / title > < / svg >
    }
}
