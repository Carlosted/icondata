#[cfg(feature = "ImStrikethrough")]
use leptos::*;
#[cfg(feature = "ImStrikethrough")]
///This icon requires the feature `ImStrikethrough` to be enabled.
#[component]
pub fn Strikethrough(
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
        "M16 8v1h-3.664c0.43 0.602 0.664 1.292 0.664 2 0 1.107-0.573 2.172-1.572 2.921-0.927 0.696-2.145 1.079-3.428 1.079s-2.501-0.383-3.428-1.079c-0.999-0.749-1.572-1.814-1.572-2.921h2c0 1.084 1.374 2 3 2s3-0.916 3-2c0-1.084-1.374-2-3-2h-8v-1h4.68c-0.037-0.026-0.073-0.052-0.108-0.079-0.999-0.749-1.572-1.814-1.572-2.921s0.573-2.172 1.572-2.921c0.927-0.696 2.145-1.079 3.428-1.079s2.501 0.383 3.428 1.079c0.999 0.749 1.572 1.814 1.572 2.921h-2c0-1.084-1.374-2-3-2s-3 0.916-3 2c0 1.084 1.374 2 3 2 1.234 0 2.407 0.354 3.32 1h4.68z"
        /> < title > { title } < / title > < / svg >
    }
}
