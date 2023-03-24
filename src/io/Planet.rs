#[cfg(feature = "IoPlanet")]
use leptos::*;
#[cfg(feature = "IoPlanet")]
///This icon requires the feature `IoPlanet` to be enabled.
#[component]
pub fn Planet(
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
        "M96.85,286.62a8,8,0,0,0-12.53,8.25C102.07,373.28,172.3,432,256,432a175.31,175.31,0,0,0,52.41-8,8,8,0,0,0,.79-15,1120,1120,0,0,1-109.48-55.61A1126.24,1126.24,0,0,1,96.85,286.62Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M492.72,339.51c-4.19-5.58-9.11-11.44-14.7-17.53a15.83,15.83,0,0,0-26.56,5.13c0,.16-.11.31-.17.47a15.75,15.75,0,0,0,3.15,16.06c22.74,25,26.42,38.51,25.48,41.36-2,2.23-17.05,6.89-58.15-3.53q-8.83-2.24-19.32-5.46-6.76-2.08-13.79-4.49h0a176.76,176.76,0,0,0,19.54-27.25c.17-.29.35-.58.52-.88A175.39,175.39,0,0,0,432,256,178.87,178.87,0,0,0,431,237C421.43,148.83,346.6,80,256,80A175.37,175.37,0,0,0,149.6,115.89a177.4,177.4,0,0,0-45.83,51.84c-.16.29-.34.58-.51.87a175.48,175.48,0,0,0-13.83,30.52q-5.59-4.87-10.79-9.67c-5.39-5-10.17-9.63-14.42-14C34.65,145.19,31.13,129.84,32.06,127c2-2.23,15.54-5.87,48.62,1.31A15.82,15.82,0,0,0,96.22,123l.36-.44a15.74,15.74,0,0,0-8.67-25.43A237.38,237.38,0,0,0,64.13,93C33.41,89.47,13.3,95.52,4.35,111,1.11,116.58-2,126.09,1.63,139.6,7,159.66,26.14,184,53.23,209.5c8.63,8.13,18.06,16.37,28.12,24.64,7.32,6,15,12.06,22.9,18.08q7.91,6,16.15,12T137.1,276c25.41,17.61,52.26,34.52,78.59,49.69q14.34,8.26,28.64,16t28.37,14.81c21.9,11,43.35,20.92,63.86,29.43q13.19,5.48,25.81,10.16c11.89,4.42,23.37,8.31,34.31,11.59l1.1.33c25.73,7.66,47.42,11.69,64.48,12H464c21.64,0,36.3-6.38,43.58-19C516.67,385.39,511.66,364.69,492.72,339.51Z"
        /> < title > { title } < / title > < / svg >
    }
}
