#[cfg(feature = "SiCilium")]
use leptos::*;
#[cfg(feature = "SiCilium")]
///This icon requires the feature `SiCilium` to be enabled.
#[component]
pub fn Cilium(
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
        "M13.607 14.583h-3.215l-1.626-2.764 1.626-2.802h3.215l1.626 2.802-1.626 2.764ZM14.186 8H9.799l-2.2 3.813 2.2 3.787h4.387l2.213-3.787L14.186 8Zm-4.387 8.4-2.2 3.813L9.799 24h4.387l2.213-3.787-2.213-3.813H9.799Zm-1.034 3.819 1.627-2.802h3.215l1.626 2.802-1.626 2.765h-3.215l-1.627-2.765ZM9.799 0l-2.2 3.813 2.2 3.787h4.387l2.213-3.787L14.186 0H9.799ZM8.765 3.819l1.627-2.802h3.215l1.626 2.802-1.626 2.764h-3.215L8.765 3.819Zm8.234 8.581-2.2 3.813 2.2 3.787h4.388l2.213-3.787-2.213-3.813h-4.388Zm-1.034 3.819 1.627-2.802h3.215l1.626 2.802-1.626 2.765h-3.215l-1.627-2.765ZM16.999 4l-2.2 3.813 2.2 3.787h4.388L23.6 7.813 21.387 4h-4.388Zm-1.034 3.819 1.627-2.802h3.215l1.626 2.802-1.626 2.764h-3.215l-1.627-2.764ZM2.599 12.4l-2.2 3.813L2.599 20h4.387l2.213-3.787L6.986 12.4H2.599Zm-1.034 3.819 1.627-2.802h3.214l1.627 2.802-1.627 2.765H3.192l-1.627-2.765ZM2.599 4l-2.2 3.813 2.2 3.787h4.387l2.213-3.787L6.986 4H2.599ZM1.565 7.819l1.627-2.802h3.214l1.627 2.802-1.627 2.764H3.192L1.565 7.819Z"
        /> < title > { title } < / title > < / svg >
    }
}
