#[cfg(feature = "HiMdSolidForward")]
use leptos::*;
#[cfg(feature = "HiMdSolidForward")]
///This icon requires the feature `HiMdSolidForward` to be enabled.
#[component]
pub fn Forward(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.28824 4.81895C2.28891 4.20172 1 4.92057 1 6.09515V13.9056C1 15.0802 2.2889 15.7991 3.28824 15.1818L9.61101 11.2766C9.76598 11.1809 9.89564 11.064 10 10.9328V13.9056C10 15.0802 11.2889 15.7991 12.2882 15.1818L18.611 11.2766C19.56 10.6904 19.56 9.31035 18.611 8.72419L12.2882 4.81895C11.2889 4.20172 10 4.92057 10 6.09515V9.06794C9.89564 8.93679 9.76598 8.81991 9.61101 8.72419L3.28824 4.81895Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
