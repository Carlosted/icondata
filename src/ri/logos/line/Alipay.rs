#[cfg(feature = "RiLogosLineAlipay")]
use leptos::*;
#[cfg(feature = "RiLogosLineAlipay")]
///This icon requires the feature `RiLogosLineAlipay` to be enabled.
#[component]
pub fn Alipay(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M18.408 16.79c-2.173-.95-3.72-1.646-4.64-2.086-1.4 1.696-2.872 2.72-5.08 2.72S5 16.064 5.176 14.392c.12-1.096.872-2.888 4.128-2.576 1.72.16 2.504.48 3.912.944.36-.664.664-1.4.888-2.176H7.88v-.616h3.072V8.864H7.2v-.68h3.752V6.592s.032-.248.312-.248H12.8v1.848h4v.68h-4v1.104h3.264a12.41 12.41 0 0 1-1.32 3.32c.51.182 2.097.676 4.76 1.483a8 8 0 1 0-1.096 2.012zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm-3.568-5.632c1.44 0 2.824-.872 3.96-2.352-1.608-.776-2.944-1.16-4.44-1.16-1.304 0-1.984.8-2.104 1.416-.12.616.248 2.096 2.584 2.096z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
