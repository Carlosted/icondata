#[cfg(feature = "BiRegularStation")]
use leptos::*;
#[cfg(feature = "BiRegularStation")]
///This icon requires the feature `BiRegularStation` to be enabled.
#[component]
pub fn Station(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "2" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m7.758 16.243 1.414-1.415a3.97 3.97 0 0 1-1.173-2.831c0-1.068.417-2.071 1.173-2.825L7.758 7.756a5.957 5.957 0 0 0-1.76 4.24c0 1.604.625 3.112 1.76 4.247zm8.484 0A5.96 5.96 0 0 0 18 12a5.96 5.96 0 0 0-1.758-4.243l-1.414 1.414C15.584 9.927 16 10.932 16 12s-.416 2.073-1.172 2.829l1.414 1.414z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.344 17.657a7.953 7.953 0 0 1-2.345-5.659c0-2.137.833-4.145 2.345-5.654L4.93 4.929c-1.89 1.886-2.931 4.397-2.931 7.069s1.041 5.183 2.931 7.073l1.414-1.414zM17.657 6.343A7.948 7.948 0 0 1 20 12a7.948 7.948 0 0 1-2.343 5.657l1.414 1.414A9.932 9.932 0 0 0 22 12a9.934 9.934 0 0 0-2.929-7.071l-1.414 1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
