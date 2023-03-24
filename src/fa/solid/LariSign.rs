#[cfg(feature = "FaSolidLariSign")]
use leptos::*;
#[cfg(feature = "FaSolidLariSign")]
///This icon requires the feature `FaSolidLariSign` to be enabled.
#[component]
pub fn LariSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M144 32c17.7 0 32 14.3 32 32V96.7c5.3-.4 10.6-.7 16-.7s10.7 .2 16 .7V64c0-17.7 14.3-32 32-32s32 14.3 32 32v49.4c54.9 25.2 95.8 75.5 108.2 136.2c3.5 17.3-7.7 34.2-25 37.7s-34.2-7.7-37.7-25c-6.1-29.9-22.5-55.9-45.4-74.3V256c0 17.7-14.3 32-32 32s-32-14.3-32-32V161c-5.2-.7-10.6-1-16-1s-10.8 .3-16 1v95c0 17.7-14.3 32-32 32s-32-14.3-32-32V188.1C82.7 211.5 64 247.6 64 288c0 70.7 57.3 128 128 128H352c17.7 0 32 14.3 32 32s-14.3 32-32 32H192 32c-17.7 0-32-14.3-32-32s14.3-32 32-32H48.9C18.5 382 0 337.2 0 288c0-77.5 45.9-144.3 112-174.6V64c0-17.7 14.3-32 32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
