#[cfg(feature = "HiMdSolidHeart")]
use leptos::*;
#[cfg(feature = "HiMdSolidHeart")]
///This icon requires the feature `HiMdSolidHeart` to be enabled.
#[component]
pub fn Heart(
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
        "M9.65298 16.9149L9.6476 16.9121L9.62912 16.9024C9.61341 16.8941 9.59102 16.8822 9.56238 16.8667C9.50511 16.8358 9.42281 16.7907 9.31906 16.732C9.11164 16.6146 8.81794 16.4425 8.46663 16.2206C7.76556 15.7777 6.82731 15.1314 5.88539 14.3197C4.04447 12.7332 2 10.3523 2 7.5C2 5.01472 4.01472 3 6.5 3C7.9144 3 9.17542 3.65238 10 4.67158C10.8246 3.65238 12.0856 3 13.5 3C15.9853 3 18 5.01472 18 7.5C18 10.3523 15.9555 12.7332 14.1146 14.3197C13.1727 15.1314 12.2344 15.7777 11.5334 16.2206C11.1821 16.4425 10.8884 16.6146 10.6809 16.732C10.5772 16.7907 10.4949 16.8358 10.4376 16.8667C10.409 16.8822 10.3866 16.8941 10.3709 16.9024L10.3524 16.9121L10.347 16.9149L10.3453 16.9158C10.13 17.03 9.87 17.03 9.65529 16.9161L9.65298 16.9149Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
