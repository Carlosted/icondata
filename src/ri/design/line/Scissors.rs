#[cfg(feature = "RiDesignLineScissors")]
use leptos::*;
#[cfg(feature = "RiDesignLineScissors")]
///This icon requires the feature `RiDesignLineScissors` to be enabled.
#[component]
pub fn Scissors(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M9.446 8.032L12 10.586l6.728-6.728a2 2 0 0 1 2.828 0l-12.11 12.11a4 4 0 1 1-1.414-1.414L10.586 12 8.032 9.446a4 4 0 1 1 1.414-1.414zm5.38 5.38l6.73 6.73a2 2 0 0 1-2.828 0l-5.317-5.316 1.415-1.415zm-7.412 3.174a2 2 0 1 0-2.828 2.828 2 2 0 0 0 2.828-2.828zm0-9.172a2 2 0 1 0-2.828-2.828 2 2 0 0 0 2.828 2.828z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
