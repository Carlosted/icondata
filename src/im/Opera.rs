#[cfg(feature = "ImOpera")]
use leptos::*;
#[cfg(feature = "ImOpera")]
///This icon requires the feature `ImOpera` to be enabled.
#[component]
pub fn Opera(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M16 8v0 0c0 2.369-1.031 4.5-2.669 5.963-2.053 1-3.966 0.3-4.597-0.137 2.016-0.441 3.537-2.878 3.537-5.825s-1.522-5.384-3.537-5.828c0.634-0.438 2.547-1.137 4.597-0.138 1.637 1.466 2.669 3.597 2.669 5.966v0 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M5.366 3.491c-0.884 1.044-1.456 2.587-1.494 4.322 0 0.003 0 0.372 0 0.378 0.038 1.731 0.613 3.275 1.497 4.319 1.147 1.491 2.853 2.434 4.759 2.434 1.172 0 2.269-0.356 3.206-0.978-1.419 1.266-3.287 2.034-5.334 2.034-0.128 0-0.256-0.003-0.381-0.009-4.241-0.2-7.619-3.7-7.619-7.991 0-4.419 3.581-8 8-8 0.009 0 0.019 0 0.031 0 2.037 0.006 3.894 0.775 5.303 2.038-0.938-0.622-2.034-0.981-3.206-0.981-1.906 0-3.612 0.944-4.763 2.434z"
        /> < title > { title } < / title > < / svg >
    }
}
