#[cfg(feature = "IoLogoCodepen")]
use leptos::*;
#[cfg(feature = "IoLogoCodepen")]
///This icon requires the feature `IoLogoCodepen` to be enabled.
#[component]
pub fn LogoCodepen(
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
        "M241.24,303.94c-15.32-10.36-30.74-20.57-46.06-30.93-2-1.38-3.43-1.48-5.5,0L150.8,299.13C182,319.9,244,361.32,244,361.32V307.53C244,306.31,242.45,304.75,241.24,303.94Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M195.09,240.67q23.19-15.24,46.11-30.86a7.54,7.54,0,0,0,2.8-5.34v-51.7s-62,41.12-93.26,61.94c13.7,9.16,26.67,17.91,39.78,26.44C191.54,241.81,193.92,241.43,195.09,240.67Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M269.84,209.35q23.71,16.07,47.63,31.82a4.3,4.3,0,0,0,3.83,0l39.76-26.47L268,152.48v53.35A4.79,4.79,0,0,0,269.84,209.35Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M258.11,230.37a5.27,5.27,0,0,0-4.74.17c-4.82,3-9.47,6.2-14.17,9.35-8.25,5.53-25.35,17-25.35,17l38.84,25.86a6.18,6.18,0,0,0,6.26.11l39-26S263.88,234.2,258.11,230.37Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "141 237.12 141 276.73 170.62 256.89 141 237.12" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,32C132.29,32,32,132.29,32,256S132.29,480,256,480,480,379.71,480,256,379.71,32,256,32ZM395,297c0,5.78-2.65,9.86-7.51,13.09q-61.71,41-123.29,82.19c-5.85,3.92-11.17,3.75-17-.14q-61.17-41-122.63-81.67c-5.11-3.39-7.59-7.56-7.59-13.73V217c0-6.14,2.52-10.34,7.62-13.72,40.91-27.13,81.94-54.36,122.73-81.68,5.82-3.89,11.09-4,16.94-.09q61.54,41.21,123.26,82.19c4.68,3.11,7.45,6.95,7.45,12.66Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M316.25,273.23q-22.59,15.34-45.39,30.34c-2.41,1.58-2.89,3.31-2.86,6.19V361.1l93-62-38.53-25.88C320.17,271.61,318.58,271.65,316.25,273.23Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "370 276.68 370 237.06 340.41 256.93 370 276.68" /> < title > { title } < / title
        > < / svg >
    }
}
