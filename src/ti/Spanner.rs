#[cfg(feature = "TiSpanner")]
use leptos::*;
#[cfg(feature = "TiSpanner")]
///This icon requires the feature `TiSpanner` to be enabled.
#[component]
pub fn Spanner(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.285 7.119c-.05-.168-.184-.299-.354-.344-.172-.047-.352.003-.477.126l-2.616 2.557-1.914-.383-.381-1.907 2.645-2.585c.126-.123.178-.303.137-.474s-.168-.308-.336-.361c-.531-.166-1.018-.248-1.489-.248-2.757 0-5 2.243-5 5 0 .323.038.65.118 1.01-.562.463-1.096.862-1.701 1.314-.865.646-1.845 1.377-3.182 2.506-.785.686-1.235 1.659-1.235 2.67 0 1.93 1.57 3.5 3.5 3.5 1.021 0 1.993-.456 2.662-1.25 1.149-1.347 1.891-2.336 2.544-3.209.442-.591.832-1.111 1.283-1.66.36.081.688.119 1.011.119 2.757 0 5-2.243 5-5 0-.437-.068-.875-.215-1.381zm-12.285 9.881c-.553 0-1-.447-1-1s.447-1 1-1 1 .447 1 1-.447 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
