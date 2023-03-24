#[cfg(feature = "CgTapSingle")]
use leptos::*;
#[cfg(feature = "CgTapSingle")]
///This icon requires the feature `CgTapSingle` to be enabled.
#[component]
pub fn TapSingle(
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
        "M12.0491 3.11401C14.1927 3.11401 16.1393 3.95706 17.5756 5.32967L16.1609 6.74437C15.087 5.73346 13.6404 5.11401 12.0491 5.11401C10.4086 5.11401 8.92183 5.77243 7.83868 6.83944L6.42444 5.42519C7.86954 3.99627 9.85631 3.11401 12.0491 3.11401Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.9767 11.886C10.9767 11.3337 11.4244 10.886 11.9767 10.886C12.529 10.886 12.9767 11.3337 12.9767 11.886V13.886H10.9767V11.886Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M11.9767 6.88599C9.21526 6.88599 6.97668 9.12456 6.97668 11.886V15.886C6.97668 18.6474 9.21526 20.886 11.9767 20.886C14.7381 20.886 16.9767 18.6474 16.9767 15.886V11.886C16.9767 9.12456 14.7381 6.88599 11.9767 6.88599ZM14.9767 15.886V11.886C14.9767 10.2291 13.6335 8.88599 11.9767 8.88599C10.3198 8.88599 8.97668 10.2291 8.97668 11.886V15.886C8.97668 17.5428 10.3198 18.886 11.9767 18.886C13.6335 18.886 14.9767 17.5428 14.9767 15.886Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
