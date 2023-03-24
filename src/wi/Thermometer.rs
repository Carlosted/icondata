#[cfg(feature = "WiThermometer")]
use leptos::*;
#[cfg(feature = "WiThermometer")]
///This icon requires the feature `WiThermometer` to be enabled.
#[component]
pub fn Thermometer(
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
        "M9.91,19.56c0-0.85,0.2-1.64,0.59-2.38s0.94-1.35,1.65-1.84V5.42c0-0.8,0.27-1.48,0.82-2.03S14.2,2.55,15,2.55&#xA;	c0.81,0,1.49,0.28,2.04,0.83c0.55,0.56,0.83,1.23,0.83,2.03v9.92c0.71,0.49,1.25,1.11,1.64,1.84s0.58,1.53,0.58,2.38&#xA;	c0,0.92-0.23,1.78-0.68,2.56s-1.07,1.4-1.85,1.85s-1.63,0.68-2.56,0.68c-0.92,0-1.77-0.23-2.55-0.68s-1.4-1.07-1.86-1.85&#xA;	S9.91,20.48,9.91,19.56z M11.67,19.56c0,0.93,0.33,1.73,0.98,2.39c0.65,0.66,1.44,0.99,2.36,0.99c0.93,0,1.73-0.33,2.4-1&#xA;	s1.01-1.46,1.01-2.37c0-0.62-0.16-1.2-0.48-1.73c-0.32-0.53-0.76-0.94-1.32-1.23l-0.28-0.14c-0.1-0.04-0.15-0.14-0.15-0.29V5.42&#xA;	c0-0.32-0.11-0.59-0.34-0.81C15.62,4.4,15.34,4.29,15,4.29c-0.32,0-0.6,0.11-0.83,0.32c-0.23,0.21-0.34,0.48-0.34,0.81v10.74&#xA;	c0,0.15-0.05,0.25-0.14,0.29l-0.27,0.14c-0.55,0.29-0.98,0.7-1.29,1.23C11.82,18.35,11.67,18.92,11.67,19.56z M12.45,19.56&#xA;	c0,0.71,0.24,1.32,0.73,1.82s1.07,0.75,1.76,0.75s1.28-0.25,1.79-0.75c0.51-0.5,0.76-1.11,0.76-1.81c0-0.63-0.22-1.19-0.65-1.67&#xA;	c-0.43-0.48-0.96-0.77-1.58-0.85V9.69c0-0.06-0.03-0.13-0.1-0.19c-0.07-0.07-0.14-0.1-0.22-0.1c-0.09,0-0.16,0.03-0.21,0.08&#xA;	c-0.05,0.06-0.08,0.12-0.08,0.21v7.34c-0.61,0.09-1.13,0.37-1.56,0.85C12.66,18.37,12.45,18.92,12.45,19.56z"
        /> < title > { title } < / title > < / svg >
    }
}
