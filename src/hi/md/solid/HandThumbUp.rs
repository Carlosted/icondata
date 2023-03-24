#[cfg(feature = "HiMdSolidHandThumbUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidHandThumbUp")]
///This icon requires the feature `HiMdSolidHandThumbUp` to be enabled.
#[component]
pub fn HandThumbUp(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M1 8.25C1 7.55964 1.55964 7 2.25 7C2.94036 7 3.5 7.55964 3.5 8.25V15.75C3.5 16.4404 2.94036 17 2.25 17C1.55964 17 1 16.4404 1 15.75V8.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 3V1.70016C11 1.43243 11.1397 1.17406 11.3949 1.09316C11.5858 1.03264 11.7891 1 12 1C13.1046 1 14 1.89543 14 3C14 3.99504 13.8183 4.94764 13.4864 5.82646C13.2823 6.36683 13.6524 7 14.23 7H16.75C17.9926 7 19.0111 8.00957 18.8962 9.24689C18.7031 11.3266 18.2447 13.3294 17.555 15.2209C17.1534 16.3225 16.0723 17 14.8998 17H11.7082C11.2425 17 10.7831 16.8916 10.3666 16.6833L7.63344 15.3167C7.21687 15.1084 6.75753 15 6.2918 15H5V8H5.9632C6.64763 8 7.22105 7.51748 7.57541 6.93193C8.06387 6.1248 8.82962 5.50423 9.74061 5.20229C10.1734 5.05884 10.5942 4.81595 10.7525 4.38836C10.9126 3.95587 11 3.48815 11 3Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
