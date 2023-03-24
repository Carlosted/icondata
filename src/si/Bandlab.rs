#[cfg(feature = "SiBandlab")]
use leptos::*;
#[cfg(feature = "SiBandlab")]
///This icon requires the feature `SiBandlab` to be enabled.
#[component]
pub fn Bandlab(
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
        "M10.87714 17.58857C12.84 17.58857 15.3 15.78 15.3 13.32c0-.72-.15429-1.43143-.45429-2.09143l-3.41142-8.34H17.16L15.65143 0H7.48286l4.68 11.56286C10.1657 12 8.34 13.56 8.34 15.36c0 1.30286.94286 2.22857 2.53714 2.22857.02737 0 0 0 5.1-11.50286H19.32L22.88571 12.24A8.82857 8.82857 0 0 1 24 16.26c0 4.5-3.54857 7.74-8.57143 7.74H8.57143C3.54 24 0 20.76 0 16.26c.04286-1.41429.42857-2.78571 1.11429-4.02857L4.67143 6.0857h3.34286l-4.17429 7.26a6.05143 6.05143 0 0 0-.82286 2.86286c0 2.86286 1.95429 4.90286 5.65715 4.90286h6.65142c3.68572 0 5.65715-2.04 5.65715-4.90286a6.05143 6.05143 0 0 0-.83143-2.86286l-4.17429-7.26H19.32h-3.34286"
        /> < title > { title } < / title > < / svg >
    }
}
