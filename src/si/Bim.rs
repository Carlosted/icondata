#[cfg(feature = "SiBim")]
use leptos::*;
#[cfg(feature = "SiBim")]
///This icon requires the feature `SiBim` to be enabled.
#[component]
pub fn Bim(
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
        "M3.327 6.3015c-1.8378 0-3.3266 1.4908-3.3266 3.3283v4.7424c0 1.8375 1.4888 3.3263 3.3265 3.3263h17.347c1.8376 0 3.3265-1.4888 3.3265-3.3263V9.6298c0-1.8375-1.4889-3.3283-3.3265-3.3283H12.353L11.06 8.1922 9.7863 6.3015Zm1.5742 2.1896c.8137-.0085 1.57.0699 2.01.2422.7978.3017 1.254.96 1.293 1.8067.0294.612-.2962 1.1623-.791 1.5801.5608.3311.9783.8269.9649 1.5392-.0257 1.2596-1.2067 2.0391-3.3362 1.9903-1.1473-.0269-1.7047-.0285-2.3694-.1739V8.6395c1.1595-.1564 1.288-.14 2.2287-.1484Zm9.5223.1113h1.9903l1.2833 3.2247 1.2735-3.2247h2.0511l.5507 6.8675h-1.9707l-.1446-3.9123-1.7716 3.8986-1.6466-3.885-.3418 3.8987h-2.0158Zm-4.4732.0234h2.0901v6.8675h-2.09Zm-5.2347 1.4298v1.4532h.8086a.7257.7257 0 0 0 .7266-.7266c0-.4008-.2965-.7266-.7266-.7266zm0 2.7872v1.1973h.7793c.4105.0232.9576-.1498.963-.586-.0245-.4923-.5244-.637-.9337-.6113z"
        /> < title > { title } < / title > < / svg >
    }
}
