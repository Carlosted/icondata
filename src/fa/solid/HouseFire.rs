#[cfg(feature = "FaSolidHouseFire")]
use leptos::*;
#[cfg(feature = "FaSolidHouseFire")]
///This icon requires the feature `FaSolidHouseFire` to be enabled.
#[component]
pub fn HouseFire(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M288 350.1l0 1.9H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L447.3 128.1c-12.3-1-25 3-34.8 11.7c-35.4 31.6-65.6 67.7-87.3 102.8C304.3 276.5 288 314.9 288 350.1zM453.5 163.8c19.7 17.8 38.2 37 55.5 57.7c7.9-9.9 16.8-20.7 26.5-29.5c5.6-5.1 14.4-5.1 20 0c24.7 22.7 45.6 52.7 60.4 81.1c14.5 28 24.2 58.8 24.2 79C640 440 568.7 512 480 512c-89.7 0-160-72.1-160-159.8c0-26.4 12.7-60.7 32.4-92.6c20-32.4 48.1-66.1 81.4-95.8c2.8-2.5 6.4-3.8 10-3.7c3.5 0 7 1.3 9.8 3.8zM530 433c30-21 38-63 20-96c-2-4-4-8-7-12l-36 42s-58-74-62-79c-30 37-45 58-45 82c0 49 36 78 81 78c18 0 34-5 49-15z"
        /> < title > { title } < / title > < / svg >
    }
}
