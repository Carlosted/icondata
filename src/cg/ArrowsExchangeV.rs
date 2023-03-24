#[cfg(feature = "CgArrowsExchangeV")]
use leptos::*;
#[cfg(feature = "CgArrowsExchangeV")]
///This icon requires the feature `CgArrowsExchangeV` to be enabled.
#[component]
pub fn ArrowsExchangeV(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.9842 15C12.9842 15.5523 13.4319 16 13.9842 16C14.3416 16 14.6552 15.8125 14.832 15.5305L17.5196 12.8429C17.9101 12.4524 17.9101 11.8192 17.5196 11.4287C17.1291 11.0382 16.4959 11.0382 16.1054 11.4287L14.9842 12.5499L14.9842 5C14.9842 4.44771 14.5364 4 13.9842 4C13.4319 4 12.9842 4.44772 12.9842 5L12.9842 14.9506C12.984 14.9597 12.984 14.9688 12.9842 14.9779V15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.016 9C11.016 8.44771 10.5682 8 10.016 8C9.65856 8 9.34496 8.18748 9.16813 8.46947L6.48052 11.1571C6.08999 11.5476 6.08999 12.1808 6.48051 12.5713C6.87104 12.9618 7.5042 12.9618 7.89473 12.5713L9.01596 11.4501L9.01596 19C9.01596 19.5523 9.46367 20 10.016 20C10.5682 20 11.016 19.5523 11.016 19L11.016 9.04945C11.0161 9.04033 11.0161 9.03121 11.016 9.02208V9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
