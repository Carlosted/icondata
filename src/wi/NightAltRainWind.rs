#[cfg(feature = "WiNightAltRainWind")]
use leptos::*;
#[cfg(feature = "WiNightAltRainWind")]
///This icon requires the feature `WiNightAltRainWind` to be enabled.
#[component]
pub fn NightAltRainWind(
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
        "M4.06,16.93c0,1.12,0.33,2.12,1,3c0.67,0.88,1.52,1.47,2.57,1.77c0.09,0.02,0.17-0.01,0.24-0.08L9,20.22&#xA;	c-0.88,0-1.63-0.32-2.27-0.97c-0.64-0.65-0.96-1.42-0.96-2.32c0-0.84,0.28-1.56,0.84-2.17s1.27-0.95,2.11-1.03l0.5-0.07&#xA;	c0.12,0,0.19-0.06,0.19-0.19l0.08-0.53c0.12-1.09,0.59-2,1.41-2.73c0.81-0.73,1.77-1.1,2.86-1.1c1.09,0,2.04,0.37,2.86,1.1&#xA;	s1.29,1.64,1.41,2.72l0.07,0.58c0,0.11,0.06,0.17,0.18,0.17h1.62c0.88,0,1.64,0.32,2.28,0.96s0.96,1.4,0.96,2.28&#xA;	c0,0.85-0.28,1.59-0.84,2.22s-1.25,0.98-2.07,1.05c-0.45,0.06-0.74,0.15-0.86,0.28l-2.33,2.91c-0.16,0.17-0.22,0.38-0.19,0.63&#xA;	c0.02,0.24,0.13,0.43,0.31,0.59c0.18,0.16,0.37,0.23,0.57,0.23c0.23,0,0.44-0.12,0.64-0.38l2.04-2.59c0.62-0.06,1.2-0.24,1.76-0.52&#xA;	c0.55-0.28,1.03-0.65,1.42-1.08c0.39-0.44,0.71-0.95,0.94-1.53c0.23-0.58,0.35-1.18,0.35-1.81c0-0.87-0.23-1.68-0.68-2.44&#xA;	c0.81-0.74,1.34-1.61,1.58-2.62v-0.09l0.2-0.77l-0.76-0.26c-0.57-0.17-1.06-0.45-1.47-0.83s-0.69-0.8-0.86-1.23&#xA;	c-0.17-0.43-0.26-0.87-0.26-1.31c0-0.26,0.03-0.52,0.08-0.8l0.19-0.78l-0.83-0.23c-0.01,0-0.02,0-0.03-0.01s-0.02-0.02-0.04-0.02&#xA;	s-0.03-0.01-0.04-0.02C21.91,5.5,21.9,5.49,21.9,5.49c-0.44-0.11-0.85-0.16-1.25-0.16c-0.38,0.01-0.76,0.05-1.15,0.14&#xA;	s-0.78,0.22-1.2,0.41c-0.42,0.19-0.82,0.46-1.2,0.81s-0.72,0.76-1,1.24c-0.75-0.33-1.53-0.49-2.34-0.49c-1.41,0-2.67,0.44-3.76,1.31&#xA;	s-1.8,1.99-2.11,3.36c-1.13,0.27-2.05,0.86-2.76,1.75S4.06,15.77,4.06,16.93z M7.77,24.92c0,0.13,0.02,0.23,0.07,0.31&#xA;	c0.09,0.22,0.23,0.37,0.43,0.46c0.22,0.1,0.44,0.11,0.67,0.03c0.23-0.08,0.38-0.23,0.46-0.44c0.1-0.22,0.1-0.44,0.01-0.67&#xA;	c-0.09-0.23-0.24-0.38-0.45-0.45c-0.22-0.1-0.44-0.11-0.66-0.02c-0.22,0.08-0.37,0.24-0.45,0.45C7.79,24.67,7.77,24.79,7.77,24.92z&#xA;	 M9.61,22.47v0.11c0.02,0.23,0.13,0.41,0.33,0.55c0.13,0.15,0.31,0.22,0.54,0.22c0.23-0.01,0.45-0.11,0.66-0.32l2.33-2.92&#xA;	c0.14-0.17,0.19-0.38,0.17-0.62c-0.03-0.24-0.12-0.43-0.3-0.58c-0.18-0.14-0.38-0.2-0.63-0.18c-0.24,0.02-0.43,0.14-0.57,0.34&#xA;	l-2.32,2.86C9.68,22.09,9.61,22.27,9.61,22.47z M10.19,27.68c0.09,0.21,0.24,0.36,0.46,0.45c0.11,0.05,0.22,0.08,0.33,0.08&#xA;	c0.06,0,0.16-0.02,0.3-0.06c0.21-0.09,0.36-0.23,0.44-0.44c0.08-0.22,0.08-0.43,0.01-0.65c-0.07-0.21-0.22-0.37-0.44-0.48&#xA;	c-0.22-0.08-0.43-0.08-0.63,0s-0.35,0.23-0.45,0.44C10.1,27.22,10.09,27.43,10.19,27.68z M11.78,25.02v0.08&#xA;	c0.02,0.22,0.13,0.42,0.32,0.58c0.19,0.16,0.38,0.24,0.56,0.24c0.22,0,0.42-0.11,0.6-0.34l4.31-5.36c0.14-0.17,0.21-0.38,0.19-0.62&#xA;	c-0.02-0.24-0.12-0.44-0.29-0.58c-0.2-0.14-0.42-0.2-0.66-0.18c-0.24,0.02-0.43,0.12-0.57,0.3l-4.27,5.36&#xA;	C11.84,24.65,11.78,24.83,11.78,25.02z M15.29,26.13c0,0.11,0.02,0.22,0.07,0.33c0.08,0.23,0.24,0.38,0.47,0.47&#xA;	c0.23,0.09,0.43,0.09,0.61,0.02c0.22-0.09,0.37-0.24,0.46-0.46c0.1-0.22,0.11-0.43,0.03-0.64c-0.08-0.21-0.23-0.36-0.45-0.46&#xA;	c-0.22-0.08-0.44-0.08-0.65,0c-0.22,0.08-0.37,0.22-0.47,0.42C15.31,25.92,15.29,26.03,15.29,26.13z M17.57,8.81&#xA;	c0.31-0.57,0.75-1.01,1.3-1.32c0.55-0.3,1.14-0.45,1.76-0.44c0.12,0,0.21,0,0.26,0.01v0.3c0,0.97,0.27,1.89,0.8,2.75&#xA;	c0.53,0.87,1.26,1.52,2.19,1.96c-0.25,0.47-0.51,0.84-0.79,1.12c-0.89-0.79-1.96-1.18-3.22-1.18h-0.32&#xA;	C19.26,10.74,18.6,9.67,17.57,8.81z"
        /> < title > { title } < / title > < / svg >
    }
}
