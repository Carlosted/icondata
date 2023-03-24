#[cfg(feature = "SiCodeigniter")]
use leptos::*;
#[cfg(feature = "SiCodeigniter")]
///This icon requires the feature `SiCodeigniter` to be enabled.
#[component]
pub fn Codeigniter(
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
        "M11.466 0c.88 1.423-.28 3.306-1.207 4.358-.899 1.02-1.992 1.873-2.985 2.8-1.066.996-2.091 2.044-2.967 3.213-1.753 2.339-2.827 5.28-2.038 8.199.788 2.916 3.314 4.772 6.167 5.429-1.44-.622-2.786-2.203-2.79-3.82-.003-1.765 1.115-3.262 2.505-4.246-.167.632-.258 1.21.155 1.774a1.68 1.68 0 0 0 1.696.642c1.487-.326 1.556-1.96.674-2.914-.872-.943-1.715-2.009-1.384-3.377.167-.685.588-1.328 1.121-1.787-.41 1.078.755 2.14 1.523 2.67 1.332.918 2.793 1.612 4.017 2.688 1.288 1.132 2.24 2.661 2.047 4.435-.208 1.923-1.736 3.26-3.45 3.936 3.622-.8 7.365-3.61 7.44-7.627.093-3.032-1.903-5.717-5.158-7.384.19.48.074.697-.058.924-.55.944-2.082 1.152-2.835.184-1.205-1.548.025-3.216.197-4.855.215-2.055-1.073-4.049-2.67-5.242z"
        /> < title > { title } < / title > < / svg >
    }
}
