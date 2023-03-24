#[cfg(feature = "TbMarkdownOff")]
use leptos::*;
#[cfg(feature = "TbMarkdownOff")]
///This icon requires the feature `TbMarkdownOff` to be enabled.
#[component]
pub fn MarkdownOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-markdown-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 5h10a2 2 0 0 1 2 2v10" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 19h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 1.85 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 15v-6l2 2l1 -1m1 1v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.5 13.5l.5 -.5m-2 -1v-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
