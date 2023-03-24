#[cfg(feature = "SiPixabay")]
use leptos::*;
#[cfg(feature = "SiPixabay")]
///This icon requires the feature `SiPixabay` to be enabled.
#[component]
pub fn Pixabay(
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
        "M5.627 10.234a.888.888 0 01-.887-.888H1.7c0 .49-.398.888-.888.888H0v9.447h15.56v-9.447H5.64zm-.886 2.796h-3.04v-1.323h3.04v1.323zm5.344 5.234a3.271 3.271 0 01-3.267-3.269c0-1.802 1.466-3.193 3.267-3.193s3.267 1.39 3.267 3.193a3.271 3.271 0 01-3.267 3.269zm1.756-3.269c0 .969-.788 1.757-1.756 1.757a1.759 1.759 0 01-1.756-1.757c0-.969.788-1.757 1.756-1.757s1.756.788 1.756 1.757zM24 9.501l-3.93 10.156-3.164-1.226V16.7l2.242.869 2.765-7.146L11.55 6.407l-.96 2.482h-1.73l1.769-4.57L24 9.501Z"
        /> < title > { title } < / title > < / svg >
    }
}
