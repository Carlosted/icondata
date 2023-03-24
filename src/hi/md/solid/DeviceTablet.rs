#[cfg(feature = "HiMdSolidDeviceTablet")]
use leptos::*;
#[cfg(feature = "HiMdSolidDeviceTablet")]
///This icon requires the feature `HiMdSolidDeviceTablet` to be enabled.
#[component]
pub fn DeviceTablet(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5 1C3.34315 1 2 2.34315 2 4V16C2 17.6569 3.34315 19 5 19H15C16.6569 19 18 17.6569 18 16V4C18 2.34315 16.6569 1 15 1H5ZM3.5 4C3.5 3.17157 4.17157 2.5 5 2.5H15C15.8284 2.5 16.5 3.17157 16.5 4V16C16.5 16.8284 15.8284 17.5 15 17.5H5C4.17157 17.5 3.5 16.8284 3.5 16V4ZM8.75 15.5C8.33579 15.5 8 15.8358 8 16.25C8 16.6642 8.33579 17 8.75 17H11.25C11.6642 17 12 16.6642 12 16.25C12 15.8358 11.6642 15.5 11.25 15.5H8.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
