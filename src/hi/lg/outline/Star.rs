#[cfg(feature = "HiLgOutlineStar")]
use leptos::*;
#[cfg(feature = "HiLgOutlineStar")]
///This icon requires the feature `HiLgOutlineStar` to be enabled.
#[component]
pub fn Star(
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
        "M11.4806 3.49883C11.6728 3.03685 12.3272 3.03685 12.5193 3.49883L14.6453 8.61028C14.7263 8.80504 14.9095 8.93811 15.1197 8.95497L20.638 9.39736C21.1367 9.43735 21.339 10.0598 20.959 10.3853L16.7546 13.9867C16.5945 14.1239 16.5245 14.3392 16.5734 14.5444L17.8579 19.9293C17.974 20.416 17.4446 20.8007 17.0176 20.5398L12.2932 17.6542C12.1132 17.5443 11.8868 17.5443 11.7068 17.6542L6.98238 20.5398C6.55539 20.8007 6.02594 20.416 6.14203 19.9293L7.42652 14.5444C7.47546 14.3392 7.4055 14.1239 7.24531 13.9867L3.04099 10.3853C2.661 10.0598 2.86323 9.43735 3.36197 9.39736L8.88022 8.95497C9.09048 8.93811 9.27363 8.80504 9.35464 8.61028L11.4806 3.49883Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
