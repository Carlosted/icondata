#[cfg(feature = "IoTennisballSharp")]
use leptos::*;
#[cfg(feature = "IoTennisballSharp")]
///This icon requires the feature `IoTennisballSharp` to be enabled.
#[component]
pub fn TennisballSharp(
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
        "M246.4,480a181,181,0,0,0,3.22-22.86c.35-4.61.53-9.31.53-14,0-100-81.34-181.32-181.32-181.32A181.72,181.72,0,0,0,32,265.61,224.2,224.2,0,0,0,246.4,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M284.63,227.37A222.73,222.73,0,0,1,219,68.83a227.09,227.09,0,0,1,2.62-34.42A224.41,224.41,0,0,0,34.41,221.58,227.09,227.09,0,0,1,68.83,219a222.73,222.73,0,0,1,158.54,65.67A222.73,222.73,0,0,1,293,443.17c0,5.74-.22,11.54-.65,17.23s-1.11,11.51-2,17.2A224.42,224.42,0,0,0,477.59,290.42,227.09,227.09,0,0,1,443.17,293,222.73,222.73,0,0,1,284.63,227.37Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M443.17,250.15A181.72,181.72,0,0,0,480,246.39,224.2,224.2,0,0,0,265.61,32a181.72,181.72,0,0,0-3.76,36.83C261.85,168.81,343.19,250.15,443.17,250.15Z"
        /> < title > { title } < / title > < / svg >
    }
}
