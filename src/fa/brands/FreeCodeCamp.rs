#[cfg(feature = "FaBrandsFreeCodeCamp")]
use leptos::*;
#[cfg(feature = "FaBrandsFreeCodeCamp")]
///This icon requires the feature `FaBrandsFreeCodeCamp` to be enabled.
#[component]
pub fn FreeCodeCamp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M97.22,96.21c10.36-10.65,16-17.12,16-21.9,0-2.76-1.92-5.51-3.83-7.42A14.81,14.81,0,0,0,101,64.05c-8.48,0-20.92,8.79-35.84,25.69C23.68,137,2.51,182.81,3.37,250.34s17.47,117,54.06,161.87C76.22,435.86,90.62,448,100.9,448a13.55,13.55,0,0,0,8.37-3.84c1.91-2.76,3.81-5.63,3.81-8.38,0-5.63-3.86-12.2-13.2-20.55-44.45-42.33-67.32-97-67.48-165C32.25,188.8,54,137.83,97.22,96.21ZM239.47,420.07c.58.37.91.55.91.55Zm93.79.55.17-.13C333.24,420.62,333.17,420.67,333.26,420.62Zm3.13-158.18c-16.24-4.15,50.41-82.89-68.05-177.17,0,0,15.54,49.38-62.83,159.57-74.27,104.35,23.46,168.73,34,175.23-6.73-4.35-47.4-35.7,9.55-128.64,11-18.3,25.53-34.87,43.5-72.16,0,0,15.91,22.45,7.6,71.13C287.7,364,354,342.91,355,343.94c22.75,26.78-17.72,73.51-21.58,76.55,5.49-3.65,117.71-78,33-188.1C360.43,238.4,352.62,266.59,336.39,262.44ZM510.88,89.69C496,72.79,483.52,64,475,64a14.81,14.81,0,0,0-8.39,2.84c-1.91,1.91-3.83,4.66-3.83,7.42,0,4.78,5.6,11.26,16,21.9,43.23,41.61,65,92.59,64.82,154.06-.16,68-23,122.63-67.48,165-9.34,8.35-13.18,14.92-13.2,20.55,0,2.75,1.9,5.62,3.81,8.38A13.61,13.61,0,0,0,475.1,448c10.28,0,24.68-12.13,43.47-35.79,36.59-44.85,53.14-94.38,54.06-161.87S552.32,137,510.88,89.69Z"
        /> < title > { title } < / title > < / svg >
    }
}
