#[cfg(feature = "OcLgCloudOffline")]
use leptos::*;
#[cfg(feature = "OcLgCloudOffline")]
///This icon requires the feature `OcLgCloudOffline` to be enabled.
#[component]
pub fn CloudOffline(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m2.78 2.22 19.5 19.5a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-2.845-2.845a6.932 6.932 0 0 1-.944.065H5.017C2.229 20 0 17.831 0 15.059a4.899 4.899 0 0 1 3.111-4.58A7.52 7.52 0 0 1 4.36 5.922L1.72 3.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018ZM16.94 18.5 5.448 7.01a6.026 6.026 0 0 0-.794 3.853.75.75 0 0 1-.552.869c-1.52.385-2.603 1.712-2.603 3.328 0 1.917 1.532 3.44 3.517 3.44Zm-6.104-16a7.865 7.865 0 0 0-3.638.88.75.75 0 1 0 .692 1.331A6.365 6.365 0 0 1 10.836 4c2.588 0 4.77 1.5 5.72 3.655l.179.445a.75.75 0 0 0 .696.471c2.843 0 5.069 2.206 5.069 4.965a4.9 4.9 0 0 1-1.684 3.716.75.75 0 0 0 .986 1.13A6.396 6.396 0 0 0 24 13.536c0-3.44-2.652-6.191-6.054-6.445l-.002-.006a.634.634 0 0 0-.01-.022C16.749 4.358 14.026 2.5 10.837 2.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
