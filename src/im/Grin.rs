#[cfg(feature = "ImGrin")]
use leptos::*;
#[cfg(feature = "ImGrin")]
///This icon requires the feature `ImGrin` to be enabled.
#[component]
pub fn Grin(
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
        "M8 16c4.418 0 8-3.582 8-8s-3.582-8-8-8-8 3.582-8 8 3.582 8 8 8zM8 1.5c3.59 0 6.5 2.91 6.5 6.5s-2.91 6.5-6.5 6.5-6.5-2.91-6.5-6.5 2.91-6.5 6.5-6.5zM3 8v1c0 2.2 1.8 4 4 4h2c2.2 0 4-1.8 4-4v-1h-10zM6 11.828c-0.415-0.148-0.796-0.388-1.118-0.71-0.569-0.569-0.882-1.321-0.882-2.118h2v2.828zM9 12h-2v-3h2v3zM11.118 11.118c-0.322 0.322-0.703 0.562-1.118 0.71v-2.828h2c0 0.797-0.313 1.549-0.882 2.118zM3.521 6c0 0 0 0 0 0 0.153 0 0.283-0.11 0.308-0.261 0.096-0.573 0.589-0.989 1.171-0.989s1.074 0.416 1.171 0.989c0.025 0.151 0.156 0.261 0.308 0.261s0.283-0.11 0.308-0.261c0.017-0.101 0.025-0.202 0.025-0.302 0-0.999-0.813-1.813-1.813-1.813s-1.813 0.813-1.813 1.813c0 0.1 0.009 0.201 0.025 0.302 0.025 0.151 0.156 0.261 0.308 0.261zM9.521 6c0 0 0 0 0 0 0.153 0 0.283-0.11 0.308-0.261 0.096-0.573 0.589-0.989 1.171-0.989s1.074 0.416 1.171 0.989c0.025 0.151 0.156 0.261 0.308 0.261s0.283-0.11 0.308-0.261c0.017-0.101 0.025-0.202 0.025-0.302 0-0.999-0.813-1.813-1.813-1.813s-1.813 0.813-1.813 1.813c0 0.1 0.008 0.201 0.025 0.302 0.025 0.151 0.156 0.261 0.308 0.261z"
        /> < title > { title } < / title > < / svg >
    }
}
