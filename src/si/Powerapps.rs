#[cfg(feature = "SiPowerapps")]
use leptos::*;
#[cfg(feature = "SiPowerapps")]
///This icon requires the feature `SiPowerapps` to be enabled.
#[component]
pub fn Powerapps(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.012,18.027L14.261,23.21C14.072,23.417 13.803,23.535 13.523,23.535C13.242,23.535 12.974,23.417 12.784,23.21L8.636,18.685C8.286,18.304 8.286,17.712 8.636,17.332L12.902,12.677C13.251,12.296 13.251,11.704 12.902,11.323L8.636,6.668C8.286,6.288 8.286,5.696 8.636,5.315L12.784,0.79C12.974,0.583 13.242,0.465 13.523,0.465C13.803,0.465 14.072,0.583 14.261,0.79L19.012,5.973C18.598,5.977 18.203,6.153 17.924,6.459L14.084,10.647C13.387,11.409 13.387,12.591 14.084,13.353L17.924,17.541C18.217,17.861 18.614,18.023 19.012,18.027ZM11.399,22.438L10.772,23.154C10.582,23.372 10.307,23.496 10.018,23.496C9.73,23.496 9.455,23.372 9.265,23.154L0.371,12.989C-0.124,12.426 -0.124,11.574 0.371,11.011L9.265,0.846C9.455,0.628 9.73,0.504 10.018,0.504C10.307,0.504 10.582,0.628 10.772,0.846L11.399,1.562L8.268,4.978C7.743,5.548 7.743,6.436 8.268,7.006L12.534,11.661C12.708,11.852 12.708,12.148 12.534,12.339L8.268,16.994C7.743,17.564 7.743,18.452 8.268,19.022L11.399,22.438ZM19.756,17.216C19.567,17.414 19.304,17.527 19.03,17.527C18.749,17.527 18.482,17.409 18.292,17.203L14.453,13.015C13.93,12.444 13.93,11.556 14.453,10.985L18.292,6.797C18.482,6.591 18.749,6.473 19.03,6.473C19.304,6.473 19.567,6.586 19.756,6.784L23.606,10.985C24.131,11.556 24.131,12.444 23.606,13.015L19.756,17.216Z"
        /> < title > { title } < / title > < / svg >
    }
}
