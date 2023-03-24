#[cfg(feature = "FaSolidRibbon")]
use leptos::*;
#[cfg(feature = "FaSolidRibbon")]
///This icon requires the feature `FaSolidRibbon` to be enabled.
#[component]
pub fn Ribbon(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M315.2 322.8l0 0-133.9-146 0 0L128 118.6c7.8-5.1 37-22.6 78-22.6s70.2 17.4 78 22.6L227.7 180l85.6 93.4 27.4-29.8c16.3-17.7 25.3-40.9 25.3-65V149.1c0-19-5.6-37.5-16.1-53.3L309.8 35.6C294.9 13.4 269.9 0 243.2 0h-76c-25.8 0-50.1 12.5-65.1 33.5L63.9 87C52.3 103.2 46 122.8 46 142.8V164c0 23.2 8.4 45.6 23.6 63.1l56 64.2 0 0 83.3 95.6 0 0 91.8 105.3c10 11.5 26.8 14.3 40 6.8l54.5-31.1c17.8-10.2 21.6-34.3 7.7-49.4l-87.7-95.7zM187.2 410.6l-83.3-95.6L9.1 418.5C-4.8 433.6-1 457.7 16.8 467.9l55.1 31.5c13 7.4 29.3 4.9 39.4-6.1l75.9-82.6z"
        /> < title > { title } < / title > < / svg >
    }
}
