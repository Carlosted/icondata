#[cfg(feature = "AiFilledUpCircle")]
use leptos::*;
#[cfg(feature = "AiFilledUpCircle")]
///This icon requires the feature `AiFilledUpCircle` to be enabled.
#[component]
pub fn UpCircle(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm178 555h-46.9c-10.2 0-19.9-4.9-25.9-13.2L512 460.4 406.8 605.8c-6 8.3-15.6 13.2-25.9 13.2H334c-6.5 0-10.3-7.4-6.5-12.7l178-246c3.2-4.4 9.7-4.4 12.9 0l178 246c3.9 5.3.1 12.7-6.4 12.7z"
        /> < title > { title } < / title > < / svg >
    }
}
