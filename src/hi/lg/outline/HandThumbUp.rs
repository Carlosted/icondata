#[cfg(feature = "HiLgOutlineHandThumbUp")]
use leptos::*;
#[cfg(feature = "HiLgOutlineHandThumbUp")]
///This icon requires the feature `HiLgOutlineHandThumbUp` to be enabled.
#[component]
pub fn HandThumbUp(
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
        "M6.63257 10.5C7.43892 10.5 8.16648 10.0542 8.6641 9.41967C9.43726 8.43384 10.4117 7.6134 11.5255 7.02021C12.2477 6.63563 12.8743 6.06428 13.1781 5.30464C13.3908 4.7731 13.5 4.20587 13.5 3.63338V3C13.5 2.58579 13.8358 2.25 14.25 2.25C15.4926 2.25 16.5 3.25736 16.5 4.5C16.5 5.65163 16.2404 6.74263 15.7766 7.71771C15.511 8.27604 15.8836 9 16.5019 9H19.6277C20.6544 9 21.5733 9.69399 21.682 10.7149C21.7269 11.1371 21.75 11.5658 21.75 12C21.75 14.8476 20.7581 17.4636 19.101 19.5212C18.7134 20.0025 18.1142 20.25 17.4962 20.25H13.4802C12.9966 20.25 12.5161 20.172 12.0572 20.0191L8.94278 18.9809C8.48393 18.828 8.00342 18.75 7.51975 18.75H5.90421M14.25 9H16.5M5.90421 18.75C5.98702 18.9546 6.07713 19.1554 6.17423 19.3522C6.37137 19.7517 6.0962 20.25 5.65067 20.25H4.74289C3.85418 20.25 3.02991 19.732 2.77056 18.882C2.43208 17.7726 2.25 16.5951 2.25 15.375C2.25 13.8225 2.54481 12.3389 3.08149 10.9771C3.38655 10.203 4.16733 9.75 4.99936 9.75H6.05212C6.52404 9.75 6.7973 10.3059 6.5523 10.7093C5.72588 12.0698 5.25 13.6668 5.25 15.375C5.25 16.5685 5.48232 17.7078 5.90421 18.75Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
