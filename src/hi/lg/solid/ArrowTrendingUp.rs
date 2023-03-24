#[cfg(feature = "HiLgSolidArrowTrendingUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowTrendingUp")]
///This icon requires the feature `HiLgSolidArrowTrendingUp` to be enabled.
#[component]
pub fn ArrowTrendingUp(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.2194 6.26756C15.3679 5.88086 15.8017 5.68771 16.1884 5.83615L22.1297 8.11679C22.3154 8.18807 22.4651 8.33021 22.546 8.51192C22.627 8.69364 22.6324 8.90005 22.5611 9.08575L20.2804 15.027C20.132 15.4137 19.6982 15.6069 19.3115 15.4584C18.9248 15.31 18.7316 14.8762 18.8801 14.4895L20.5118 10.2386L19.4253 10.7223C16.9721 11.8146 15.1036 13.6754 13.975 15.8959C13.8662 16.11 13.6614 16.2591 13.4241 16.2968C13.1869 16.3345 12.946 16.2563 12.7761 16.0864L9 12.3103L2.78033 18.53C2.48744 18.8229 2.01256 18.8229 1.71967 18.53C1.42678 18.2371 1.42678 17.7622 1.71967 17.4693L8.46967 10.7193C8.61032 10.5787 8.80109 10.4996 9 10.4996C9.19891 10.4996 9.38968 10.5787 9.53033 10.7193L13.1363 14.3253C14.4369 12.2042 16.3711 10.4402 18.8152 9.35203L19.9017 8.86828L15.6508 7.23652C15.2641 7.08808 15.071 6.65426 15.2194 6.26756Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
