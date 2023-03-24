#[cfg(feature = "VsIssueDraft")]
use leptos::*;
#[cfg(feature = "VsIssueDraft")]
///This icon requires the feature `VsIssueDraft` to be enabled.
#[component]
pub fn IssueDraft(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.7324 9.20047L13.6835 9.50931C13.889 8.87656 14 8.20125 14 7.5C14 6.79875 13.889 6.12344 13.6835 5.49069L12.7324 5.79953C12.9058 6.33376 13 6.9049 13 7.5C13 8.0951 12.9058 8.66624 12.7324 9.20047ZM12.4021 5.00313L13.2928 4.54842C12.6696 3.3279 11.6721 2.33037 10.4516 1.70723L9.99687 2.59787C11.0298 3.12523 11.8748 3.9702 12.4021 5.00313ZM9.20047 2.26763L9.50931 1.31652C8.87656 1.11105 8.20125 1 7.5 1C6.79875 1 6.12344 1.11105 5.49069 1.31652L5.79953 2.26763C6.33376 2.09415 6.9049 2 7.5 2C8.0951 2 8.66624 2.09415 9.20047 2.26763ZM5.00313 2.59787L4.54842 1.70723C3.3279 2.33037 2.33037 3.3279 1.70723 4.54842L2.59787 5.00313C3.12523 3.9702 3.9702 3.12523 5.00313 2.59787ZM1 7.5C1 6.79875 1.11105 6.12344 1.31652 5.49069L2.26763 5.79953C2.09415 6.33376 2 6.9049 2 7.5C2 8.0951 2.09415 8.66624 2.26763 9.20047L1.31652 9.50931C1.11105 8.87656 1 8.20125 1 7.5ZM2.59787 9.99687L1.70723 10.4516C2.33037 11.6721 3.3279 12.6696 4.54842 13.2928L5.00313 12.4021C3.9702 11.8748 3.12523 11.0298 2.59787 9.99687ZM5.79953 12.7324L5.49069 13.6835C6.12344 13.889 6.79875 14 7.5 14C8.20125 14 8.87656 13.889 9.50931 13.6835L9.20047 12.7324C8.66624 12.9058 8.0951 13 7.5 13C6.9049 13 6.33376 12.9058 5.79953 12.7324ZM9.99687 12.4021L10.4516 13.2928C11.6721 12.6696 12.6696 11.6721 13.2928 10.4516L12.4021 9.99687C11.8748 11.0298 11.0298 11.8748 9.99687 12.4021ZM7.50002 8.5C8.0523 8.5 8.50002 8.05228 8.50002 7.5C8.50002 6.94772 8.0523 6.5 7.50002 6.5C6.94773 6.5 6.50002 6.94772 6.50002 7.5C6.50002 8.05228 6.94773 8.5 7.50002 8.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
