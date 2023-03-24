#[cfg(feature = "IoAtCircleSharp")]
use leptos::*;
#[cfg(feature = "IoAtCircleSharp")]
///This icon requires the feature `IoAtCircleSharp` to be enabled.
#[component]
pub fn AtCircleSharp(
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
        "M256.41,48.74c-114.85,0-208,93.11-208,208s93.12,208,208,208,208-93.12,208-208S371.25,48.74,256.41,48.74ZM381.22,252c-2.85,32.63-16.78,49.7-28,58.26s-24.69,12.34-36.33,10.26a41.63,41.63,0,0,1-27-17.52,60.84,60.84,0,0,1-43.72,17.9,51.65,51.65,0,0,1-38.55-16.83c-11.39-12.42-17-30.36-15.33-49.23,3.05-35,30.92-57.39,56.87-61.48,27.21-4.29,52.24,6.54,62.91,19.46l3.84,4.66-6.34,50.38c-1.19,14.34,3.29,23.48,12.29,25.1,2.39.42,8.11-.13,14.38-4.93,6.72-5.15,15.14-16,17.1-38.47,2.32-26.55-4.35-49.19-19.28-65.49-15.5-16.9-39.1-25.84-68.24-25.84-54,0-101.81,44.43-106.57,99-2.29,26.2,5.66,50.68,22.39,68.93C198,344,221,353.88,246.29,353.88c19,0,30.61-2.05,49.49-8.78L309,340.41l9.39,26.37-13.19,4.7c-21.82,7.77-36.68,10.4-58.88,10.4-33.28,0-63.57-13.06-85.3-36.77-22.09-24.11-32.62-56.17-29.65-90.29,2.91-33.33,18.46-64.63,43.77-88.12s57.57-36.49,90.7-36.49c37.2,0,67.94,12.08,88.88,34.93C374.77,187.05,384.2,217.89,381.22,252Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M253.51,221c-14.83,2.33-31.55,15.84-33.34,36.26-1,11.06,2,21.22,8.08,27.87a23.63,23.63,0,0,0,17.91,7.75c19.7,0,33.8-14.79,36.8-38.59l1.75-13.89h.09l1.65-13.11a49.63,49.63,0,0,0-32.94-6.3Z"
        /> < title > { title } < / title > < / svg >
    }
}
