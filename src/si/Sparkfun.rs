#[cfg(feature = "SiSparkfun")]
use leptos::*;
#[cfg(feature = "SiSparkfun")]
///This icon requires the feature `SiSparkfun` to be enabled.
#[component]
pub fn Sparkfun(
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
        "M16.307 5.476c-.756.134-1.975-.615-2.59-1.362-.755-.922-.66-1.647-.071-2.29.883-.978 2.396-.6 2.396-.6s-2.772-2.432-5.658-.44c-2.571 1.77-1.833 4.183.487 6.288 2.09 1.902.42 3.988-1.686 3.717-1.443-.184-2.034-1.343-1.687-2.054.298-.608 1.335-.982 1.335-.982s-1.19-.484-2.592.044c-1.259.474-2.297 1.515-2.214 4.12V24s1.301-1.604 2.83-3.236c1.714-1.84 2.495-3.084 4.254-2.938 3.328.205 5.735-1.273 7.371-3.645 3.141-4.563.67-9.68-1.43-10.343 0 0 .34 1.438-.745 1.638z"
        /> < title > { title } < / title > < / svg >
    }
}
