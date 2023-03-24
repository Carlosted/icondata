#[cfg(feature = "SiXaml")]
use leptos::*;
#[cfg(feature = "SiXaml")]
///This icon requires the feature `SiXaml` to be enabled.
#[component]
pub fn Xaml(
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
        "M6.3912 1.5373a.7847.7847 0 0 0-.679.3914l-5.6065 9.678a.7847.7847 0 0 0 0 .7867l5.6065 9.6779a.7847.7847 0 0 0 .679.3914h11.2176c.28 0 .5387-.1492.679-.3914l5.6065-9.678a.7847.7847 0 0 0 0-.7867l-5.6065-9.6779a.7848.7848 0 0 0-.679-.3914zm.0302.837h10.542l-5.2093 9.2112H5.9679l4.7196-8.1747a.1308.1308 0 0 0-.1132-.1962L7.5062 3.213a.2615.2615 0 0 0-.2266.1307l-4.7633 8.2419h-1.431zm11.3325.3025L23.1549 12l-5.3722 9.2735-5.2987-9.2784zm.1712 2.8248a.1295.1295 0 0 0-.1132.0665l-1.5186 2.689a.2616.2616 0 0 0 .0013.2595l1.943 3.3611a.2615.2615 0 0 1 0 .2617l-1.9268 3.3375a.2616.2616 0 0 0-.0006.2605l1.5272 2.6742c.05.0877.1763.088.2268.0006l3.5503-6.1431a.5231.5231 0 0 0-.0001-.5238L18.039 5.567a.1295.1295 0 0 0-.1139-.0653zM1.09 12.4225h1.4363l4.7634 8.2314a.2616.2616 0 0 0 .2262.1305l3.0718.0018a.1308.1308 0 0 0 .1133-.1962l-4.7215-8.1675h5.7848l5.2557 9.2032H6.4214z"
        /> < title > { title } < / title > < / svg >
    }
}
