#[cfg(feature = "IoWalk")]
use leptos::*;
#[cfg(feature = "IoWalk")]
///This icon requires the feature `IoWalk` to be enabled.
#[component]
pub fn Walk(
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
        "M312.55,479.9l-56.42-114-44.62-57A72.37,72.37,0,0,1,201.45,272V143.64H217a40,40,0,0,1,40,40V365.85"
        style =
        "stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M127.38,291.78V217.71s37-74.07,74.07-74.07" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368.09,291.78a18.49,18.49,0,0,1-10.26-3.11L297.7,250A21.18,21.18,0,0,1,288,232.21v-23.7a5.65,5.65,0,0,1,8.69-4.77l81.65,54.11a18.52,18.52,0,0,1-10.29,33.93Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M171.91,493.47a18.5,18.5,0,0,1-14.83-7.41c-6.14-8.18-4-17.18,3.7-25.92l59.95-74.66a7.41,7.41,0,0,1,10.76,2.06c1.56,2.54,3.38,5.65,5.19,9.09,5.24,9.95,6,16.11-1.68,25.7-8,10-52,67.44-52,67.44C180.38,492.75,175.77,493.47,171.91,493.47Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "257" cy = "69.56" r =
        "37.04" style =
        "stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:16px" /> <
        title > { title } < / title > < / svg >
    }
}
