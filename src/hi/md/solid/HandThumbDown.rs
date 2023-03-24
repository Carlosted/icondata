#[cfg(feature = "HiMdSolidHandThumbDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidHandThumbDown")]
///This icon requires the feature `HiMdSolidHandThumbDown` to be enabled.
#[component]
pub fn HandThumbDown(
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
        "M18.9053 12.75C18.9053 13.4404 18.3456 14 17.6553 14C16.9649 14 16.4053 13.4404 16.4053 12.75L16.4053 5.25C16.4053 4.55964 16.9649 4 17.6553 4C18.3456 4 18.9053 4.55964 18.9053 5.25L18.9053 12.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.90527 17L8.90527 18.2998C8.90527 18.5676 8.76554 18.8259 8.51033 18.9068C8.31945 18.9674 8.11618 19 7.90527 19C6.8007 19 5.90527 18.1046 5.90527 17C5.90527 16.005 6.08694 15.0524 6.4189 14.1735C6.62302 13.6332 6.25287 13 5.67524 13L3.15527 13C1.91263 13 0.894185 11.9904 1.00907 10.7531C1.20216 8.67339 1.66061 6.67064 2.35024 4.77906C2.75183 3.67752 3.83297 3 5.00543 3L8.19707 3C8.66281 3 9.12214 3.10843 9.53871 3.31672L12.2718 4.68328C12.6884 4.89156 13.1477 5 13.6135 5L14.9053 5L14.9053 12L13.9421 12C13.2576 12 12.6842 12.4825 12.3299 13.0681C11.8414 13.8752 11.0756 14.4958 10.1647 14.7977C9.73188 14.9412 9.31105 15.184 9.15279 15.6116C8.99272 16.0441 8.90527 16.5119 8.90527 17Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
