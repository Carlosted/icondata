#[cfg(feature = "FaSolidPersonWalkingWithCane")]
use leptos::*;
#[cfg(feature = "FaSolidPersonWalkingWithCane")]
///This icon requires the feature `FaSolidPersonWalkingWithCane` to be enabled.
#[component]
pub fn PersonWalkingWithCane(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M144 96a48 48 0 1 0 0-96 48 48 0 1 0 0 96zm-8.4 32c-36.4 0-69.6 20.5-85.9 53.1L3.4 273.7c-7.9 15.8-1.5 35 14.3 42.9s35 1.5 42.9-14.3L96 231.6v43.2c0 17 6.7 33.3 18.7 45.3L192 397.3V480c0 17.7 14.3 32 32 32s32-14.3 32-32V390.6c0-12.7-5.1-24.9-14.1-33.9L192 306.7V213.3l70.4 93.9c10.6 14.1 30.7 17 44.8 6.4s17-30.7 6.4-44.8L236.8 166.4C218.7 142.2 190.2 128 160 128H135.6zM96.3 346.8L65 472.2c-4.3 17.1 6.1 34.5 23.3 38.8s34.5-6.1 38.8-23.3l22-88.2L96.3 346.8zM418.8 505.1c5 7.3 15 9.1 22.3 4s9.1-15 4-22.3L326.9 316.1c-2.8 3.8-6.1 7.3-10.1 10.3c-5 3.8-10.5 6.4-16.2 7.9L418.8 505.1z"
        /> < title > { title } < / title > < / svg >
    }
}
