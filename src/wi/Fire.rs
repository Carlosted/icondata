#[cfg(feature = "WiFire")]
use leptos::*;
#[cfg(feature = "WiFire")]
///This icon requires the feature `WiFire` to be enabled.
#[component]
pub fn Fire(
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
        "M7.38,21.83c0-0.3,0.1-0.55,0.29-0.76c0.19-0.21,0.43-0.31,0.7-0.31v-0.02l13.16,0.04c0.06-0.01,0.1-0.02,0.1-0.02&#xA;	c0.27,0.01,0.51,0.12,0.7,0.33c0.19,0.21,0.29,0.47,0.28,0.77c0,0.3-0.1,0.55-0.29,0.76c-0.19,0.21-0.43,0.31-0.7,0.31v0.01&#xA;	L8.59,22.9c-0.1,0.01-0.17,0.02-0.22,0.02c-0.28-0.01-0.51-0.11-0.7-0.32C7.47,22.39,7.37,22.13,7.38,21.83z M8.2,16.37&#xA;	c-0.01-0.43,0.04-0.93,0.16-1.52c0.06-0.3,0.2-0.76,0.44-1.37c0.02-0.05,0.07-0.14,0.13-0.28c0.01,0.02,0.03,0.03,0.04,0.05&#xA;	s0.02,0.02,0.02,0.03c0.11,0.44,0.27,0.84,0.49,1.2c0.21,0.32,0.48,0.56,0.82,0.69c0.26,0.11,0.63,0.17,1.1,0.18&#xA;	c0.02,0,0.05,0,0.08,0c0.03,0,0.06,0,0.08,0c-0.33-0.33-0.59-0.67-0.79-1c-0.3-0.52-0.49-1.12-0.57-1.81&#xA;	c-0.06-0.54-0.03-1.19,0.09-1.96c0.02-0.15,0.12-0.49,0.29-1.01c0.15-0.47,0.36-0.9,0.64-1.28C11.54,7.8,12,7.3,12.61,6.78&#xA;	c0.37-0.31,0.89-0.67,1.56-1.07c0.07-0.04,0.18-0.11,0.35-0.19c0,0.02,0,0.04,0,0.05s0,0.03,0,0.04v0.02&#xA;	c-0.24,0.57-0.41,1.15-0.49,1.73c-0.06,0.53,0.02,1.02,0.24,1.48c0.17,0.36,0.48,0.75,0.92,1.15c0.09,0.09,0.29,0.29,0.6,0.58&#xA;	c0.3,0.29,0.54,0.52,0.7,0.68l0.25,0.25c0.26-0.38,0.41-0.83,0.44-1.35c0.04-0.55,0-1.15-0.14-1.8c0-0.01,0-0.04,0.01-0.11&#xA;	c0.02,0.02,0.13,0.1,0.3,0.24c0.56,0.5,0.98,0.95,1.28,1.34c0.48,0.62,0.83,1.21,1.06,1.74c0.19,0.46,0.31,0.92,0.38,1.4&#xA;	c0.06,0.42,0.08,0.77,0.07,1.05c-0.01,0.78-0.1,1.43-0.25,1.96c-0.07,0.21-0.13,0.38-0.19,0.52c0.25-0.07,0.47-0.16,0.65-0.26&#xA;	c0.25-0.16,0.45-0.37,0.6-0.66c0.16-0.29,0.29-0.62,0.38-0.98c0-0.01,0.01-0.03,0.03-0.05c0.01,0.02,0.02,0.05,0.05,0.09&#xA;	c0.02,0.04,0.04,0.07,0.05,0.1c0.13,0.31,0.22,0.63,0.27,0.97c0.08,0.38,0.1,0.75,0.08,1.13c-0.02,0.29-0.07,0.56-0.16,0.81&#xA;	c-0.08,0.24-0.16,0.43-0.22,0.58c-0.19,0.38-0.39,0.71-0.62,0.98c-0.06,0.07-0.11,0.13-0.14,0.16H9.67&#xA;	c-0.01-0.01-0.03-0.03-0.07-0.06s-0.06-0.05-0.08-0.07C9.26,18.98,8.98,18.6,8.7,18.1c-0.08-0.15-0.18-0.38-0.29-0.69&#xA;	C8.29,17.1,8.22,16.75,8.2,16.37z"
        /> < title > { title } < / title > < / svg >
    }
}
