#[cfg(feature = "HiLgSolidCloudArrowUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidCloudArrowUp")]
///This icon requires the feature `HiLgSolidCloudArrowUp` to be enabled.
#[component]
pub fn CloudArrowUp(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.5 3.75C7.18629 3.75 4.5 6.43629 4.5 9.75C4.5 9.91685 4.50683 10.0822 4.52024 10.2459C2.73627 11.084 1.5 12.8968 1.5 15C1.5 17.8995 3.85051 20.25 6.75 20.25H18C20.4853 20.25 22.5 18.2353 22.5 15.75C22.5 14.0653 21.5744 12.5981 20.2058 11.827C20.2349 11.6386 20.25 11.4459 20.25 11.25C20.25 9.17893 18.5711 7.5 16.5 7.5C16.3559 7.5 16.2135 7.50816 16.0733 7.52408C15.1893 5.31282 13.028 3.75 10.5 3.75ZM12.5303 9.21967C12.2374 8.92678 11.7626 8.92678 11.4697 9.21967L8.46967 12.2197C8.17678 12.5126 8.17678 12.9874 8.46967 13.2803C8.76256 13.5732 9.23744 13.5732 9.53033 13.2803L11.25 11.5607L11.25 16.5C11.25 16.9142 11.5858 17.25 12 17.25C12.4142 17.25 12.75 16.9142 12.75 16.5V11.5607L14.4697 13.2803C14.7626 13.5732 15.2374 13.5732 15.5303 13.2803C15.8232 12.9874 15.8232 12.5126 15.5303 12.2197L12.5303 9.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
