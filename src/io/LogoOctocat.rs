#[cfg(feature = "IoLogoOctocat")]
use leptos::*;
#[cfg(feature = "IoLogoOctocat")]
///This icon requires the feature `IoLogoOctocat` to be enabled.
#[component]
pub fn LogoOctocat(
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
        "M172.86,290.12c-9.75,0-18.11,4.56-24.86,13.87s-10.07,20.58-10.07,34,3.43,24.91,10.07,34.12S163,386,172.86,386c9.1,0,17-4.66,23.68-13.87s10.07-20.58,10.07-34.12-3.43-24.81-10.07-34S182,290.12,172.86,290.12Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M340.32,290.12c-9.64,0-18.11,4.56-24.86,13.87s-10.07,20.58-10.07,34,3.43,24.91,10.07,34.12S330.57,386,340.32,386c9.11,0,17-4.66,23.79-13.87s10.07-20.58,10.07-34.12-3.43-24.81-10.07-34S349.54,290.12,340.32,290.12Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M459.36,165h0c-.11,0,2.89-15.49.32-42.47-2.36-27-8-51.78-17.25-74.53,0,0-4.72.87-13.72,3.14S405,58,384.89,67.18c-19.82,9.2-40.71,21.44-62.46,36.29-14.79-4.23-36.86-6.39-66.43-6.39-28.18,0-50.25,2.16-66.43,6.39Q117.9,53.25,69.46,48,55.65,82.13,52.32,122.75c-2.57,27,.43,42.58.43,42.58C26.71,193.82,16,234.88,16,268.78c0,26.22.75,49.94,6.54,71,6,20.91,13.6,38,22.6,51.14A147.49,147.49,0,0,0,79,425.43c13.39,10.08,25.71,17.34,36.86,21.89,11.25,4.76,24,8.23,38.57,10.72a279.19,279.19,0,0,0,32.68,4.34s30,1.62,69,1.62S325,462.38,325,462.38A285.25,285.25,0,0,0,357.68,458a178.91,178.91,0,0,0,38.46-10.72c11.15-4.66,23.47-11.81,37-21.89a145,145,0,0,0,33.75-34.55c9-13.11,16.6-30.23,22.6-51.14S496,294.89,496,268.67C496,235.85,485.29,194.25,459.36,165ZM389.29,418.07C359.39,432.26,315.46,438,257.18,438h-2.25c-58.29,0-102.22-5.63-131.57-19.93s-44.25-43.45-44.25-87.43c0-26.32,9.21-47.66,27.32-64,7.93-7,17.57-11.92,29.57-14.84s22.93-3,33.21-2.71c10.08.43,24.22,2.38,42.11,3.79s31.39,3.25,44.79,3.25c12.53,0,29.14-2.17,55.82-4.33s46.61-3.25,59.46-1.09c13.18,2.17,24.65,6.72,34.4,15.93q28.44,25.67,28.5,64C434.18,374.62,419.07,403.88,389.29,418.07Z"
        /> < title > { title } < / title > < / svg >
    }
}
