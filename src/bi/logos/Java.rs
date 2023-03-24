#[cfg(feature = "BiLogosJava")]
use leptos::*;
#[cfg(feature = "BiLogosJava")]
///This icon requires the feature `BiLogosJava` to be enabled.
#[component]
pub fn Java(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9.37 17.51c-3.09.86 1.87 2.64 5.8 1a7.18 7.18 0 0 1-1.1-.54 11.59 11.59 0 0 1-4.16.18c-1.31-.15-.54-.64-.54-.64zm5.33-1.68a16.27 16.27 0 0 1-5.36.26c-1.31-.14-.45-.77-.45-.77-3.4 1.12 1.89 2.4 6.64 1a2.25 2.25 0 0 1-.83-.49zm1.82-9.73s-6.87 1.71-3.59 5.49a1.47 1.47 0 0 1-.25 2.12s2.46-1.27 1.33-2.86-1.86-2.22 2.51-4.75z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.13 19.14s.56.46-.63.82c-2.27.69-9.43.9-11.42 0-.72-.31.62-.74 1-.83a2.54 2.54 0 0 1 .69-.08c-.8-.56-5.15 1.1-2.21 1.58 8.05 1.37 14.64-.56 12.57-1.49zm-2.76-4.88a7 7 0 0 1 .92-.49s-1.51.27-3 .4a28.11 28.11 0 0 1-4.82.05C6.09 13.91 9.74 13 9.74 13a6.81 6.81 0 0 0-3.16.75c-2.06 1.03 5.09 1.48 8.79.51zm.91 2.44a.39.39 0 0 1-.08.09c5-1.32 3.18-4.65.78-3.81a.71.71 0 0 0-.33.25s.14-.05.43-.12c1.22-.25 2.92 1.63-.8 3.59zM12 13c-.74-1.67-3.26-3.14 0-5.71C16.09 4.08 14 2 14 2c.84 3.31-3 4.31-4.34 6.37-.93 1.41.47 2.92 2.34 4.63z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.41 21.32a29.75 29.75 0 0 1-9.14.14s.46.38 2.84.53c3.61.23 9.15-.12 9.29-1.83 0 0-.26.64-2.99 1.16z"
        /> < title > { title } < / title > < / svg >
    }
}
