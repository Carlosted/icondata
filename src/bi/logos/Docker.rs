#[cfg(feature = "BiLogosDocker")]
use leptos::*;
#[cfg(feature = "BiLogosDocker")]
///This icon requires the feature `BiLogosDocker` to be enabled.
#[component]
pub fn Docker(
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
        "M20.17 9.82a4.76 4.76 0 0 0-.84.07 3.12 3.12 0 0 0-1.43-2.14l-.28-.16-.19.27a3.7 3.7 0 0 0-.51 1.19 2.84 2.84 0 0 0 .33 2.22 4.11 4.11 0 0 1-1.45.35H2.63a.63.63 0 0 0-.63.62 9.6 9.6 0 0 0 .58 3.39 5 5 0 0 0 2 2.6 8.86 8.86 0 0 0 4.42.95 13.27 13.27 0 0 0 2.42-.18 10.09 10.09 0 0 0 3.19-1.15A8.9 8.9 0 0 0 16.78 16a11.94 11.94 0 0 0 2.13-3.67h.19a3.08 3.08 0 0 0 2.23-.84 2.36 2.36 0 0 0 .59-.87l.08-.22-.2-.16a2.69 2.69 0 0 0-1.63-.42z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.61 9.35H3.85a.16.16 0 0 0-.16.15v1.58a.16.16 0 0 0 .16.15h1.76a.16.16 0 0 0 .16-.15V9.5a.16.16 0 0 0-.16-.15zm2.44 0H6.28a.16.16 0 0 0-.16.15v1.58a.16.16 0 0 0 .16.15h1.77a.15.15 0 0 0 .15-.15V9.5a.15.15 0 0 0-.15-.15zm2.47 0H8.75a.15.15 0 0 0-.15.15v1.58a.15.15 0 0 0 .15.15h1.77a.15.15 0 0 0 .15-.15V9.5a.15.15 0 0 0-.15-.15zm.67 0a.15.15 0 0 0-.19.15v1.58a.15.15 0 0 0 .15.15H13a.15.15 0 0 0 .15-.15V9.5a.15.15 0 0 0-.15-.15zM6.28 7.09H8a.16.16 0 0 1 .16.16v1.56A.16.16 0 0 1 8 9H6.28a.15.15 0 0 1-.15-.15V7.24a.15.15 0 0 1 .15-.15zm2.47 0h1.77a.15.15 0 0 1 .15.15v1.57a.16.16 0 0 1-.16.16H8.75a.15.15 0 0 1-.15-.15V7.24a.15.15 0 0 1 .15-.15zm2.44 0H13a.15.15 0 0 1 .15.15v1.57A.15.15 0 0 1 13 9h-1.81a.16.16 0 0 1-.19-.19V7.24a.15.15 0 0 1 .19-.15z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "11.04" y = "4.82" width =
        "2.07" height = "1.88" rx = ".15" />< path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M13.65 9.35a.15.15 0 0 0-.15.15v1.58a.15.15 0 0 0 .15.15h1.77a.15.15 0 0 0 .15-.15V9.5a.15.15 0 0 0-.15-.15z"
        /> < title > { title } < / title > < / svg >
    }
}
