#[cfg(feature = "FaSolidHollyBerry")]
use leptos::*;
#[cfg(feature = "FaSolidHollyBerry")]
///This icon requires the feature `FaSolidHollyBerry` to be enabled.
#[component]
pub fn HollyBerry(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M247 96a48 48 0 1 0 0-96 48 48 0 1 0 0 96zm-80 96a48 48 0 1 0 0-96 48 48 0 1 0 0 96zM267.8 383.8c1 .1 2.1 .2 3.2 .2c39.8 0 72 32.2 72 72v22.7c0 16.4 16 27.9 31.6 22.8l12.8-4.3c18-6 37.3-6.5 55.6-1.5l19.4 5.3c17.9 4.9 34.4-11.6 29.5-29.5L486.6 452c-5-18.3-4.4-37.6 1.5-55.6l4.3-12.8c5.2-15.5-6.4-31.6-22.8-31.6c-34.6 0-62.7-28.1-62.7-62.7v-32c0-16.4-16-27.9-31.6-22.8l-12.8 4.3c-18 6-37.3 6.5-55.6 1.5l-29.6-8.1c-2.9-.8-5.9-1-8.7-.7c4.2 9.7 5.8 20.8 3.7 32.3L266 298.7c-1.5 8.4-1.4 17 .5 25.3l5.3 23.9c2.8 12.7 1.1 25.2-4 35.9zM118.6 234.5c-15.5-5.2-31.6 6.4-31.6 22.8v32C87 323.9 58.9 352 24.3 352C7.9 352-3.6 368 1.5 383.6l4.3 12.8c6 18 6.5 37.3 1.5 55.6L2.1 471.5c-4.9 17.9 11.6 34.4 29.5 29.5L51 495.6c18.3-5 37.6-4.5 55.6 1.5l12.8 4.3c15.5 5.2 31.6-6.4 31.6-22.8v-32c0-34.6 28.1-62.7 62.7-62.7c16.4 0 27.9-16 22.8-31.6l-4.3-12.8c-6-18-6.5-37.3-1.5-55.6l5.3-19.4c4.9-17.9-11.6-34.4-29.5-29.5L187 240.4c-18.3 5-37.6 4.4-55.6-1.5l-12.8-4.3zM375 144a48 48 0 1 0 -96 0 48 48 0 1 0 96 0z"
        /> < title > { title } < / title > < / svg >
    }
}
