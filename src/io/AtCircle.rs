#[cfg(feature = "IoAtCircle")]
use leptos::*;
#[cfg(feature = "IoAtCircle")]
///This icon requires the feature `IoAtCircle` to be enabled.
#[component]
pub fn AtCircle(
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
        "M255.46,48.74c-114.84,0-208,93.11-208,208s93.12,208,208,208,208-93.12,208-208S370.31,48.74,255.46,48.74ZM380.28,252c-2.85,32.63-16.79,49.7-28,58.26S327.61,322.58,316,320.5a41.61,41.61,0,0,1-26.82-17.19,62.06,62.06,0,0,1-44,17.57,51.66,51.66,0,0,1-38.55-16.83c-11.38-12.42-17-30.36-15.32-49.23,3-35,30.91-57.39,56.87-61.48,27.2-4.29,52.23,6.54,62.9,19.46l3.85,4.66-6.34,50.38c-1.19,14.34,3.28,23.48,12.29,25.1,2.39.42,8.1-.13,14.37-4.93,6.72-5.15,15.14-16,17.1-38.47C354.7,223,348,200.35,333.1,184.05c-15.49-16.9-39.09-25.84-68.23-25.84-54,0-101.81,44.43-106.58,99-2.28,26.2,5.67,50.68,22.4,68.93C197.05,344,220,353.88,245.35,353.88c19,0,30.61-2.05,49.48-8.78a14,14,0,0,1,9.4,26.38c-21.82,7.77-36.68,10.4-58.88,10.4-33.28,0-63.57-13.06-85.3-36.77C138,321,127.42,288.94,130.4,254.82c2.91-33.33,18.45-64.63,43.77-88.12s57.57-36.49,90.7-36.49c37.2,0,67.93,12.08,88.87,34.93C373.83,187.05,383.25,217.89,380.28,252Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M252.57,221c-14.83,2.33-31.56,15.84-33.34,36.26-1,11.06,2,21.22,8.07,27.87a23.65,23.65,0,0,0,17.91,7.75c20.31,0,34.73-14.94,36.75-38.06a14,14,0,0,1,.34-2.07l3.2-25.45a49.61,49.61,0,0,0-32.93-6.3Z"
        /> < title > { title } < / title > < / svg >
    }
}
