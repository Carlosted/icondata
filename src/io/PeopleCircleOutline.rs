#[cfg(feature = "IoPeopleCircleOutline")]
use leptos::*;
#[cfg(feature = "IoPeopleCircleOutline")]
///This icon requires the feature `IoPeopleCircleOutline` to be enabled.
#[component]
pub fn PeopleCircleOutline(
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
        "M256,464C141.31,464,48,370.69,48,256S141.31,48,256,48s208,93.31,208,208S370.69,464,256,464Zm0-384C159,80,80,159,80,256S159,432,256,432s176-78.95,176-176S353.05,80,256,80Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M323.67,292c-17.4,0-34.21-7.72-47.34-21.73a83.76,83.76,0,0,1-22-51.32c-1.47-20.7,4.88-39.75,17.88-53.62S303.38,144,323.67,144c20.14,0,38.37,7.62,51.33,21.46s19.47,33,18,53.51h0a84,84,0,0,1-22,51.3C357.86,284.28,341.06,292,323.67,292Zm55.81-74h0Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M163.82,295.36c-29.76,0-55.93-27.51-58.33-61.33-1.23-17.32,4.15-33.33,15.17-45.08s26.22-18,43.15-18,32.12,6.44,43.07,18.14,16.5,27.82,15.25,45C219.69,267.86,193.53,295.36,163.82,295.36Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M420.37,355.28c-1.59-4.7-5.46-9.71-13.22-14.46-23.46-14.33-52.32-21.91-83.48-21.91-30.57,0-60.23,7.9-83.53,22.25-26.25,16.17-43.89,39.75-51,68.18-1.68,6.69-4.13,19.14-1.51,26.11a192.18,192.18,0,0,0,232.75-80.17Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M163.63,401.37c7.07-28.21,22.12-51.73,45.47-70.75a8,8,0,0,0-2.59-13.77c-12-3.83-25.7-5.88-42.69-5.88-23.82,0-49.11,6.45-68.14,18.17-5.4,3.33-10.7,4.61-14.78,5.75a192.84,192.84,0,0,0,77.78,86.64l1.79-.14A102.82,102.82,0,0,1,163.63,401.37Z"
        /> < title > { title } < / title > < / svg >
    }
}
