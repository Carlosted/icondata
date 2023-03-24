#[cfg(feature = "SiKatana")]
use leptos::*;
#[cfg(feature = "SiKatana")]
///This icon requires the feature `SiKatana` to be enabled.
#[component]
pub fn Katana(
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
        "M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm.016 22.762H12c-5.95-.009-10.765-4.84-10.756-10.789.009-5.95 4.839-10.766 10.789-10.757 5.943.009 10.756 4.829 10.756 10.773 0 5.95-4.823 10.773-10.773 10.773zm9.475-10.857a5.562 5.562 0 0 1-9.142 3.214 6.331 6.331 0 0 0 3.251-2.062l.104.169c.339.584.568 1.226.676 1.893a6.281 6.281 0 0 0-.349-2.656 6.328 6.328 0 0 0-8.94-8.63 5.563 5.563 0 0 1 7.418 6.256 6.334 6.334 0 0 0-3.425-1.762l.093-.175a5.53 5.53 0 0 1 1.304-1.533 6.31 6.31 0 0 0-2.122 1.636 6.327 6.327 0 0 0-3.016 12.044 5.564 5.564 0 0 1 1.713-9.562 6.33 6.33 0 0 0 .185 3.818h-.186a5.535 5.535 0 0 1-1.98-.36 6.295 6.295 0 0 0 2.471 1.025 6.328 6.328 0 0 0 8.513 2.758 6.319 6.319 0 0 0 3.432-6.073zm-11.018-1.443a5.582 5.582 0 0 1 3.6.998 5.584 5.584 0 0 1-2.667 2.618 5.57 5.57 0 0 1-.933-3.616z"
        /> < title > { title } < / title > < / svg >
    }
}
