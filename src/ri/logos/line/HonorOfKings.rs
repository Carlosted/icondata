#[cfg(feature = "RiLogosLineHonorOfKings")]
use leptos::*;
#[cfg(feature = "RiLogosLineHonorOfKings")]
///This icon requires the feature `RiLogosLineHonorOfKings` to be enabled.
#[component]
pub fn HonorOfKings(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M18.328 4.256l-1.423 1.423c-3.138-2.442-7.677-2.22-10.562.664-2.374 2.374-2.944 5.868-1.71 8.78l2.417-2.417c-.213-1.503.258-3.085 1.414-4.242 1.71-1.71 4.352-1.922 6.293-.636l-1.464 1.464c-1.115-.532-2.49-.337-3.414.587-.924.923-1.12 2.299-.587 3.414l-6.45 6.45c-.034-3.5-.591-4.812-.788-6.702-.301-2.894.657-5.894 2.875-8.112 3.666-3.666 9.471-3.89 13.4-.673zm2.83.002c.034 3.5.591 4.811.788 6.701.301 2.894-.657 5.894-2.875 8.112-3.666 3.666-9.471 3.89-13.4.673l1.424-1.423c3.138 2.442 7.677 2.22 10.562-.664 2.374-2.374 2.944-5.868 1.71-8.78l-2.417 2.417c.213 1.503-.258 3.085-1.414 4.242-1.71 1.71-4.352 1.922-6.293.636l1.464-1.464c1.115.532 2.49.337 3.414-.587.924-.923 1.12-2.299.587-3.414l6.45-6.45z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
