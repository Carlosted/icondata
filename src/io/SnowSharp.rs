#[cfg(feature = "IoSnowSharp")]
use leptos::*;
#[cfg(feature = "IoSnowSharp")]
///This icon requires the feature `IoSnowSharp` to be enabled.
#[component]
pub fn SnowSharp(
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
        "M447.88,313.27l19.25-10.64-21.28-38.51L426.6,274.76a133.42,133.42,0,0,0-38.54,32.1L300,256l88.07-50.86a133.42,133.42,0,0,0,38.54,32.1l19.25,10.64,21.28-38.51-19.25-10.64a89.27,89.27,0,0,1-20.93-16L480,152.05,458,114,405,144.58a89.07,89.07,0,0,1-3.42-26.15l.41-22-44-.82-.41,22A133.62,133.62,0,0,0,366.07,167L278,217.89V116.18a133.52,133.52,0,0,0,47.06-17.33L343.9,87.5,321.19,49.81,302.35,61.16A89.5,89.5,0,0,1,278,71.27V16H234V71.27a89.5,89.5,0,0,1-24.35-10.11L190.81,49.81,168.1,87.5l18.84,11.35A133.52,133.52,0,0,0,234,116.18V217.89L145.93,167a133.62,133.62,0,0,0,8.53-49.43l-.41-22-44,.82.41,22a89.07,89.07,0,0,1-3.42,26.15L54,114l-22,38.1,53.05,30.64a89.27,89.27,0,0,1-20.93,16L44.87,209.37l21.28,38.51L85.4,237.24a133.42,133.42,0,0,0,38.54-32.1L212,256l-88.07,50.86a133.42,133.42,0,0,0-38.54-32.1L66.15,264.12,44.87,302.63l19.25,10.64a89.27,89.27,0,0,1,20.93,16L32,360l22,38.1,53.05-30.63a89.07,89.07,0,0,1,3.42,26.15l-.41,22,44,.82.41-22A133.62,133.62,0,0,0,145.93,345L234,294.11V395.82a133.52,133.52,0,0,0-47.06,17.33L168.1,424.5l22.71,37.69,18.84-11.35A89.5,89.5,0,0,1,234,440.73V496h44V440.73a89.5,89.5,0,0,1,24.35,10.11l18.84,11.35L343.9,424.5l-18.84-11.35A133.52,133.52,0,0,0,278,395.82V294.11L366.07,345a133.62,133.62,0,0,0-8.53,49.43l.41,22,44-.82-.41-22A89.07,89.07,0,0,1,405,367.42L458,398.05,480,360,427,329.31A89.27,89.27,0,0,1,447.88,313.27Z"
        /> < title > { title } < / title > < / svg >
    }
}
