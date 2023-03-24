#[cfg(feature = "ImBug")]
use leptos::*;
#[cfg(feature = "ImBug")]
///This icon requires the feature `ImBug` to be enabled.
#[component]
pub fn Bug(
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
        "M16 9v-1h-3.020c-0.092-1.136-0.497-2.172-1.12-3.004h2.53l1.095-4.379-0.97-0.243-0.905 3.621h-2.729c-0.014-0.011-0.028-0.021-0.042-0.032 0.105-0.305 0.162-0.632 0.162-0.972 0-1.653-1.343-2.992-3-2.992s-3 1.34-3 2.992c0 0.34 0.057 0.667 0.162 0.972-0.014 0.011-0.028 0.021-0.042 0.032h-2.729l-0.905-3.621-0.97 0.243 1.095 4.379h2.53c-0.623 0.832-1.028 1.868-1.12 3.004h-3.020v1h3.021c0.059 0.713 0.242 1.388 0.526 1.996h-1.937l-1.095 4.379 0.97 0.243 0.905-3.621h1.756c0.917 1.219 2.303 1.996 3.854 1.996s2.937-0.777 3.854-1.996h1.756l0.905 3.621 0.97-0.243-1.095-4.379h-1.937c0.283-0.608 0.466-1.283 0.526-1.996h3.021z"
        /> < title > { title } < / title > < / svg >
    }
}
