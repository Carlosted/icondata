#[cfg(feature = "IoHandLeft")]
use leptos::*;
#[cfg(feature = "IoHandLeft")]
///This icon requires the feature `IoHandLeft` to be enabled.
#[component]
pub fn HandLeft(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432.8,211.44h0c-15.52-8.82-34.91-2.28-43.31,13.68l-41.38,84.41a7,7,0,0,1-8.93,3.43h0a7,7,0,0,1-4.41-6.52V72c0-13.91-12.85-24-26.77-24s-26,10.09-26,24V228.64A11.24,11.24,0,0,1,271.21,240,11,11,0,0,1,260,229V24c0-13.91-10.94-24-24.86-24S210,10.09,210,24V228.64A11.24,11.24,0,0,1,199.21,240,11,11,0,0,1,188,229V56c0-13.91-12.08-24-26-24s-26,11.09-26,25V244.64A11.24,11.24,0,0,1,125.21,256,11,11,0,0,1,114,245V120c0-13.91-11.08-24-25-24s-25.12,10.22-25,24V336c0,117.41,72,176,160,176h16c88,0,115.71-39.6,136-88l68.71-169C451.33,237,448.31,220.25,432.8,211.44Z"
        /> < title > { title } < / title > < / svg >
    }
}
