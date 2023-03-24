#[cfg(feature = "BiSolidBasketball")]
use leptos::*;
#[cfg(feature = "BiSolidBasketball")]
///This icon requires the feature `BiSolidBasketball` to be enabled.
#[component]
pub fn Basketball(
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
        "M18.328 4.258a9.953 9.953 0 0 0-5.949-2.235 8.99 8.99 0 0 1-1.835 7.107L12 10.586l6.328-6.328zM7.701 9.115 4.258 5.672a9.938 9.938 0 0 0-2.112 4.704 7.007 7.007 0 0 0 5.555-1.261zm12.041-3.443L13.414 12l1.456 1.456a8.993 8.993 0 0 1 7.107-1.835 9.953 9.953 0 0 0-2.235-5.949zm2.112 7.952a7.007 7.007 0 0 0-5.555 1.261l3.443 3.443a9.924 9.924 0 0 0 2.112-4.704zM9.115 7.701a7.007 7.007 0 0 0 1.261-5.555 9.928 9.928 0 0 0-4.704 2.112l3.443 3.443zm4.509 14.153a9.936 9.936 0 0 0 4.704-2.111L14.885 16.3a7.003 7.003 0 0 0-1.261 5.554zM12 13.414l-6.328 6.328a9.953 9.953 0 0 0 5.949 2.235 8.99 8.99 0 0 1 1.835-7.107L12 13.414zm-7.742 4.914L10.586 12 9.13 10.544a8.993 8.993 0 0 1-7.107 1.835 9.953 9.953 0 0 0 2.235 5.949z"
        /> < title > { title } < / title > < / svg >
    }
}
